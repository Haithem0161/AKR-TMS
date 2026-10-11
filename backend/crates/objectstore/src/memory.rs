//! An object store held in memory, for tests and for a developer's machine with no store running.
//!
//! It keeps every version of a key and refuses what the port refuses (a digest that does not match, a
//! key that is not a plain path, a URL life outside 60 to 300 seconds, a bucket nobody created). It does
//! not enforce retention: the conformance suite proves that against a real store.

use std::collections::{BTreeSet, HashMap};
use std::sync::Mutex;
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use sha2::{Digest, Sha256};
use url::Url;

use crate::{
    LockMode, ObjectMeta, ObjectStore, ObjectStoreError, Retention, VersionId, buckets, check_key,
    check_ttl,
};

struct Stored {
    version_id: VersionId,
    bytes: Bytes,
    sha256: [u8; 32],
    retention: Option<Retention>,
}

#[derive(Default)]
struct Inner {
    objects: HashMap<(String, String), Vec<Stored>>,
    writes: u64,
}

pub struct MemoryObjectStore {
    buckets: BTreeSet<String>,
    inner: Mutex<Inner>,
}

impl Default for MemoryObjectStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryObjectStore {
    /// Holds the five buckets `akr-ctl store init` creates.
    pub fn new() -> Self {
        Self::with_buckets(buckets::ALL)
    }

    pub fn with_buckets<'a>(names: impl IntoIterator<Item = &'a str>) -> Self {
        Self {
            buckets: names.into_iter().map(str::to_owned).collect(),
            inner: Mutex::new(Inner::default()),
        }
    }

    fn check_bucket(&self, bucket: &str) -> Result<(), ObjectStoreError> {
        if self.buckets.contains(bucket) {
            Ok(())
        } else {
            Err(ObjectStoreError::NotFound(bucket.to_owned()))
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// How many versions the key has: an overwrite adds one and removes none.
    pub fn version_count(&self, bucket: &str, key: &str) -> usize {
        self.lock()
            .objects
            .get(&(bucket.to_owned(), key.to_owned()))
            .map_or(0, Vec::len)
    }
}

#[async_trait]
impl ObjectStore for MemoryObjectStore {
    async fn put(
        &self,
        bucket: &str,
        key: &str,
        bytes: Bytes,
        sha256: [u8; 32],
        lock: LockMode,
    ) -> Result<VersionId, ObjectStoreError> {
        self.check_bucket(bucket)?;
        check_key(key)?;
        let actual: [u8; 32] = Sha256::digest(&bytes).into();
        if actual != sha256 {
            return Err(ObjectStoreError::ChecksumMismatch {
                key: key.to_owned(),
                side: "the digest its writer gave",
            });
        }
        let mut inner = self.lock();
        inner.writes += 1;
        let version_id = VersionId(format!("mem-{:08}", inner.writes));
        inner
            .objects
            .entry((bucket.to_owned(), key.to_owned()))
            .or_default()
            .push(Stored {
                version_id: version_id.clone(),
                bytes,
                sha256,
                retention: lock.retention(),
            });
        Ok(version_id)
    }

    async fn get(&self, bucket: &str, key: &str) -> Result<Bytes, ObjectStoreError> {
        self.check_bucket(bucket)?;
        check_key(key)?;
        self.lock()
            .objects
            .get(&(bucket.to_owned(), key.to_owned()))
            .and_then(|versions| versions.last())
            .map(|stored| stored.bytes.clone())
            .ok_or_else(|| ObjectStoreError::NotFound(format!("{bucket}/{key}")))
    }

    async fn head(&self, bucket: &str, key: &str) -> Result<Option<ObjectMeta>, ObjectStoreError> {
        self.check_bucket(bucket)?;
        check_key(key)?;
        Ok(self
            .lock()
            .objects
            .get(&(bucket.to_owned(), key.to_owned()))
            .and_then(|versions| versions.last())
            .map(|stored| ObjectMeta {
                size: stored.bytes.len() as u64,
                version_id: stored.version_id.clone(),
                sha256: Some(stored.sha256),
                retention: stored.retention,
                legal_hold: false,
            }))
    }

    fn presign_get(&self, bucket: &str, key: &str, ttl: Duration) -> Result<Url, ObjectStoreError> {
        self.check_bucket(bucket)?;
        check_key(key)?;
        let expires_s = check_ttl(ttl)?;
        let mut url = Url::parse("memory://object-store/")
            .map_err(|e| ObjectStoreError::Config(e.to_string()))?;
        url.set_path(&format!("/{bucket}/{key}"));
        url.set_query(Some(&format!("X-Amz-Expires={expires_s}")));
        Ok(url)
    }

    async fn head_bucket(&self, bucket: &str) -> Result<(), ObjectStoreError> {
        self.check_bucket(bucket)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;

    fn digest(bytes: &[u8]) -> [u8; 32] {
        Sha256::digest(bytes).into()
    }

    #[tokio::test]
    async fn what_is_written_is_read_back_with_its_digest_and_lock() {
        let store = MemoryObjectStore::new();
        let until = Utc.with_ymd_and_hms(2036, 10, 11, 0, 0, 0).unwrap();
        let body = Bytes::from_static(b"evidence");
        let version = store
            .put(
                buckets::EVIDENCE,
                "cases/1/a.pdf",
                body.clone(),
                digest(&body),
                LockMode::Compliance { until },
            )
            .await
            .unwrap();

        assert_eq!(
            store.get(buckets::EVIDENCE, "cases/1/a.pdf").await.unwrap(),
            body
        );
        let meta = store
            .head(buckets::EVIDENCE, "cases/1/a.pdf")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(meta.size, 8);
        assert_eq!(meta.version_id, version);
        assert_eq!(meta.sha256, Some(digest(&body)));
        assert_eq!(meta.retention.unwrap().until, until);
        assert_eq!(
            store
                .head(buckets::EVIDENCE, "cases/1/b.pdf")
                .await
                .unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn an_overwrite_adds_a_version_and_keeps_the_old_one() {
        let store = MemoryObjectStore::new();
        let first = store
            .put(
                buckets::EVIDENCE,
                "k",
                Bytes::from_static(b"one"),
                digest(b"one"),
                LockMode::BucketDefault,
            )
            .await
            .unwrap();
        let second = store
            .put(
                buckets::EVIDENCE,
                "k",
                Bytes::from_static(b"two"),
                digest(b"two"),
                LockMode::BucketDefault,
            )
            .await
            .unwrap();
        assert_ne!(first, second);
        assert_eq!(store.version_count(buckets::EVIDENCE, "k"), 2);
        assert_eq!(store.get(buckets::EVIDENCE, "k").await.unwrap(), "two");
    }

    #[tokio::test]
    async fn bytes_that_do_not_match_their_digest_are_not_stored() {
        let store = MemoryObjectStore::new();
        let refused = store
            .put(
                buckets::EVIDENCE,
                "k",
                Bytes::from_static(b"tampered"),
                digest(b"original"),
                LockMode::BucketDefault,
            )
            .await;
        assert!(matches!(
            refused,
            Err(ObjectStoreError::ChecksumMismatch { .. })
        ));
        assert_eq!(store.version_count(buckets::EVIDENCE, "k"), 0);
    }

    #[tokio::test]
    async fn an_unknown_bucket_a_missing_key_and_a_bad_key_are_told_apart() {
        let store = MemoryObjectStore::with_buckets(["akr-evidence"]);
        assert!(store.head_bucket("akr-evidence").await.is_ok());
        assert!(matches!(
            store.head_bucket("akr-anchors").await,
            Err(ObjectStoreError::NotFound(_))
        ));
        assert!(matches!(
            store.get("akr-evidence", "absent").await,
            Err(ObjectStoreError::NotFound(_))
        ));
        assert!(matches!(
            store.get("akr-evidence", "a/../b").await,
            Err(ObjectStoreError::InvalidKey(_))
        ));
    }

    #[test]
    fn a_presigned_url_keeps_the_port_rule() {
        let store = MemoryObjectStore::new();
        let url = store
            .presign_get(buckets::EXPORTS, "a/b.pdf", Duration::from_secs(60))
            .unwrap();
        assert_eq!(
            url.as_str(),
            "memory://object-store/akr-exports/a/b.pdf?X-Amz-Expires=60"
        );
        assert!(matches!(
            store.presign_get(buckets::EXPORTS, "a/b.pdf", Duration::from_secs(30)),
            Err(ObjectStoreError::PresignTtl(30))
        ));
    }
}
