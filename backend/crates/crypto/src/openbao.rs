//! The crypto port over OpenBao (TS-24, SP-11).
//!
//! Key hierarchy: the transit key `akr-kek` wraps data keys; a data key is made once per family and month,
//! kept wrapped in `admin.data_keys`, unwrapped once per process and held in memory. Sealing and opening are
//! local AES-256-GCM; only a key this process has not met costs an OpenBao call.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use chrono::{DateTime, Datelike, Utc};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::Crypto;
use crate::bao::{BaoClient, KEK};
use crate::error::CryptoError;
use crate::family::Family;
use crate::purpose::{HASH_LEN, Purpose, derive_key, hmac_sha256};
use crate::sealed::{DataKey, KEY_LEN, Sealed};
use crate::signature::Signature;

/// A row of `admin.data_keys`: a data key in its wrapped form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataKeyRow {
    pub id: Uuid,
    pub family: Family,
    /// The UTC month the key seals in, `YYYY-MM`.
    pub period: String,
    /// Transit ciphertext of the key, `vault:v<n>:<base64>`.
    pub wrapped: String,
}

/// Where wrapped data keys are kept. The PostgreSQL adapter is `akr_db::PgDataKeyStore`.
#[async_trait]
pub trait DataKeyStore: Send + Sync {
    async fn find(&self, id: Uuid) -> Result<Option<DataKeyRow>, CryptoError>;

    async fn find_period(
        &self,
        family: Family,
        period: &str,
    ) -> Result<Option<DataKeyRow>, CryptoError>;

    /// Inserts `row` unless its family and period already have a key, and returns the row that is there
    /// afterwards: the caller's, or the one another node wrote first.
    async fn insert_if_absent(&self, row: DataKeyRow) -> Result<DataKeyRow, CryptoError>;
}

/// The keys of the keyed-hash purposes a binary uses, read from KV once at start.
#[derive(Default)]
pub struct HashKeys {
    by_purpose: HashMap<&'static str, Zeroizing<[u8; HASH_LEN]>>,
}

impl HashKeys {
    /// Reads the KV entry of every purpose in `purposes` (field `key`, base64, at least 32 bytes) and derives
    /// the purpose keys. It runs before the login token is narrowed, while KV is still readable.
    pub async fn load(bao: &BaoClient, purposes: &[Purpose]) -> Result<Self, CryptoError> {
        let mut entries: HashMap<&'static str, Zeroizing<Vec<u8>>> = HashMap::new();
        let mut by_purpose = HashMap::new();
        for purpose in purposes {
            if !entries.contains_key(purpose.source()) {
                let item = bao.kv_read(purpose.source()).await?;
                entries.insert(purpose.source(), entry_key(purpose.source(), &item)?);
            }
            if let Some(entry) = entries.get(purpose.source()) {
                by_purpose.insert(purpose.name(), derive_key(entry, purpose)?);
            }
        }
        Ok(Self { by_purpose })
    }
}

/// The key bytes of a `keys/<name>` KV item.
pub fn entry_key(path: &str, item: &serde_json::Value) -> Result<Zeroizing<Vec<u8>>, CryptoError> {
    let key = item
        .get("key")
        .and_then(serde_json::Value::as_str)
        .and_then(|b64| BASE64.decode(b64).ok())
        .map(Zeroizing::new)
        .ok_or_else(|| CryptoError::Unavailable(format!("KV item {path} has no base64 `key`")))?;
    if key.len() < KEY_LEN {
        return Err(CryptoError::Unavailable(format!(
            "the key of KV item {path} is shorter than 256 bits"
        )));
    }
    Ok(key)
}

#[derive(Default)]
struct KeyCache {
    by_id: HashMap<Uuid, Arc<DataKey>>,
    active: HashMap<(Family, String), Uuid>,
}

pub struct OpenBaoCrypto {
    bao: BaoClient,
    store: Arc<dyn DataKeyStore>,
    hash_keys: HashKeys,
    cache: RwLock<KeyCache>,
    /// Serialises the slow path, so one process asks OpenBao once for a key it has not met.
    loading: Mutex<()>,
}

impl OpenBaoCrypto {
    pub fn new(bao: BaoClient, store: Arc<dyn DataKeyStore>, hash_keys: HashKeys) -> Self {
        Self {
            bao,
            store,
            hash_keys,
            cache: RwLock::new(KeyCache::default()),
            loading: Mutex::new(()),
        }
    }

    /// Makes sure every family has its data key for the month of `at` and for the month after it, so a
    /// process that may only unwrap keys (`akr-api`) always finds the key of a new month waiting.
    pub async fn ensure_keys(&self, at: DateTime<Utc>) -> Result<(), CryptoError> {
        for family in Family::ALL {
            for period in [period_of(at), period_after(at)] {
                self.active_key(family, &period).await?;
            }
        }
        Ok(())
    }

    async fn active_key(&self, family: Family, period: &str) -> Result<Arc<DataKey>, CryptoError> {
        let slot = (family, period.to_owned());
        if let Some(key) = self.cached_active(&slot).await {
            return Ok(key);
        }
        let _loading = self.loading.lock().await;
        if let Some(key) = self.cached_active(&slot).await {
            return Ok(key);
        }
        let row = match self.store.find_period(family, period).await? {
            Some(row) => row,
            None => {
                let wrapped = self.bao.datakey_wrapped(KEK).await?;
                self.store
                    .insert_if_absent(DataKeyRow {
                        id: Uuid::now_v7(),
                        family,
                        period: period.to_owned(),
                        wrapped: wrapped.ciphertext,
                    })
                    .await?
            }
        };
        let key = Arc::new(self.unwrap(&row).await?);
        let mut cache = self.cache.write().await;
        cache.by_id.insert(key.id, Arc::clone(&key));
        cache.active.insert(slot, key.id);
        Ok(key)
    }

    async fn cached_active(&self, slot: &(Family, String)) -> Option<Arc<DataKey>> {
        let cache = self.cache.read().await;
        cache
            .active
            .get(slot)
            .and_then(|id| cache.by_id.get(id))
            .cloned()
    }

    async fn key_by_id(&self, family: Family, id: Uuid) -> Result<Arc<DataKey>, CryptoError> {
        if let Some(key) = self.cache.read().await.by_id.get(&id).cloned() {
            return Ok(key);
        }
        let _loading = self.loading.lock().await;
        if let Some(key) = self.cache.read().await.by_id.get(&id).cloned() {
            return Ok(key);
        }
        let row = self
            .store
            .find(id)
            .await?
            .ok_or_else(|| CryptoError::UnknownKey(format!("data key {id}")))?;
        if row.family != family {
            return Err(CryptoError::Rejected);
        }
        let key = Arc::new(self.unwrap(&row).await?);
        self.cache.write().await.by_id.insert(id, Arc::clone(&key));
        Ok(key)
    }

    async fn unwrap(&self, row: &DataKeyRow) -> Result<DataKey, CryptoError> {
        let plain = self.bao.decrypt(KEK, &row.wrapped).await?;
        let bytes: [u8; KEY_LEN] = plain.as_slice().try_into().map_err(|_| {
            CryptoError::Unavailable("an unwrapped data key is not 256 bits".into())
        })?;
        Ok(DataKey {
            id: row.id,
            family: row.family,
            key_version: wrapped_version(&row.wrapped)?,
            bytes: Zeroizing::new(bytes),
        })
    }
}

#[async_trait]
impl Crypto for OpenBaoCrypto {
    async fn seal(
        &self,
        family: Family,
        aad: &[u8],
        plaintext: &[u8],
    ) -> Result<Sealed, CryptoError> {
        self.active_key(family, &period_of(Utc::now()))
            .await?
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
        self.key_by_id(family, sealed.dek_id)
            .await?
            .open(aad, sealed)
    }

    async fn keyed_hash(&self, purpose: &Purpose, data: &[u8]) -> Result<[u8; 32], CryptoError> {
        let key = self
            .hash_keys
            .by_purpose
            .get(purpose.name())
            .ok_or_else(|| CryptoError::UnknownKey(format!("purpose {}", purpose.name())))?;
        hmac_sha256(key.as_slice(), &[data])
    }

    async fn sign(&self, key: &str, msg: &[u8]) -> Result<Signature, CryptoError> {
        Signature::from_transit(&self.bao.sign(key, msg).await?)
    }

    async fn verify(&self, key: &str, msg: &[u8], sig: &Signature) -> Result<bool, CryptoError> {
        self.bao.verify(key, msg, &sig.to_transit()).await
    }
}

/// The `n` of `vault:v<n>:<base64>`: the KEK version that wrapped a data key.
pub fn wrapped_version(wrapped: &str) -> Result<u32, CryptoError> {
    wrapped
        .strip_prefix("vault:v")
        .and_then(|rest| rest.split_once(':'))
        .and_then(|(version, _)| version.parse::<u32>().ok())
        .ok_or_else(|| {
            CryptoError::Unavailable("a wrapped data key is not vault:v<n>:<base64>".into())
        })
}

pub fn period_of(at: DateTime<Utc>) -> String {
    format!("{:04}-{:02}", at.year(), at.month())
}

fn period_after(at: DateTime<Utc>) -> String {
    if at.month() == 12 {
        format!("{:04}-01", at.year() + 1)
    } else {
        format!("{:04}-{:02}", at.year(), at.month() + 1)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use chrono::TimeZone;
    use serde_json::json;

    use super::*;

    #[test]
    fn a_period_is_the_utc_month_and_december_rolls_the_year() {
        let june = Utc.with_ymd_and_hms(2026, 6, 30, 23, 59, 59).unwrap();
        assert_eq!(period_of(june), "2026-06");
        assert_eq!(period_after(june), "2026-07");
        let december = Utc.with_ymd_and_hms(2026, 12, 1, 0, 0, 0).unwrap();
        assert_eq!(period_of(december), "2026-12");
        assert_eq!(period_after(december), "2027-01");
    }

    #[test]
    fn the_kek_version_is_read_from_the_wrapped_key() {
        assert_eq!(wrapped_version("vault:v3:AAAA").unwrap(), 3);
        assert!(wrapped_version("AAAA").is_err());
        assert!(wrapped_version("vault:vx:AAAA").is_err());
    }

    #[test]
    fn a_kv_key_item_needs_256_bits_of_base64() {
        let good = json!({ "key": BASE64.encode([7u8; 32]) });
        assert_eq!(entry_key("keys/test", &good).unwrap().len(), 32);
        assert!(entry_key("keys/test", &json!({ "key": BASE64.encode([7u8; 16]) })).is_err());
        assert!(entry_key("keys/test", &json!({ "key": "not base64 !" })).is_err());
        assert!(entry_key("keys/test", &json!({})).is_err());
    }
}
