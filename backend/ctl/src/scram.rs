//! The SCRAM-SHA-256 verifier PostgreSQL stores for a password (RFC 5802, RFC 7677).
//!
//! `akr-ctl db provision` sends the verifier, never the password: the clear text then appears in no statement,
//! no server log and no replication stream.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

/// PostgreSQL's own iteration count for `scram-sha-256`.
pub const ITERATIONS: u32 = 4096;
const SALT_LEN: usize = 16;
const KEY_LEN: usize = 32;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ScramError {
    #[error("the password is empty")]
    Empty,
    /// SCRAM normalises a password with SASLprep before hashing it. Printable ASCII is the range SASLprep
    /// leaves alone, so a password in it hashes the same here and in every client.
    #[error("the password holds a character outside printable ASCII")]
    NotPrintableAscii,
    #[error("no randomness: {0}")]
    Random(String),
}

/// The verifier of `password` under a fresh salt.
pub fn verifier(password: &str) -> Result<String, ScramError> {
    let mut salt = [0u8; SALT_LEN];
    getrandom::fill(&mut salt).map_err(|e| ScramError::Random(e.to_string()))?;
    verifier_with(password, &salt, ITERATIONS)
}

/// `SCRAM-SHA-256$<iterations>:<salt>$<StoredKey>:<ServerKey>`, the form of `pg_authid.rolpassword`.
pub fn verifier_with(password: &str, salt: &[u8], iterations: u32) -> Result<String, ScramError> {
    if password.is_empty() {
        return Err(ScramError::Empty);
    }
    if !password.bytes().all(|b| (0x20..0x7f).contains(&b)) {
        return Err(ScramError::NotPrintableAscii);
    }
    let salted = salted_password(password.as_bytes(), salt, iterations);
    let client_key = hmac(salted.as_slice(), b"Client Key");
    let stored_key: [u8; KEY_LEN] = Sha256::digest(client_key.as_slice()).into();
    let server_key = hmac(salted.as_slice(), b"Server Key");
    Ok(format!(
        "SCRAM-SHA-256${iterations}:{}${}:{}",
        BASE64.encode(salt),
        BASE64.encode(stored_key),
        BASE64.encode(server_key.as_slice())
    ))
}

fn hmac(key: &[u8], message: &[u8]) -> Zeroizing<[u8; KEY_LEN]> {
    let mut mac = <Hmac<Sha256> as KeyInit>::new_from_slice(key)
        .unwrap_or_else(|_| unreachable!("HMAC takes a key of any length"));
    mac.update(message);
    Zeroizing::new(mac.finalize().into_bytes().into())
}

/// PBKDF2-HMAC-SHA-256 with one output block, which is all SCRAM asks for (`Hi` of RFC 5802).
fn salted_password(password: &[u8], salt: &[u8], iterations: u32) -> Zeroizing<[u8; KEY_LEN]> {
    let mut first = Vec::with_capacity(salt.len() + 4);
    first.extend_from_slice(salt);
    first.extend_from_slice(&1u32.to_be_bytes());

    let mut round = hmac(password, &first);
    let mut out = Zeroizing::new(*round);
    for _ in 1..iterations {
        round = hmac(password, round.as_slice());
        for (acc, byte) in out.iter_mut().zip(round.iter()) {
            *acc ^= byte;
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn the_key_derivation_matches_the_published_pbkdf2_vectors() {
        let derive =
            |iterations| hex::encode(salted_password(b"password", b"salt", iterations).as_slice());
        assert_eq!(
            derive(1),
            "120fb6cffcf8b32c43e7225256c4f837a86548c92ccc35480805987cb70be17b"
        );
        assert_eq!(
            derive(2),
            "ae4d0c95af6b46d32d0adff928f06dd02a303f8ef3c251dfd6e2d85a95474c43"
        );
        assert_eq!(
            derive(4096),
            "c5e478d59288c841aa530db6845c4c8d962893a001ce4e11a4963873aa98134a"
        );
    }

    /// The exchange printed in RFC 7677 section 3: user `user`, password `pencil`.
    #[test]
    fn the_verifier_holds_the_keys_of_the_rfc_7677_exchange() {
        const AUTH_MESSAGE: &str = "n=user,r=rOprNGfwEbeRWgbNEkqO,\
            r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096,\
            c=biws,r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0";
        let salt = BASE64.decode("W22ZaJ0SNY7soEsUEjb6gQ==").unwrap();

        let verifier = verifier_with("pencil", &salt, 4096).unwrap();
        let (head, keys) = verifier.rsplit_once('$').unwrap();
        assert_eq!(head, "SCRAM-SHA-256$4096:W22ZaJ0SNY7soEsUEjb6gQ==");
        let (stored_key, server_key) = keys.split_once(':').unwrap();
        let stored_key = BASE64.decode(stored_key).unwrap();
        let server_key = BASE64.decode(server_key).unwrap();

        let server_signature = hmac(&server_key, AUTH_MESSAGE.as_bytes());
        assert_eq!(
            BASE64.encode(server_signature.as_slice()),
            "6rriTRBi23WpRR/wtup+mMhUZUn/dB5nLTJRsjl95G4="
        );

        let proof = BASE64
            .decode("dHzbZapWIk4jUhN+Ute9ytag9zjfMHgsqmmiz7AndVQ=")
            .unwrap();
        let client_signature = hmac(&stored_key, AUTH_MESSAGE.as_bytes());
        let client_key: Vec<u8> = proof
            .iter()
            .zip(client_signature.iter())
            .map(|(p, s)| p ^ s)
            .collect();
        assert_eq!(
            Sha256::digest(&client_key).as_slice(),
            stored_key.as_slice()
        );
    }

    #[test]
    fn a_fresh_salt_gives_another_verifier_for_the_same_password() {
        let first = verifier("correct-Horse-7").unwrap();
        let second = verifier("correct-Horse-7").unwrap();
        assert_ne!(first, second);
        assert!(first.starts_with("SCRAM-SHA-256$4096:"));
        assert!(!first.contains("correct-Horse-7"));
    }

    #[test]
    fn an_empty_password_and_one_outside_printable_ascii_are_refused() {
        assert_eq!(verifier(""), Err(ScramError::Empty));
        assert_eq!(verifier("كلمة"), Err(ScramError::NotPrintableAscii));
        assert_eq!(verifier("tab\there"), Err(ScramError::NotPrintableAscii));
    }
}
