//! `backup_check`: measures how old the last backup and the last restore drill are.
//!
//! The backup tool touches a marker file after a pass that succeeded. This job reads the marker's age into
//! `akr_backup_age_seconds`, and the age of the last recorded restore drill into
//! `akr_restore_drill_age_days`. A stale or missing marker is a finding, not a failure of the job: the job
//! succeeds, the gauge and an error log carry the finding, and the alert rules act on the gauge. A marker
//! that cannot be read and a drill that was never recorded are reported as infinitely old.

use std::path::Path;
use std::time::{Duration, SystemTime};

use akr_common::settings::SettingsService;
use akr_jobs::{JobContext, JobError, JobHandler};
use akr_telemetry::metrics::{BACKUP_AGE_SECONDS, RESTORE_DRILL_AGE_DAYS};
use chrono::{DateTime, Utc};
use serde_json::Value;

const SECONDS_PER_HOUR: u64 = 3_600;
const SECONDS_PER_DAY: f64 = 86_400.0;

pub struct BackupCheck {
    pub settings: SettingsService,
}

/// What one run found.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BackupFinding {
    /// `None`: the marker cannot be read.
    pub backup_age: Option<Duration>,
    pub backup_stale: bool,
    /// `None`: no restore drill was ever recorded.
    pub drill_age_days: Option<f64>,
    pub drill_stale: bool,
}

impl BackupCheck {
    pub async fn check(&self, now: SystemTime) -> BackupFinding {
        let path = self.settings.ops_backup_marker_path().await;
        let max_age_h = self.settings.ops_backup_max_age_h().await;
        let backup_age = marker_age(Path::new(&path), now).await;
        let backup_stale = is_stale(
            backup_age.map(|age| age.as_secs_f64()),
            hours(max_age_h).as_secs_f64(),
        );

        let last_drill = self.settings.ops_last_restore_drill_at().await;
        let max_drill_days = self.settings.ops_restore_drill_max_age_days().await;
        let drill_age_days = last_drill.map(|at| days_since(at, now.into()));
        let drill_stale = is_stale(drill_age_days, max_drill_days as f64);

        metrics::gauge!(BACKUP_AGE_SECONDS)
            .set(backup_age.map_or(f64::INFINITY, |age| age.as_secs_f64()));
        metrics::gauge!(RESTORE_DRILL_AGE_DAYS).set(drill_age_days.unwrap_or(f64::INFINITY));

        match backup_age {
            None => tracing::error!(marker = %path, "the backup marker cannot be read"),
            Some(age) if backup_stale => tracing::error!(
                marker = %path,
                age_h = age.as_secs() / SECONDS_PER_HOUR,
                max_age_h,
                "the last backup is older than allowed"
            ),
            Some(_) => {}
        }
        match drill_age_days {
            None => tracing::error!("no restore drill was ever recorded"),
            Some(age_days) if drill_stale => tracing::error!(
                age_days = age_days.floor(),
                max_age_days = max_drill_days,
                "the last restore drill is older than allowed"
            ),
            Some(_) => {}
        }

        BackupFinding {
            backup_age,
            backup_stale,
            drill_age_days,
            drill_stale,
        }
    }
}

impl JobHandler for BackupCheck {
    const KIND: &'static str = "backup_check";

    async fn run(&self, _ctx: JobContext, _payload: Value) -> Result<(), JobError> {
        self.check(SystemTime::now()).await;
        Ok(())
    }
}

fn hours(n: i64) -> Duration {
    Duration::from_secs(u64::try_from(n).unwrap_or(0) * SECONDS_PER_HOUR)
}

/// An age nobody could measure is stale.
fn is_stale(age: Option<f64>, max: f64) -> bool {
    age.is_none_or(|age| age > max)
}

/// The time since the marker file was last touched; `None` when there is no such file or it cannot be
/// read. A marker dated after `now` (a clock that was set back) counts as just touched.
pub async fn marker_age(path: &Path, now: SystemTime) -> Option<Duration> {
    let meta = tokio::fs::metadata(path).await.ok()?;
    if !meta.is_file() {
        return None;
    }
    let touched = meta.modified().ok()?;
    Some(now.duration_since(touched).unwrap_or_default())
}

fn days_since(at: DateTime<Utc>, now: DateTime<Utc>) -> f64 {
    let seconds = (now - at).num_seconds().max(0);
    seconds as f64 / SECONDS_PER_DAY
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("akr-backup-check-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir.join(name)
    }

    #[tokio::test]
    async fn a_missing_marker_has_no_age() {
        let age = marker_age(
            Path::new("/nonexistent/akr/backup/heartbeat"),
            SystemTime::now(),
        )
        .await;
        assert_eq!(age, None);
    }

    #[tokio::test]
    async fn a_directory_in_place_of_the_marker_has_no_age() {
        let dir = scratch("a-directory");
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(marker_age(&dir, SystemTime::now()).await, None);
    }

    #[tokio::test]
    async fn a_touched_marker_ages_with_the_clock() {
        let marker = scratch("heartbeat");
        std::fs::write(&marker, b"").unwrap();
        let touched = std::fs::metadata(&marker).unwrap().modified().unwrap();

        let later = touched + Duration::from_secs(3 * SECONDS_PER_HOUR);
        assert_eq!(
            marker_age(&marker, later).await,
            Some(Duration::from_secs(3 * SECONDS_PER_HOUR))
        );

        let earlier = touched - Duration::from_secs(60);
        assert_eq!(marker_age(&marker, earlier).await, Some(Duration::ZERO));
    }

    #[test]
    fn an_age_past_the_limit_or_unknown_is_stale() {
        assert!(!is_stale(Some(10.0), 10.0));
        assert!(is_stale(Some(10.1), 10.0));
        assert!(is_stale(None, 10.0));
        assert_eq!(hours(26), Duration::from_secs(26 * SECONDS_PER_HOUR));
        assert_eq!(hours(-1), Duration::ZERO);
    }

    #[test]
    fn a_drill_ages_in_days_and_never_below_zero() {
        let now = DateTime::parse_from_rfc3339("2026-10-11T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let ten_days_ago = now - chrono::Duration::days(10);
        assert!((days_since(ten_days_ago, now) - 10.0).abs() < f64::EPSILON);
        assert!(days_since(now + chrono::Duration::days(1), now).abs() < f64::EPSILON);
    }
}
