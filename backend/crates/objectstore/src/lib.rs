//! The object-store port (TS-22, TS-41, TS-42).
//!
//! The product talks to its store over the S3 API only, and every request is shaped to pass the S3 guard:
//! a single signed part, every `x-amz-*` header signed, no credentials in the query of a write. The store
//! behind the port is a deployment choice (the Silo build, SeaweedFS, RustFS, a bank's WORM array).

use std::sync::Arc;
use std::time::Duration;

use akr_common::config::{CoreConfig, S3Credential};
use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use url::Url;

pub mod client;
pub mod conformance;
pub mod memory;
pub mod sigv4;
mod xml;

pub use client::{BucketLock, DefaultRetention, ObjectVersion, RetentionPeriod, S3Client};
pub use memory::MemoryObjectStore;

/// The buckets `akr-ctl store init` creates.
pub mod buckets {
    /// Object lock, COMPLIANCE; the default retention per class is set by phase 02.
    pub const EVIDENCE: &str = "akr-evidence";
    /// Object lock, COMPLIANCE, ten years.
    pub const ANCHORS: &str = "akr-anchors";
    /// The only bucket an application credential may delete from.
    pub const QUARANTINE: &str = "akr-quarantine";
    pub const MODELS: &str = "akr-models";
    pub const EXPORTS: &str = "akr-exports";

    pub const ALL: [&str; 5] = [EVIDENCE, ANCHORS, QUARANTINE, MODELS, EXPORTS];
}

/// The store of a binary: the S3 client, pointed at the guard, when `S3_ENDPOINT` is set; without it,
/// outside production, a store in memory.
pub fn connect(
    core: &CoreConfig,
    credential: Option<S3Credential>,
) -> Result<Arc<dyn ObjectStore>, ObjectStoreError> {
    match (&core.s3_endpoint, credential) {
        (Some(endpoint), Some(credential)) => Ok(Arc::new(S3Client::new(endpoint, credential)?)),
        (Some(_), None) => Err(ObjectStoreError::Config(
            "S3_ENDPOINT is set and no object-store credential was read".to_owned(),
        )),
        (None, _) if core.env.is_production() => Err(ObjectStoreError::Config(
            "S3_ENDPOINT is not set".to_owned(),
        )),
        (None, _) => {
            tracing::warn!("S3_ENDPOINT is not set: objects are kept in memory and lost at exit");
            Ok(Arc::new(MemoryObjectStore::new()))
        }
    }
}

/// The shortest and the longest life of a presigned URL.
pub const PRESIGN_TTL_MIN: Duration = Duration::from_secs(60);
pub const PRESIGN_TTL_MAX: Duration = Duration::from_secs(300);

#[derive(Debug, thiserror::Error)]
pub enum ObjectStoreError {
    #[error("the object store is unreachable: {0}")]
    Unavailable(String),
    #[error("the object store has no `{0}`")]
    NotFound(String),
    #[error("the object store refused {operation}: {status} {code}")]
    Refused {
        operation: &'static str,
        status: u16,
        /// The S3 error code, or empty when the answer carried none.
        code: String,
    },
    #[error("a presigned URL lives 60 to 300 seconds, not {0}")]
    PresignTtl(u64),
    #[error("the SHA-256 of `{key}` does not match {side}")]
    ChecksumMismatch { key: String, side: &'static str },
    #[error("`{0}` cannot be an object key")]
    InvalidKey(String),
    #[error("the object store answered {operation} with something unreadable: {reason}")]
    Malformed {
        operation: &'static str,
        reason: String,
    },
    #[error("object store configuration: {0}")]
    Config(String),
}

/// The store's name for one version of an object. An unversioned bucket names its only version `null`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionId(pub String);

impl VersionId {
    pub const UNVERSIONED: &'static str = "null";

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for VersionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetentionMode {
    /// Nobody shortens or removes it, the root credential included.
    Compliance,
    /// A credential holding `s3:BypassGovernanceRetention` may; no application credential does.
    Governance,
}

impl RetentionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compliance => "COMPLIANCE",
            Self::Governance => "GOVERNANCE",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "COMPLIANCE" => Some(Self::Compliance),
            "GOVERNANCE" => Some(Self::Governance),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Retention {
    pub mode: RetentionMode,
    pub until: DateTime<Utc>,
}

/// The lock a write asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockMode {
    /// The write names no lock: the bucket's default retention applies, if it has one.
    BucketDefault,
    Governance {
        until: DateTime<Utc>,
    },
    Compliance {
        until: DateTime<Utc>,
    },
}

impl LockMode {
    pub fn retention(self) -> Option<Retention> {
        match self {
            Self::BucketDefault => None,
            Self::Governance { until } => Some(Retention {
                mode: RetentionMode::Governance,
                until,
            }),
            Self::Compliance { until } => Some(Retention {
                mode: RetentionMode::Compliance,
                until,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectMeta {
    pub size: u64,
    pub version_id: VersionId,
    /// `None` for an object that was not written through this port.
    pub sha256: Option<[u8; 32]>,
    pub retention: Option<Retention>,
    pub legal_hold: bool,
}

#[async_trait]
pub trait ObjectStore: Send + Sync {
    /// Writes one object in a single signed part. `sha256` is the digest the caller computed; the write
    /// is refused before it leaves when the bytes do not match it, and the store checks it again.
    async fn put(
        &self,
        bucket: &str,
        key: &str,
        bytes: Bytes,
        sha256: [u8; 32],
        lock: LockMode,
    ) -> Result<VersionId, ObjectStoreError>;

    /// The latest version, checked against the digest the store recorded.
    async fn get(&self, bucket: &str, key: &str) -> Result<Bytes, ObjectStoreError>;

    /// `None` when the bucket holds no such key.
    async fn head(&self, bucket: &str, key: &str) -> Result<Option<ObjectMeta>, ObjectStoreError>;

    /// A URL that reads the object without a credential for `ttl`, which is 60 to 300 seconds. There is
    /// no presigned write.
    fn presign_get(&self, bucket: &str, key: &str, ttl: Duration) -> Result<Url, ObjectStoreError>;

    /// Succeeds when the bucket exists and the credential may reach it: a binary's readiness check.
    async fn head_bucket(&self, bucket: &str) -> Result<(), ObjectStoreError>;
}

/// A key is a path of non-empty segments, none of them `.` or `..`: a URL parser would fold those, and
/// the request would then name another object than the one that was signed.
pub(crate) fn check_key(key: &str) -> Result<(), ObjectStoreError> {
    let sound = !key.is_empty()
        && key.len() <= 1024
        && !key.chars().any(char::is_control)
        && key
            .split('/')
            .all(|segment| !segment.is_empty() && segment != "." && segment != "..");
    if sound {
        Ok(())
    } else {
        Err(ObjectStoreError::InvalidKey(key.to_owned()))
    }
}

pub(crate) fn check_ttl(ttl: Duration) -> Result<u64, ObjectStoreError> {
    if (PRESIGN_TTL_MIN..=PRESIGN_TTL_MAX).contains(&ttl) {
        Ok(ttl.as_secs())
    } else {
        Err(ObjectStoreError::PresignTtl(ttl.as_secs()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_key_is_a_plain_path() {
        for key in [
            "a",
            "evidence/2026/10/a b.pdf",
            "staging/ملف.bin",
            "a/.b/c..d",
        ] {
            assert!(check_key(key).is_ok(), "{key}");
        }
        for key in ["", "/a", "a/", "a//b", "a/./b", "a/../b", "..", "a\nb"] {
            assert!(
                matches!(check_key(key), Err(ObjectStoreError::InvalidKey(_))),
                "{key:?}"
            );
        }
    }

    #[test]
    fn a_presigned_url_lives_one_to_five_minutes() {
        assert_eq!(check_ttl(Duration::from_secs(60)).ok(), Some(60));
        assert_eq!(check_ttl(Duration::from_secs(300)).ok(), Some(300));
        for seconds in [0, 59, 301, 86_400] {
            assert!(matches!(
                check_ttl(Duration::from_secs(seconds)),
                Err(ObjectStoreError::PresignTtl(s)) if s == seconds
            ));
        }
    }
}
