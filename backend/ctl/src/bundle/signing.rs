//! The vendor's Ed25519 key: the public half the bank pins by fingerprint, and a private half read from a
//! file for development and tests. A release is signed by cosign with the key in the vendor's KMS; the
//! signature it writes for a blob is the one read here.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use ed25519_dalek::{Signature, Signer as _, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

use crate::error::CtlError;

const KEY_LEN: usize = 32;
/// The DER of `SubjectPublicKeyInfo { algorithm: id-Ed25519, subjectPublicKey: BIT STRING }` before the key.
const SPKI_PREFIX: [u8; 12] = [
    0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
];
/// The DER of a PKCS#8 `PrivateKeyInfo` for Ed25519 before the seed.
const PKCS8_PREFIX: [u8; 16] = [
    0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22, 0x04, 0x20,
];
const PUBLIC_LABEL: &str = "PUBLIC KEY";
const PRIVATE_LABEL: &str = "PRIVATE KEY";

fn pem_body(pem: &str, label: &str) -> Option<Vec<u8>> {
    let begin = format!("-----BEGIN {label}-----");
    let end = format!("-----END {label}-----");
    let start = pem.find(&begin)? + begin.len();
    let stop = pem[start..].find(&end)? + start;
    let body: String = pem[start..stop].split_whitespace().collect();
    BASE64.decode(body).ok()
}

fn pem(label: &str, der: &[u8]) -> String {
    let body = BASE64.encode(der);
    let mut out = format!("-----BEGIN {label}-----\n");
    for line in body.as_bytes().chunks(64) {
        out.push_str(&String::from_utf8_lossy(line));
        out.push('\n');
    }
    out.push_str(&format!("-----END {label}-----\n"));
    out
}

/// The vendor's public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorKey {
    key: VerifyingKey,
}

impl VendorKey {
    /// Reads a PEM `PUBLIC KEY` that holds an Ed25519 key.
    pub fn from_pem(text: &str) -> Result<Self, CtlError> {
        let not_a_key = || CtlError::usage("the public key is not a PEM Ed25519 public key");
        let der = pem_body(text, PUBLIC_LABEL).ok_or_else(not_a_key)?;
        let raw = der.strip_prefix(&SPKI_PREFIX[..]).ok_or_else(not_a_key)?;
        let raw: [u8; KEY_LEN] = raw.try_into().map_err(|_| not_a_key())?;
        Ok(Self {
            key: VerifyingKey::from_bytes(&raw).map_err(|_| not_a_key())?,
        })
    }

    fn der(&self) -> Vec<u8> {
        [&SPKI_PREFIX[..], self.key.as_bytes()].concat()
    }

    pub fn to_pem(&self) -> String {
        pem(PUBLIC_LABEL, &self.der())
    }

    /// The SHA-256 of the DER form, in lowercase hex: what the bank receives on a separate channel.
    pub fn fingerprint(&self) -> String {
        hex::encode(Sha256::digest(self.der()))
    }

    /// `signature` is base64, as cosign writes it. Anything that is not a valid signature is `false`.
    pub fn verifies(&self, message: &[u8], signature: &str) -> bool {
        let Ok(bytes) = BASE64.decode(signature.trim()) else {
            return false;
        };
        let Ok(signature) = Signature::from_slice(&bytes) else {
            return false;
        };
        self.key.verify_strict(message, &signature).is_ok()
    }
}

/// Compares fingerprints as people type them: case and `:` separators do not matter.
pub fn same_fingerprint(given: &str, computed: &str) -> bool {
    let normal = |text: &str| -> String {
        text.trim()
            .chars()
            .filter(|c| *c != ':')
            .map(|c| c.to_ascii_lowercase())
            .collect()
    };
    let given = normal(given);
    given.len() == 64 && given == normal(computed)
}

/// A private key in a file. Never the release key: that one does not leave the vendor's KMS.
pub struct DevSigner {
    key: SigningKey,
}

impl DevSigner {
    /// Reads a PEM `PRIVATE KEY` (PKCS#8) that holds an Ed25519 key, as `openssl genpkey` writes it.
    pub fn from_pem(text: &str) -> Result<Self, CtlError> {
        let not_a_key =
            || CtlError::usage("the signing key is not a PEM PKCS#8 Ed25519 private key");
        let der = Zeroizing::new(pem_body(text, PRIVATE_LABEL).ok_or_else(not_a_key)?);
        let seed = der.strip_prefix(&PKCS8_PREFIX[..]).ok_or_else(not_a_key)?;
        let seed: Zeroizing<[u8; KEY_LEN]> =
            Zeroizing::new(seed.try_into().map_err(|_| not_a_key())?);
        Ok(Self::from_seed(&seed))
    }

    pub fn from_seed(seed: &[u8; KEY_LEN]) -> Self {
        Self {
            key: SigningKey::from_bytes(seed),
        }
    }

    pub fn vendor_key(&self) -> VendorKey {
        VendorKey {
            key: self.key.verifying_key(),
        }
    }

    /// The signature in base64.
    pub fn sign(&self, message: &[u8]) -> String {
        BASE64.encode(self.key.sign(message).to_bytes())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const SEED: [u8; 32] = [7; 32];

    #[test]
    fn a_public_key_survives_its_pem_form_and_has_a_stable_fingerprint() {
        let key = DevSigner::from_seed(&SEED).vendor_key();
        let pem = key.to_pem();
        assert!(pem.starts_with("-----BEGIN PUBLIC KEY-----\n"));
        let back = VendorKey::from_pem(&pem).unwrap();
        assert_eq!(back, key);
        assert_eq!(back.fingerprint(), key.fingerprint());
        assert_eq!(key.fingerprint().len(), 64);

        let other = DevSigner::from_seed(&[8; 32]).vendor_key();
        assert_ne!(other.fingerprint(), key.fingerprint());
    }

    #[test]
    fn a_signature_verifies_only_for_the_signed_bytes_and_the_signing_key() {
        let signer = DevSigner::from_seed(&SEED);
        let key = signer.vendor_key();
        let signature = signer.sign(b"manifest");

        assert!(key.verifies(b"manifest", &signature));
        assert!(key.verifies(b"manifest", &format!("{signature}\n")));
        assert!(!key.verifies(b"manifesu", &signature));
        assert!(
            !DevSigner::from_seed(&[8; 32])
                .vendor_key()
                .verifies(b"manifest", &signature)
        );
        assert!(!key.verifies(b"manifest", "not base64"));
        assert!(!key.verifies(b"manifest", &BASE64.encode([0u8; 10])));
    }

    #[test]
    fn a_pkcs8_private_key_is_read_and_anything_else_is_not_a_key() {
        let der = [&PKCS8_PREFIX[..], &SEED[..]].concat();
        let signer = DevSigner::from_pem(&pem(PRIVATE_LABEL, &der)).unwrap();
        assert_eq!(
            signer.vendor_key(),
            DevSigner::from_seed(&SEED).vendor_key()
        );

        assert!(
            DevSigner::from_pem("-----BEGIN PRIVATE KEY-----\nAAAA\n-----END PRIVATE KEY-----")
                .is_err()
        );
        assert!(VendorKey::from_pem("no key here").is_err());
        let rsa_like = pem(PUBLIC_LABEL, &[0x30, 0x82, 0x01, 0x22]);
        assert!(VendorKey::from_pem(&rsa_like).is_err());
    }

    #[test]
    fn a_fingerprint_is_compared_as_people_type_it() {
        let computed = "ab".repeat(32);
        assert!(same_fingerprint(&"AB".repeat(32), &computed));
        assert!(same_fingerprint(
            &format!(" {} ", ["ab"; 32].join(":")),
            &computed
        ));
        assert!(!same_fingerprint(&"ab".repeat(31), &computed));
        assert!(!same_fingerprint(&"ac".repeat(32), &computed));
        assert!(!same_fingerprint("", ""));
    }
}
