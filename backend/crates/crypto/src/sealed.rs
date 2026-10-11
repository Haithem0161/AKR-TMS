//! Sealed values: AES-256-GCM under a data key, with the key named inside the value.
//!
//! Stored form (a `BYTEA` column): `0x01 | family code | data key id (16) | key version (u32, big-endian) |
//! nonce (12) | ciphertext and tag`. The 22 bytes before the nonce are authenticated with the caller's AAD, so a
//! value cannot be re-labelled to another family or key.

use std::fmt;

use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::error::CryptoError;
use crate::family::Family;

pub const KEY_LEN: usize = 32;
pub const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;
const FORMAT_V1: u8 = 1;
const HEADER_LEN: usize = 1 + 1 + 16 + 4;

/// A sealed value. It has no `Serialize`: it reaches a response only through an explicit view of its opened
/// content.
#[derive(Clone, PartialEq, Eq)]
pub struct Sealed {
    pub family: Family,
    /// The data key it was sealed under (`admin.data_keys.id`).
    pub dek_id: Uuid,
    /// The version of the KEK that wrapped the data key when the value was sealed.
    pub key_version: u32,
    pub nonce: [u8; NONCE_LEN],
    /// Ciphertext followed by the 16-byte tag.
    pub ciphertext: Vec<u8>,
}

impl Sealed {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_LEN + NONCE_LEN + self.ciphertext.len());
        out.extend_from_slice(&header(self.family, self.dek_id, self.key_version));
        out.extend_from_slice(&self.nonce);
        out.extend_from_slice(&self.ciphertext);
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Sealed, CryptoError> {
        if bytes.len() < HEADER_LEN + NONCE_LEN + TAG_LEN || bytes[0] != FORMAT_V1 {
            return Err(CryptoError::Rejected);
        }
        let family = Family::from_code(bytes[1]).ok_or(CryptoError::Rejected)?;
        let dek_id = Uuid::from_slice(&bytes[2..18]).map_err(|_| CryptoError::Rejected)?;
        let key_version = u32::from_be_bytes(
            bytes[18..HEADER_LEN]
                .try_into()
                .map_err(|_| CryptoError::Rejected)?,
        );
        let nonce = bytes[HEADER_LEN..HEADER_LEN + NONCE_LEN]
            .try_into()
            .map_err(|_| CryptoError::Rejected)?;
        Ok(Sealed {
            family,
            dek_id,
            key_version,
            nonce,
            ciphertext: bytes[HEADER_LEN + NONCE_LEN..].to_vec(),
        })
    }
}

impl fmt::Debug for Sealed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sealed")
            .field("family", &self.family)
            .field("dek_id", &self.dek_id)
            .field("key_version", &self.key_version)
            .field("len", &self.ciphertext.len())
            .finish()
    }
}

/// An unwrapped data key. It lives in process memory only and is wiped when dropped.
pub(crate) struct DataKey {
    pub id: Uuid,
    pub family: Family,
    pub key_version: u32,
    pub bytes: Zeroizing<[u8; KEY_LEN]>,
}

impl DataKey {
    pub fn seal(&self, aad: &[u8], plaintext: &[u8]) -> Result<Sealed, CryptoError> {
        let mut nonce = [0u8; NONCE_LEN];
        getrandom::fill(&mut nonce)
            .map_err(|e| CryptoError::Unavailable(format!("no randomness: {e}")))?;
        let ciphertext = self
            .cipher()
            .encrypt(
                &Nonce::from(nonce),
                Payload {
                    msg: plaintext,
                    aad: &full_aad(self.family, self.id, self.key_version, aad),
                },
            )
            .map_err(|_| CryptoError::Rejected)?;
        Ok(Sealed {
            family: self.family,
            dek_id: self.id,
            key_version: self.key_version,
            nonce,
            ciphertext,
        })
    }

    pub fn open(&self, aad: &[u8], sealed: &Sealed) -> Result<Vec<u8>, CryptoError> {
        if sealed.dek_id != self.id || sealed.family != self.family {
            return Err(CryptoError::Rejected);
        }
        self.cipher()
            .decrypt(
                &Nonce::from(sealed.nonce),
                Payload {
                    msg: &sealed.ciphertext,
                    aad: &full_aad(sealed.family, sealed.dek_id, sealed.key_version, aad),
                },
            )
            .map_err(|_| CryptoError::Rejected)
    }

    fn cipher(&self) -> Aes256Gcm {
        Aes256Gcm::new(&Key::<Aes256Gcm>::from(*self.bytes))
    }
}

fn header(family: Family, dek_id: Uuid, key_version: u32) -> [u8; HEADER_LEN] {
    let mut h = [0u8; HEADER_LEN];
    h[0] = FORMAT_V1;
    h[1] = family.code();
    h[2..18].copy_from_slice(dek_id.as_bytes());
    h[18..].copy_from_slice(&key_version.to_be_bytes());
    h
}

fn full_aad(family: Family, dek_id: Uuid, key_version: u32, aad: &[u8]) -> Vec<u8> {
    let mut full = Vec::with_capacity(HEADER_LEN + aad.len());
    full.extend_from_slice(&header(family, dek_id, key_version));
    full.extend_from_slice(aad);
    full
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn key(family: Family, byte: u8) -> DataKey {
        DataKey {
            id: Uuid::from_bytes([byte; 16]),
            family,
            key_version: 1,
            bytes: Zeroizing::new([byte; KEY_LEN]),
        }
    }

    #[test]
    fn a_sealed_value_opens_under_its_key_and_aad() {
        let k = key(Family::Idempotency, 7);
        let sealed = k.seal(b"principal|key", b"{\"a\":1}").unwrap();
        assert_eq!(k.open(b"principal|key", &sealed).unwrap(), b"{\"a\":1}");
    }

    #[test]
    fn the_ciphertext_does_not_hold_the_plaintext() {
        let k = key(Family::Idempotency, 7);
        let plain = b"a response body that must not be readable at rest";
        let stored = k.seal(b"aad", plain).unwrap().to_bytes();
        assert!(!stored.windows(plain.len()).any(|w| w == plain));
    }

    #[test]
    fn two_seals_of_one_value_differ() {
        let k = key(Family::Personal, 7);
        assert_ne!(
            k.seal(b"a", b"same").unwrap(),
            k.seal(b"a", b"same").unwrap()
        );
    }

    #[test]
    fn another_aad_another_key_or_a_changed_byte_is_rejected() {
        let k = key(Family::Personal, 7);
        let sealed = k.seal(b"row-1", b"secret").unwrap();
        assert_eq!(k.open(b"row-2", &sealed), Err(CryptoError::Rejected));

        let other = DataKey {
            bytes: Zeroizing::new([8; KEY_LEN]),
            ..key(Family::Personal, 7)
        };
        assert_eq!(other.open(b"row-1", &sealed), Err(CryptoError::Rejected));

        let mut bytes = sealed.to_bytes();
        let last = bytes.len() - 1;
        bytes[last] ^= 1;
        let tampered = Sealed::from_bytes(&bytes).unwrap();
        assert_eq!(k.open(b"row-1", &tampered), Err(CryptoError::Rejected));
    }

    #[test]
    fn a_value_relabelled_to_another_family_or_version_is_rejected() {
        let k = key(Family::Personal, 7);
        let sealed = k.seal(b"row", b"secret").unwrap();

        let relabelled = Sealed {
            key_version: 2,
            ..sealed.clone()
        };
        assert_eq!(k.open(b"row", &relabelled), Err(CryptoError::Rejected));

        let financial = DataKey {
            family: Family::Financial,
            ..key(Family::Personal, 7)
        };
        let moved = Sealed {
            family: Family::Financial,
            ..sealed
        };
        assert_eq!(financial.open(b"row", &moved), Err(CryptoError::Rejected));
    }

    #[test]
    fn the_stored_form_round_trips_and_refuses_garbage() {
        let k = key(Family::Card, 3);
        let sealed = k.seal(b"aad", b"").unwrap();
        assert_eq!(Sealed::from_bytes(&sealed.to_bytes()).unwrap(), sealed);

        assert_eq!(Sealed::from_bytes(b""), Err(CryptoError::Rejected));
        assert_eq!(Sealed::from_bytes(&[1u8; 20]), Err(CryptoError::Rejected));
        let mut wrong_version = sealed.to_bytes();
        wrong_version[0] = 9;
        assert_eq!(
            Sealed::from_bytes(&wrong_version),
            Err(CryptoError::Rejected)
        );
        let mut wrong_family = sealed.to_bytes();
        wrong_family[1] = 0;
        assert_eq!(
            Sealed::from_bytes(&wrong_family),
            Err(CryptoError::Rejected)
        );
    }

    #[test]
    fn debug_output_shows_no_bytes() {
        let k = key(Family::Personal, 7);
        let sealed = k.seal(b"row", b"secret").unwrap();
        let shown = format!("{sealed:?}");
        assert!(shown.contains("len"));
        assert!(!shown.contains("nonce"));
        assert!(!shown.contains("ciphertext"));
    }
}
