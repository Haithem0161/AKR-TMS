//! All cryptography behind one port (SP-13): sealed values, keyed hashes and signatures.
//!
//! [`OpenBaoCrypto`] is the implementation a bank runs; [`InMemoryCrypto`] serves tests and the dev profile.
//! Keys are unwrapped once and kept in process memory: OpenBao is never called per request.

use async_trait::async_trait;

pub mod bao;
pub mod error;
pub mod family;
pub mod memory;
pub mod openbao;
pub mod purpose;
pub mod sealed;
pub mod signature;

pub use bao::BaoClient;
pub use error::CryptoError;
pub use family::Family;
pub use memory::InMemoryCrypto;
pub use openbao::{DataKeyRow, DataKeyStore, OpenBaoCrypto};
pub use purpose::Purpose;
pub use sealed::Sealed;
pub use signature::{Signature, signing_key};

#[async_trait]
pub trait Crypto: Send + Sync {
    /// Seals `plaintext` under the family's data key of this month. `aad` binds the value to its place (the
    /// row id, or the principal and key of an idempotency entry): a value moved elsewhere does not open.
    async fn seal(
        &self,
        family: Family,
        aad: &[u8],
        plaintext: &[u8],
    ) -> Result<Sealed, CryptoError>;

    async fn open(
        &self,
        family: Family,
        aad: &[u8],
        sealed: &Sealed,
    ) -> Result<Vec<u8>, CryptoError>;

    /// HMAC-SHA-256 of `data` under the key of `purpose`.
    async fn keyed_hash(&self, purpose: &Purpose, data: &[u8]) -> Result<[u8; 32], CryptoError>;

    /// Signs `msg` with the newest version of the named signing key ([`signing_key`]).
    async fn sign(&self, key: &str, msg: &[u8]) -> Result<Signature, CryptoError>;

    async fn verify(&self, key: &str, msg: &[u8], sig: &Signature) -> Result<bool, CryptoError>;
}
