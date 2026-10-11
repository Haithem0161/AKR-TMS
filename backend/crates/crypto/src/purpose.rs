//! Keyed-hash purposes (phase 00, 7.15) and the HMAC arithmetic behind them.
//!
//! A purpose names what a keyed hash is for and which OpenBao KV entry its key derives from. The key of a
//! purpose is `HKDF-SHA-256(entry key, info = purpose name)`, so two purposes over one entry never produce
//! comparable hashes.

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::error::CryptoError;

type HmacSha256 = Hmac<Sha256>;

pub const HASH_LEN: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Purpose {
    name: &'static str,
    source: &'static str,
}

impl Purpose {
    /// Every declared purpose. The phase that uses a purpose adds its constant to this type and to this
    /// list; a test asserts that the names are unique.
    pub const ALL: &'static [Purpose] = &[];

    #[allow(dead_code)]
    pub(crate) const fn new(name: &'static str, source: &'static str) -> Self {
        Self { name, source }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The KV entry (relative to the `akr/` mount) whose key this purpose derives from.
    pub fn source(&self) -> &'static str {
        self.source
    }
}

/// The key of one purpose, derived from the key of its KV entry.
pub(crate) fn derive_key(
    entry_key: &[u8],
    purpose: &Purpose,
) -> Result<Zeroizing<[u8; HASH_LEN]>, CryptoError> {
    hkdf_sha256(entry_key, purpose.name().as_bytes())
}

/// HKDF-SHA-256 (RFC 5869) with an empty salt and one output block.
pub(crate) fn hkdf_sha256(
    ikm: &[u8],
    info: &[u8],
) -> Result<Zeroizing<[u8; HASH_LEN]>, CryptoError> {
    let prk = Zeroizing::new(hmac_sha256(&[0u8; HASH_LEN], &[ikm])?);
    Ok(Zeroizing::new(hmac_sha256(
        prk.as_slice(),
        &[info, &[1u8]],
    )?))
}

pub fn hmac_sha256(key: &[u8], parts: &[&[u8]]) -> Result<[u8; HASH_LEN], CryptoError> {
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|_| CryptoError::Unavailable("the hash key has an unusable length".into()))?;
    for part in parts {
        mac.update(part);
    }
    Ok(mac.finalize().into_bytes().into())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn purpose_names_are_unique() {
        let names: HashSet<_> = Purpose::ALL.iter().map(Purpose::name).collect();
        assert_eq!(names.len(), Purpose::ALL.len());
    }

    #[test]
    fn hkdf_matches_rfc_5869_test_case_3() {
        // Test case 3: SHA-256, zero-length salt and info. With a zero-length salt the PRK is keyed by a
        // string of HashLen zeros, which is what `hkdf_sha256` does.
        let ikm = [0x0b_u8; 22];
        let okm = hkdf_sha256(&ikm, b"").unwrap();
        assert_eq!(
            hex::encode(okm.as_slice()),
            "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d"
        );
    }

    #[test]
    fn hmac_matches_rfc_4231_test_case_2() {
        let tag = hmac_sha256(b"Jefe", &[b"what do ya want ", b"for nothing?"]).unwrap();
        assert_eq!(
            hex::encode(tag),
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
    }

    #[test]
    fn two_purposes_over_one_entry_get_different_keys() {
        let entry = [9u8; 32];
        let a = derive_key(&entry, &Purpose::new("test.a", "keys/test")).unwrap();
        let b = derive_key(&entry, &Purpose::new("test.b", "keys/test")).unwrap();
        assert_ne!(a.as_slice(), b.as_slice());
    }
}
