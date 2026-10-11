//! `akr-ctl store`: create the buckets, run the WORM conformance suite.

use std::path::{Path, PathBuf};

use akr_common::config::S3Credential;
use akr_objectstore::conformance::{self, ManifestEntry, Options, Report};
use akr_objectstore::{
    BucketLock, DefaultRetention, RetentionMode, RetentionPeriod, S3Client, buckets,
};
use url::Url;
use zeroize::Zeroizing;

use crate::error::CtlError;

/// Object lock is set when a bucket is created and cannot be added later.
const LOCKED: [&str; 2] = [buckets::EVIDENCE, buckets::ANCHORS];
/// An anchor outlives every record it proves.
const ANCHOR_RETENTION: DefaultRetention = DefaultRetention {
    mode: RetentionMode::Compliance,
    period: RetentionPeriod::Years(10),
};

#[derive(Debug, Clone, clap::Args)]
pub struct StoreArgs {
    /// Scheme, host and port, with no path.
    #[arg(long, env = "S3_ENDPOINT")]
    pub endpoint: Url,

    /// The file that holds the access key id.
    #[arg(long, env = "AKR_STORE_ACCESS_KEY_FILE")]
    pub access_key_file: PathBuf,

    /// The file that holds the secret access key.
    #[arg(long, env = "AKR_STORE_SECRET_KEY_FILE")]
    pub secret_key_file: PathBuf,
}

fn read_line(path: &Path) -> Result<Zeroizing<String>, CtlError> {
    let text = Zeroizing::new(std::fs::read_to_string(path).map_err(CtlError::file(path))?);
    let line = text.trim();
    if line.is_empty() {
        return Err(CtlError::usage(format!("{} is empty", path.display())));
    }
    Ok(Zeroizing::new(line.to_owned()))
}

impl StoreArgs {
    pub fn client(&self) -> Result<S3Client, CtlError> {
        let credential = S3Credential {
            access_key_id: read_line(&self.access_key_file)?.as_str().to_owned(),
            secret_access_key: read_line(&self.secret_key_file)?,
        };
        Ok(S3Client::new(&self.endpoint, credential)?)
    }
}

fn describe(lock: BucketLock) -> String {
    match (lock.enabled, lock.default) {
        (false, _) => "no object lock".to_owned(),
        (true, None) => "object lock, no default retention".to_owned(),
        (true, Some(default)) => {
            let period = match default.period {
                RetentionPeriod::Days(days) => format!("{days} days"),
                RetentionPeriod::Years(years) => format!("{years} years"),
            };
            format!("object lock, {} {period}", default.mode.as_str())
        }
    }
}

/// Creates the product's buckets, or confirms the ones that are there. A bucket that should be locked and
/// was created without object lock is refused: nothing can lock it now.
pub async fn init(store: &S3Client) -> Result<Vec<String>, CtlError> {
    let mut lines = Vec::new();
    for bucket in buckets::ALL {
        let must_lock = LOCKED.contains(&bucket);
        let created = store.create_bucket(bucket, must_lock).await?;
        let mut lock = store.object_lock(bucket).await?;
        if must_lock && !lock.enabled {
            return Err(CtlError::refused(
                "bucket_not_locked",
                format!(
                    "bucket `{bucket}` exists without object lock; object lock is set when a bucket is created \
                     and cannot be added to this one"
                ),
            ));
        }
        if bucket == buckets::ANCHORS {
            if lock.default != Some(ANCHOR_RETENTION) {
                store
                    .put_default_retention(bucket, ANCHOR_RETENTION)
                    .await?;
                lock = store.object_lock(bucket).await?;
            }
            if lock.default != Some(ANCHOR_RETENTION) {
                return Err(CtlError::refused(
                    "anchor_retention",
                    format!(
                        "bucket `{bucket}` reports {} after the default was set",
                        describe(lock)
                    ),
                ));
            }
        }
        lines.push(format!(
            "bucket {bucket}: {} ({})",
            if created { "created" } else { "present" },
            describe(lock)
        ));
    }
    Ok(lines)
}

#[derive(Debug, Clone, Default, clap::Args)]
pub struct ConformanceArgs {
    /// The endpoint is the S3 guard: a refusal by the guard is then the expected answer of a check.
    #[arg(long)]
    pub through_guard: bool,

    /// Write the versions this run locked to a file, for the restore drill to verify later.
    #[arg(long, conflicts_with = "verify_restore")]
    pub manifest_out: Option<PathBuf>,

    /// Do not run the suite: verify that every version of this manifest is in the store again.
    #[arg(long)]
    pub verify_restore: Option<PathBuf>,
}

fn lines_of(report: &Report) -> Vec<String> {
    report
        .checks
        .iter()
        .map(|check| {
            format!(
                "{}  {}: {}{}",
                if check.passed { "ok  " } else { "FAIL" },
                check.name,
                check.detail,
                if check.answered_by_guard {
                    " (answered by the guard)"
                } else {
                    ""
                }
            )
        })
        .collect()
}

/// A run passes when every check passed and the store itself was exercised. Through the guard some checks
/// are answered by the guard; that proves the guard, and is accepted only when the caller says the guard is
/// what it is testing.
fn settle(report: &Report, through_guard: bool) -> Result<(), CtlError> {
    if !report.passed() {
        return Err(CtlError::ChecksFailed {
            failed: report.checks.iter().filter(|check| !check.passed).count(),
            total: report.checks.len(),
        });
    }
    if !report.store_exercised() && !through_guard {
        return Err(CtlError::refused(
            "guard_answered",
            "the S3 guard answered at least one check, so this run did not exercise the store: point the \
             command at the store, or give --through-guard when the guard is what is tested",
        ));
    }
    Ok(())
}

pub async fn conformance(
    store: &S3Client,
    args: &ConformanceArgs,
) -> (Vec<String>, Result<(), CtlError>) {
    if let Some(path) = &args.verify_restore {
        let entries = match read_manifest(path) {
            Ok(entries) => entries,
            Err(e) => return (Vec::new(), Err(e)),
        };
        let report = conformance::verify_restore(store, &entries).await;
        return (lines_of(&report), settle(&report, true));
    }

    let report = conformance::run(store, &Options::default()).await;
    let mut lines = lines_of(&report);
    let mut outcome = settle(&report, args.through_guard);
    if let Some(path) = &args.manifest_out {
        match write_manifest(path, &report.manifest) {
            Ok(()) => lines.push(format!(
                "manifest: {} locked versions written to {}",
                report.manifest.len(),
                path.display()
            )),
            Err(e) => outcome = outcome.and(Err(e)),
        }
    }
    (lines, outcome)
}

fn read_manifest(path: &Path) -> Result<Vec<ManifestEntry>, CtlError> {
    let bytes = std::fs::read(path).map_err(CtlError::file(path))?;
    serde_json::from_slice(&bytes).map_err(|e| {
        CtlError::usage(format!(
            "{} is not a conformance manifest: {e}",
            path.display()
        ))
    })
}

fn write_manifest(path: &Path, manifest: &[ManifestEntry]) -> Result<(), CtlError> {
    let json = serde_json::to_vec_pretty(manifest)
        .map_err(|e| CtlError::usage(format!("the manifest cannot be written: {e}")))?;
    std::fs::write(path, json).map_err(CtlError::file(path))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use akr_objectstore::conformance::Check;

    use super::*;

    fn check(passed: bool, answered_by_guard: bool) -> Check {
        Check {
            name: "delete_locked_version",
            passed,
            detail: "refused".to_owned(),
            answered_by_guard,
        }
    }

    fn report(checks: Vec<Check>) -> Report {
        Report {
            checks,
            manifest: Vec::new(),
        }
    }

    #[test]
    fn a_run_the_guard_answered_passes_only_when_the_guard_is_what_is_tested() {
        let answered = report(vec![check(true, false), check(true, true)]);
        assert_eq!(
            settle(&answered, false).unwrap_err().code(),
            Some("guard_answered")
        );
        assert!(settle(&answered, true).is_ok());
        assert!(settle(&report(vec![check(true, false)]), false).is_ok());
    }

    #[test]
    fn a_failed_check_fails_the_run_and_an_empty_report_is_no_pass() {
        let failed = report(vec![check(true, false), check(false, false)]);
        assert!(matches!(
            settle(&failed, true),
            Err(CtlError::ChecksFailed {
                failed: 1,
                total: 2
            })
        ));
        assert!(settle(&report(Vec::new()), true).is_err());
        assert!(lines_of(&failed)[1].starts_with("FAIL  delete_locked_version"));
    }

    #[test]
    fn a_bucket_is_described_by_its_lock() {
        let unlocked = BucketLock {
            enabled: false,
            default: None,
        };
        let locked = BucketLock {
            enabled: true,
            default: Some(ANCHOR_RETENTION),
        };
        assert_eq!(describe(unlocked), "no object lock");
        assert_eq!(describe(locked), "object lock, COMPLIANCE 10 years");
    }

    #[test]
    fn a_manifest_written_is_the_manifest_read() {
        let path =
            std::env::temp_dir().join(format!("akr-ctl-manifest-{}.json", std::process::id()));
        let entries = vec![ManifestEntry {
            bucket: "akr-conformance".to_owned(),
            key: "probe/1".to_owned(),
            version_id: "v1".to_owned(),
            sha256: "00".repeat(32),
            retention_mode: "COMPLIANCE".to_owned(),
            retain_until: chrono::DateTime::from_timestamp(1_800_000_000, 0).unwrap(),
        }];
        write_manifest(&path, &entries).unwrap();
        assert_eq!(read_manifest(&path).unwrap(), entries);
        std::fs::write(&path, b"not json").unwrap();
        assert!(read_manifest(&path).is_err());
    }
}
