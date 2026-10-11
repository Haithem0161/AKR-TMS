//! The WORM conformance suite (TS-22): what `akr-ctl store conformance` runs at every installation and
//! upgrade, and what the release pipeline runs against the store build.
//!
//! It writes into a scratch bucket of its own and proves, with the strongest credential the operator
//! holds, that the store keeps a locked version: no delete, no shortened retention, no deleted bucket, an
//! overwrite that leaves the old version readable. A refusal counts only when the store gave it. The S3
//! guard refuses the same requests before the store sees them, so every check records who answered.

use std::time::Duration;

use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::client::{DefaultRetention, RetentionPeriod, S3Client};
use crate::{
    LockMode, ObjectStore, ObjectStoreError, PRESIGN_TTL_MIN, Retention, RetentionMode, VersionId,
};

/// The scratch bucket. It is created with object lock, so it is never one of the product's buckets.
pub const DEFAULT_BUCKET: &str = "akr-conformance";

const GUARD_CODE_PREFIX: &str = "s3guard:";
const DEFAULT_RETENTION_DAYS: u32 = 1;
const PRESIGN_PROBE_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct Options {
    pub bucket: String,
    /// How long the test version stays locked. It cannot be removed sooner, by anyone.
    pub lock_for: Duration,
    /// The widest gap allowed between the store's clock and ours: retention runs on the store's.
    pub max_clock_skew: Duration,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            bucket: DEFAULT_BUCKET.to_owned(),
            lock_for: Duration::from_secs(120),
            max_clock_skew: Duration::from_secs(5),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Check {
    pub name: &'static str,
    pub passed: bool,
    pub detail: String,
    /// The S3 guard answered, so the store was not exercised by this check.
    pub answered_by_guard: bool,
}

/// One version the suite wrote, as a restore drill must find it again.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub bucket: String,
    pub key: String,
    pub version_id: String,
    pub sha256: String,
    pub retention_mode: String,
    pub retain_until: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Report {
    pub checks: Vec<Check>,
    pub manifest: Vec<ManifestEntry>,
}

impl Report {
    pub fn passed(&self) -> bool {
        !self.checks.is_empty() && self.checks.iter().all(|check| check.passed)
    }

    /// False when the guard answered any check: the run proves the guard, not the store.
    pub fn store_exercised(&self) -> bool {
        !self.checks.iter().any(|check| check.answered_by_guard)
    }

    fn pass(&mut self, name: &'static str, detail: impl Into<String>) {
        self.checks.push(Check {
            name,
            passed: true,
            detail: detail.into(),
            answered_by_guard: false,
        });
    }

    fn fail(&mut self, name: &'static str, detail: impl Into<String>) {
        self.checks.push(Check {
            name,
            passed: false,
            detail: detail.into(),
            answered_by_guard: false,
        });
    }

    fn settle<T>(
        &mut self,
        name: &'static str,
        result: Result<T, ObjectStoreError>,
        detail: impl FnOnce(&T) -> String,
    ) -> Option<T> {
        match result {
            Ok(value) => {
                self.pass(name, detail(&value));
                Some(value)
            }
            Err(e) => {
                self.fail(name, e.to_string());
                None
            }
        }
    }

    /// Passes only when the request was refused. `what` names it for the report.
    fn refusal<T>(&mut self, name: &'static str, result: Result<T, ObjectStoreError>, what: &str) {
        match result {
            Err(ObjectStoreError::Refused { status, code, .. }) => {
                let by_guard = code.starts_with(GUARD_CODE_PREFIX);
                self.checks.push(Check {
                    name,
                    passed: true,
                    detail: format!("{what} was refused: {status} {code}"),
                    answered_by_guard: by_guard,
                });
            }
            Ok(_) => self.fail(name, format!("{what} was accepted")),
            Err(e) => self.fail(name, format!("{what} was not answered: {e}")),
        }
    }
}

fn digest(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn whole_seconds(at: DateTime<Utc>) -> DateTime<Utc> {
    DateTime::from_timestamp(at.timestamp(), 0).unwrap_or(at)
}

/// Runs every check. A check that cannot run because an earlier one failed is left out, and the report
/// then fails on the earlier one.
pub async fn run(store: &S3Client, options: &Options) -> Report {
    let mut report = Report::default();
    let bucket = options.bucket.as_str();
    let started = Utc::now();
    let key = format!("run-{}/locked.bin", started.format("%Y%m%dT%H%M%S%3fZ"));
    let defaulted_key = format!("run-{}/defaulted.bin", started.format("%Y%m%dT%H%M%S%3fZ"));

    let created = store.create_bucket(bucket, true).await;
    if report
        .settle("locked_bucket_created", created, |new| {
            if *new {
                format!("created `{bucket}` with object lock")
            } else {
                format!("`{bucket}` was already there")
            }
        })
        .is_none()
    {
        return report;
    }

    let wanted = DefaultRetention {
        mode: RetentionMode::Compliance,
        period: RetentionPeriod::Days(DEFAULT_RETENTION_DAYS),
    };
    let configured = match store.put_default_retention(bucket, wanted).await {
        Ok(()) => store.object_lock(bucket).await,
        Err(e) => Err(e),
    };
    match configured {
        Ok(lock) if lock.enabled && lock.default == Some(wanted) => report.pass(
            "default_retention_set",
            format!("COMPLIANCE, {DEFAULT_RETENTION_DAYS} day"),
        ),
        Ok(lock) => {
            report.fail(
                "default_retention_set",
                format!("the bucket reports {lock:?}"),
            );
            return report;
        }
        Err(e) => {
            report.fail("default_retention_set", e.to_string());
            return report;
        }
    }

    let original = Bytes::from(format!("akr-tms conformance {started}"));
    let until = whole_seconds(
        Utc::now() + chrono::Duration::from_std(options.lock_for).unwrap_or_default(),
    );
    let written = store
        .put(
            bucket,
            &key,
            original.clone(),
            digest(&original),
            LockMode::Compliance { until },
        )
        .await;
    let Some(version) = report.settle("locked_version_written", written, |version| {
        format!("version {version}, locked until {until}")
    }) else {
        return report;
    };
    if version.as_str() == VersionId::UNVERSIONED {
        report.fail(
            "locked_version_written",
            "the store named no version: the bucket is not versioned",
        );
        return report;
    }
    report.manifest.push(ManifestEntry {
        bucket: bucket.to_owned(),
        key: key.clone(),
        version_id: version.0.clone(),
        sha256: hex::encode(digest(&original)),
        retention_mode: RetentionMode::Compliance.as_str().to_owned(),
        retain_until: until,
    });

    match store.head(bucket, &key).await {
        Ok(Some(meta))
            if meta.sha256 == Some(digest(&original))
                && meta.retention
                    == Some(Retention {
                        mode: RetentionMode::Compliance,
                        until,
                    }) =>
        {
            report.pass(
                "checksum_and_lock_recorded",
                "the store holds the SHA-256 and the retention the write named",
            );
        }
        Ok(meta) => report.fail(
            "checksum_and_lock_recorded",
            format!("the store reports {meta:?}"),
        ),
        Err(e) => report.fail("checksum_and_lock_recorded", e.to_string()),
    }

    let read_back = store.get_version(bucket, &key, &version).await;
    match read_back {
        Ok(bytes) if bytes == original => {
            report.pass("checksum_verified_on_read", "the bytes match their digest");
        }
        Ok(_) => report.fail("checksum_verified_on_read", "the bytes differ"),
        Err(e) => report.fail("checksum_verified_on_read", e.to_string()),
    }

    let defaulted = Bytes::from_static(b"akr-tms conformance, bucket default");
    let defaulted_write = store
        .put(
            bucket,
            &defaulted_key,
            defaulted.clone(),
            digest(&defaulted),
            LockMode::BucketDefault,
        )
        .await;
    match defaulted_write {
        Ok(defaulted_version) => match store.head(bucket, &defaulted_key).await {
            Ok(Some(meta)) => match meta.retention {
                Some(retention)
                    if retention.mode == RetentionMode::Compliance
                        && retention.until > Utc::now() + chrono::Duration::hours(23) =>
                {
                    report.pass(
                        "default_retention_applied",
                        format!("a write naming no lock is held until {}", retention.until),
                    );
                    report.manifest.push(ManifestEntry {
                        bucket: bucket.to_owned(),
                        key: defaulted_key.clone(),
                        version_id: defaulted_version.0,
                        sha256: hex::encode(digest(&defaulted)),
                        retention_mode: retention.mode.as_str().to_owned(),
                        retain_until: retention.until,
                    });
                }
                other => report.fail(
                    "default_retention_applied",
                    format!("a write naming no lock carries {other:?}"),
                ),
            },
            Ok(None) => report.fail("default_retention_applied", "the object is not there"),
            Err(e) => report.fail("default_retention_applied", e.to_string()),
        },
        Err(e) => report.fail("default_retention_applied", e.to_string()),
    }

    report.refusal(
        "locked_version_delete_refused",
        store
            .delete_object(bucket, &key, Some(&version), true)
            .await,
        "deleting the locked version",
    );

    let shorter = Retention {
        mode: RetentionMode::Compliance,
        until: until - chrono::Duration::seconds(30),
    };
    report.refusal(
        "retention_shortening_refused",
        store
            .put_retention(bucket, &key, Some(&version), shorter, true)
            .await,
        "shortening the retention",
    );
    match store.retention(bucket, &key, Some(&version)).await {
        Ok(Some(retention)) if retention.until == until => {
            report.pass("retention_unchanged", format!("still held until {until}"));
        }
        Ok(other) => report.fail(
            "retention_unchanged",
            format!("the version carries {other:?}"),
        ),
        Err(e) => report.fail("retention_unchanged", e.to_string()),
    }

    let replacement = Bytes::from_static(b"akr-tms conformance, overwrite");
    let overwrite = store
        .put(
            bucket,
            &key,
            replacement.clone(),
            digest(&replacement),
            LockMode::Compliance { until },
        )
        .await;
    match overwrite {
        Ok(second) if second != version => match store.get_version(bucket, &key, &version).await {
            Ok(bytes) if bytes == original => report.pass(
                "overwrite_keeps_locked_version",
                format!("the write made version {second}; version {version} reads as written"),
            ),
            Ok(_) => report.fail(
                "overwrite_keeps_locked_version",
                "the locked version no longer holds its bytes",
            ),
            Err(e) => report.fail("overwrite_keeps_locked_version", e.to_string()),
        },
        Ok(_) => report.fail(
            "overwrite_keeps_locked_version",
            "the write replaced the locked version in place",
        ),
        Err(ObjectStoreError::Refused { status, code, .. }) => report.pass(
            "overwrite_keeps_locked_version",
            format!("the store refused the overwrite: {status} {code}"),
        ),
        Err(e) => report.fail("overwrite_keeps_locked_version", e.to_string()),
    }

    let hold = async {
        store
            .put_legal_hold(bucket, &key, Some(&version), true)
            .await?;
        let on = store.legal_hold(bucket, &key, Some(&version)).await?;
        store
            .put_legal_hold(bucket, &key, Some(&version), false)
            .await?;
        let off = store.legal_hold(bucket, &key, Some(&version)).await?;
        Ok::<_, ObjectStoreError>((on, off))
    }
    .await;
    match hold {
        Ok((true, false)) => report.pass("legal_hold_set_and_cleared", "ON, then OFF"),
        Ok((on, off)) => report.fail(
            "legal_hold_set_and_cleared",
            format!("after setting it reads {on}, after clearing it reads {off}"),
        ),
        Err(e) => report.fail("legal_hold_set_and_cleared", e.to_string()),
    }

    report.refusal(
        "bucket_deletion_refused",
        store.delete_bucket(bucket).await,
        "deleting the bucket",
    );

    presigned_urls(store, bucket, &key, &replacement, &mut report).await;

    match store.server_time(bucket).await {
        Ok(theirs) => {
            let skew = (theirs - Utc::now()).abs();
            let allowed = chrono::Duration::from_std(options.max_clock_skew).unwrap_or_default();
            // The Date header has whole seconds, so a store in step can read up to a second away.
            if skew <= allowed + chrono::Duration::seconds(1) {
                report.pass(
                    "clock_in_step",
                    format!(
                        "the store's clock is within {} s of ours",
                        skew.num_seconds() + 1
                    ),
                );
            } else {
                report.fail(
                    "clock_in_step",
                    format!(
                        "the store's clock is {} s away from ours; retention runs on the store's",
                        skew.num_seconds()
                    ),
                );
            }
        }
        Err(e) => report.fail("clock_in_step", e.to_string()),
    }

    report
}

async fn presigned_urls(
    store: &S3Client,
    bucket: &str,
    key: &str,
    expected: &Bytes,
    report: &mut Report,
) {
    const LIVE: &str = "presigned_url_reads";
    const EXPIRED: &str = "presigned_url_expires";

    let http = match reqwest::Client::builder()
        .timeout(PRESIGN_PROBE_TIMEOUT)
        .build()
    {
        Ok(http) => http,
        Err(e) => {
            report.fail(LIVE, e.to_string());
            return;
        }
    };

    match store.presign_get(bucket, key, PRESIGN_TTL_MIN) {
        Ok(url) => match http.get(url).send().await {
            Ok(response) if response.status().is_success() => match response.bytes().await {
                Ok(bytes) if bytes == *expected => {
                    report.pass(LIVE, "a fresh URL reads the object with no credential");
                }
                Ok(_) => report.fail(LIVE, "a fresh URL read other bytes"),
                Err(e) => report.fail(LIVE, e.without_url().to_string()),
            },
            Ok(response) => {
                report.fail(LIVE, format!("a fresh URL answered {}", response.status()))
            }
            Err(e) => report.fail(LIVE, e.without_url().to_string()),
        },
        Err(e) => report.fail(LIVE, e.to_string()),
    }

    let signed_at = Utc::now()
        - chrono::Duration::from_std(PRESIGN_TTL_MIN).unwrap_or_default()
        - chrono::Duration::seconds(60);
    match store.presign_get_at(bucket, key, PRESIGN_TTL_MIN, signed_at) {
        Ok(url) => match http.get(url).send().await {
            Ok(response) if response.status().as_u16() == 403 => {
                report.pass(EXPIRED, "a URL past its life answers 403");
            }
            Ok(response) => report.fail(
                EXPIRED,
                format!("a URL past its life answered {}", response.status()),
            ),
            Err(e) => report.fail(EXPIRED, e.without_url().to_string()),
        },
        Err(e) => report.fail(EXPIRED, e.to_string()),
    }
}

/// After a restore drill: every version a run wrote is there again, with its bytes and its retention.
pub async fn verify_restore(store: &S3Client, manifest: &[ManifestEntry]) -> Report {
    let mut report = Report::default();
    if manifest.is_empty() {
        report.fail("restored_versions", "the manifest names no version");
        return report;
    }
    for entry in manifest {
        let version = VersionId(entry.version_id.clone());
        match store.get_version(&entry.bucket, &entry.key, &version).await {
            Ok(bytes) if hex::encode(digest(&bytes)) == entry.sha256 => report.pass(
                "restored_version",
                format!("{}/{} version {version}", entry.bucket, entry.key),
            ),
            Ok(_) => report.fail(
                "restored_version",
                format!(
                    "{}/{} version {version} holds other bytes",
                    entry.bucket, entry.key
                ),
            ),
            Err(e) => report.fail("restored_version", e.to_string()),
        }
        match store
            .retention(&entry.bucket, &entry.key, Some(&version))
            .await
        {
            Ok(Some(retention))
                if retention.mode.as_str() == entry.retention_mode
                    && retention.until == entry.retain_until =>
            {
                report.pass(
                    "restored_retention",
                    format!(
                        "{}/{} is held until {}",
                        entry.bucket, entry.key, retention.until
                    ),
                );
            }
            Ok(other) => report.fail(
                "restored_retention",
                format!("{}/{} carries {other:?}", entry.bucket, entry.key),
            ),
            Err(e) => report.fail("restored_retention", e.to_string()),
        }
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    fn refused(code: &str) -> Result<(), ObjectStoreError> {
        Err(ObjectStoreError::Refused {
            operation: "DeleteObject",
            status: 403,
            code: code.to_owned(),
        })
    }

    #[test]
    fn a_refusal_passes_and_says_who_gave_it() {
        let mut report = Report::default();
        report.refusal("a", refused("ObjectLocked"), "deleting");
        report.refusal(
            "b",
            refused("s3guard:delete_outside_quarantine"),
            "deleting",
        );
        assert!(report.passed());
        assert!(!report.checks[0].answered_by_guard);
        assert!(report.checks[1].answered_by_guard);
        assert!(!report.store_exercised());
    }

    #[test]
    fn an_accepted_request_or_a_silent_store_fails_the_check() {
        let mut report = Report::default();
        report.refusal("a", Ok(()), "deleting");
        report.refusal(
            "b",
            Err::<(), _>(ObjectStoreError::Unavailable("connection refused".into())),
            "deleting",
        );
        assert!(!report.passed());
        assert_eq!(report.checks[0].detail, "deleting was accepted");
        assert!(report.checks.iter().all(|check| !check.passed));
    }

    #[test]
    fn an_empty_report_has_not_passed() {
        assert!(!Report::default().passed());
        assert!(Report::default().store_exercised());
    }
}
