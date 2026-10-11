//! Detached Ed25519 signatures made by OpenBao transit keys.

use std::fmt;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;

use crate::error::CryptoError;

pub const SIGNATURE_LEN: usize = 64;

/// The transit signing keys (phase 00, 7.6). `akr-sign-decision` is reserved and unused (DR-294).
pub mod signing_key {
    /// Audit anchors (phase 02).
    pub const ANCHOR: &str = "akr-sign-anchor";
    pub const PACKAGE: &str = "akr-sign-package";
    /// Coverage certificates (phases 05 and 12).
    pub const CERTIFICATE: &str = "akr-sign-certificate";
    /// Deletion certificates and batch manifests (phase 23).
    pub const DELETION: &str = "akr-sign-deletion";
    /// Support bundles (phase 25): signed by `akr-ctl` under an operator policy, never by a service.
    pub const BUNDLE: &str = "akr-sign-bundle";

    pub const ALL: [&str; 5] = [ANCHOR, PACKAGE, CERTIFICATE, DELETION, BUNDLE];
}

/// A signature and the version of the key that made it. The version is part of the signature for storage:
/// without it nothing made before a key rotation could be verified.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Signature {
    pub key_version: u32,
    pub bytes: [u8; SIGNATURE_LEN],
}

impl Signature {
    /// `vault:v<n>:<base64>`, the form transit returns and verifies.
    pub fn to_transit(&self) -> String {
        format!("vault:v{}:{}", self.key_version, BASE64.encode(self.bytes))
    }

    pub fn from_transit(text: &str) -> Result<Signature, CryptoError> {
        let malformed =
            || CryptoError::Unavailable("a signature is not vault:v<n>:<base64>".into());
        let (version, b64) = text
            .strip_prefix("vault:v")
            .and_then(|rest| rest.split_once(':'))
            .ok_or_else(malformed)?;
        let key_version = version.parse::<u32>().map_err(|_| malformed())?;
        let bytes = BASE64
            .decode(b64)
            .ok()
            .and_then(|raw| <[u8; SIGNATURE_LEN]>::try_from(raw).ok())
            .ok_or_else(malformed)?;
        Ok(Signature { key_version, bytes })
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signature(v{})", self.key_version)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn the_transit_form_round_trips_with_its_key_version() {
        let sig = Signature {
            key_version: 3,
            bytes: [5; SIGNATURE_LEN],
        };
        let text = sig.to_transit();
        assert!(text.starts_with("vault:v3:"));
        assert_eq!(Signature::from_transit(&text).unwrap(), sig);
    }

    #[test]
    fn a_malformed_signature_is_refused() {
        for bad in [
            "",
            "vault:v1",
            "vault:vx:AAAA",
            "vault:v1:not base64",
            "vault:v1:AAAA",
            "v1:AAAA",
        ] {
            assert!(Signature::from_transit(bad).is_err(), "{bad}");
        }
    }
}
