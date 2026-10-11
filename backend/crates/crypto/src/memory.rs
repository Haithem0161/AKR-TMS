//! The crypto port without OpenBao: tests and the dev profile.
//!
//! Every key derives from one seed, so two processes built from the same seed (the dev `akr-api` and
//! `akr-worker`) read each other's sealed values and a restart loses nothing. The seed is not a secret in the
//! dev profile; `Config::validate` refuses this implementation in production.

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use ed25519_dalek::{Signer as _, SigningKey, Verifier as _};
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::Crypto;
use crate::error::CryptoError;
use crate::family::Family;
use crate::purpose::{Purpose, derive_key, hkdf_sha256, hmac_sha256};
use crate::sealed::{DataKey, KEY_LEN, Sealed};
use crate::signature::Signature;

pub struct InMemoryCrypto {
    seed: Zeroizing<[u8; KEY_LEN]>,
    /// The active generation of each family's key; `rotate` moves it on.
    generations: Mutex<HashMap<Family, u32>>,
}

impl InMemoryCrypto {
    pub fn from_seed(seed: [u8; KEY_LEN]) -> Self {
        Self {
            seed: Zeroizing::new(seed),
            generations: Mutex::new(HashMap::new()),
        }
    }

    /// Fresh random keys: what a test wants.
    pub fn random() -> Result<Self, CryptoError> {
        let mut seed = [0u8; KEY_LEN];
        getrandom::fill(&mut seed)
            .map_err(|e| CryptoError::Unavailable(format!("no randomness: {e}")))?;
        Ok(Self::from_seed(seed))
    }

    /// Starts a new data key for the family, as a new month does. Values sealed before stay readable.
    pub fn rotate(&self, family: Family) {
        let mut generations = self.lock();
        *generations.entry(family).or_insert(1) += 1;
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<Family, u32>> {
        self.generations
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn generation(&self, family: Family) -> u32 {
        self.lock().get(&family).copied().unwrap_or(1)
    }

    fn data_key(&self, family: Family, generation: u32) -> Result<DataKey, CryptoError> {
        let label = format!("dek/{family}/{generation}");
        let id = hkdf_sha256(self.seed.as_slice(), format!("id/{label}").as_bytes())?;
        let mut id_bytes = [0u8; 16];
        id_bytes.copy_from_slice(&id[..16]);
        Ok(DataKey {
            id: uuid::Builder::from_custom_bytes(id_bytes).into_uuid(),
            family,
            key_version: 1,
            bytes: hkdf_sha256(self.seed.as_slice(), label.as_bytes())?,
        })
    }

    fn data_key_by_id(&self, family: Family, id: Uuid) -> Result<DataKey, CryptoError> {
        for generation in (1..=self.generation(family)).rev() {
            let key = self.data_key(family, generation)?;
            if key.id == id {
                return Ok(key);
            }
        }
        Err(CryptoError::UnknownKey(format!("data key {id}")))
    }

    fn signing_key(&self, name: &str) -> Result<SigningKey, CryptoError> {
        let seed = hkdf_sha256(self.seed.as_slice(), format!("sign/{name}").as_bytes())?;
        Ok(SigningKey::from_bytes(&seed))
    }
}

#[async_trait]
impl Crypto for InMemoryCrypto {
    async fn seal(
        &self,
        family: Family,
        aad: &[u8],
        plaintext: &[u8],
    ) -> Result<Sealed, CryptoError> {
        self.data_key(family, self.generation(family))?
            .seal(aad, plaintext)
    }

    async fn open(
        &self,
        family: Family,
        aad: &[u8],
        sealed: &Sealed,
    ) -> Result<Vec<u8>, CryptoError> {
        if sealed.family != family {
            return Err(CryptoError::Rejected);
        }
        self.data_key_by_id(family, sealed.dek_id)?
            .open(aad, sealed)
    }

    async fn keyed_hash(&self, purpose: &Purpose, data: &[u8]) -> Result<[u8; 32], CryptoError> {
        let entry = hkdf_sha256(
            self.seed.as_slice(),
            format!("kv/{}", purpose.source()).as_bytes(),
        )?;
        let key = derive_key(entry.as_slice(), purpose)?;
        hmac_sha256(key.as_slice(), &[data])
    }

    async fn sign(&self, key: &str, msg: &[u8]) -> Result<Signature, CryptoError> {
        Ok(Signature {
            key_version: 1,
            bytes: self.signing_key(key)?.sign(msg).to_bytes(),
        })
    }

    async fn verify(&self, key: &str, msg: &[u8], sig: &Signature) -> Result<bool, CryptoError> {
        if sig.key_version != 1 {
            return Ok(false);
        }
        let signature = ed25519_dalek::Signature::from_bytes(&sig.bytes);
        Ok(self
            .signing_key(key)?
            .verifying_key()
            .verify(msg, &signature)
            .is_ok())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::signature::signing_key;

    const TEST: Purpose = Purpose::new("test.sample", "keys/test");
    const OTHER: Purpose = Purpose::new("test.other", "keys/test");

    #[tokio::test]
    async fn seal_then_open_returns_the_plaintext() {
        let c = InMemoryCrypto::random().unwrap();
        let sealed = c.seal(Family::Idempotency, b"aad", b"body").await.unwrap();
        assert_eq!(
            c.open(Family::Idempotency, b"aad", &sealed).await.unwrap(),
            b"body"
        );
    }

    #[tokio::test]
    async fn a_value_does_not_open_under_another_family_or_aad() {
        let c = InMemoryCrypto::random().unwrap();
        let sealed = c.seal(Family::Personal, b"row-1", b"x").await.unwrap();
        assert_eq!(
            c.open(Family::Financial, b"row-1", &sealed).await,
            Err(CryptoError::Rejected)
        );
        assert_eq!(
            c.open(Family::Personal, b"row-2", &sealed).await,
            Err(CryptoError::Rejected)
        );
    }

    #[tokio::test]
    async fn values_sealed_before_a_rotation_stay_readable() {
        let c = InMemoryCrypto::random().unwrap();
        let old = c.seal(Family::Personal, b"a", b"before").await.unwrap();
        c.rotate(Family::Personal);
        let new = c.seal(Family::Personal, b"a", b"after").await.unwrap();
        assert_ne!(old.dek_id, new.dek_id);
        assert_eq!(
            c.open(Family::Personal, b"a", &old).await.unwrap(),
            b"before"
        );
        assert_eq!(
            c.open(Family::Personal, b"a", &new).await.unwrap(),
            b"after"
        );
    }

    #[tokio::test]
    async fn two_instances_of_one_seed_read_each_other() {
        let api = InMemoryCrypto::from_seed([4; KEY_LEN]);
        let worker = InMemoryCrypto::from_seed([4; KEY_LEN]);
        let sealed = api.seal(Family::Evidence, b"id", b"shared").await.unwrap();
        assert_eq!(
            worker.open(Family::Evidence, b"id", &sealed).await.unwrap(),
            b"shared"
        );

        let stranger = InMemoryCrypto::from_seed([5; KEY_LEN]);
        assert!(
            stranger
                .open(Family::Evidence, b"id", &sealed)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn a_keyed_hash_is_stable_and_bound_to_its_purpose() {
        let c = InMemoryCrypto::random().unwrap();
        let a = c.keyed_hash(&TEST, b"value").await.unwrap();
        assert_eq!(a, c.keyed_hash(&TEST, b"value").await.unwrap());
        assert_ne!(a, c.keyed_hash(&OTHER, b"value").await.unwrap());
        assert_ne!(a, c.keyed_hash(&TEST, b"other value").await.unwrap());
    }

    #[tokio::test]
    async fn a_signature_is_bound_to_the_message_and_the_key_name() {
        let c = InMemoryCrypto::random().unwrap();
        let sig = c.sign(signing_key::ANCHOR, b"root").await.unwrap();
        assert!(c.verify(signing_key::ANCHOR, b"root", &sig).await.unwrap());
        assert!(
            !c.verify(signing_key::ANCHOR, b"another root", &sig)
                .await
                .unwrap()
        );
        assert!(!c.verify(signing_key::PACKAGE, b"root", &sig).await.unwrap());

        let mut forged = sig;
        forged.bytes[0] ^= 1;
        assert!(
            !c.verify(signing_key::ANCHOR, b"root", &forged)
                .await
                .unwrap()
        );
    }
}
