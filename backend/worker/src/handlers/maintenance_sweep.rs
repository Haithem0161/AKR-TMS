//! `maintenance_sweep`: deletes the operational rows that are past their retention, and makes sure the
//! data keys of this month and the next exist.
//!
//! The rows are harness state, not business data, so their deletion is counted in metrics and writes no
//! audit event (DR-413).

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use akr_common::idempotency::IdempotencyService;
use akr_common::settings::SettingsService;
use akr_crypto::OpenBaoCrypto;
use akr_jobs::{JobContext, JobError, JobHandler, JobStore, OutboxStore, TimerStore};
use akr_telemetry::metrics::SWEEP_DELETED_TOTAL;
use chrono::Utc;
use serde_json::Value;

/// How many rows one statement deletes.
pub const BATCH: i64 = 5_000;
/// The rest between two full batches.
const PAUSE: Duration = Duration::from_millis(200);

pub const TABLE_JOBS: &str = "jobs";
pub const TABLE_OUTBOX: &str = "outbox";
pub const TABLE_IDEMPOTENCY_KEYS: &str = "idempotency_keys";
pub const TABLE_TIMERS: &str = "timers";

pub struct MaintenanceSweep {
    pub jobs: Arc<dyn JobStore>,
    pub outbox: Arc<dyn OutboxStore>,
    pub timers: Arc<dyn TimerStore>,
    pub idempotency: IdempotencyService,
    pub settings: SettingsService,
    /// `None` under the development key provider, which derives its keys and stores none.
    pub keys: Option<Arc<OpenBaoCrypto>>,
}

/// What one run deleted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Swept {
    pub jobs: u64,
    pub outbox: u64,
    pub idempotency_keys: u64,
    pub timers: u64,
}

impl MaintenanceSweep {
    pub async fn sweep(&self) -> Result<Swept, JobError> {
        let days = self.settings.jobs_done_retention_days().await;
        let jobs = drain(TABLE_JOBS, || {
            self.jobs.delete_finished_older_than(days, BATCH)
        })
        .await?;

        let days = self.settings.outbox_retention_days().await;
        let outbox = drain(TABLE_OUTBOX, || {
            self.outbox.delete_dispatched_older_than(days, BATCH)
        })
        .await?;

        let idempotency_keys = self
            .idempotency
            .sweep()
            .await
            .map_err(JobError::retryable)?;
        count(TABLE_IDEMPOTENCY_KEYS, idempotency_keys);

        let days = self.settings.timers_terminal_retention_days().await;
        let timers = drain(TABLE_TIMERS, || {
            self.timers.delete_terminal_older_than(days, BATCH)
        })
        .await?;

        Ok(Swept {
            jobs,
            outbox,
            idempotency_keys,
            timers,
        })
    }
}

impl JobHandler for MaintenanceSweep {
    const KIND: &'static str = "maintenance_sweep";

    async fn run(&self, _ctx: JobContext, _payload: Value) -> Result<(), JobError> {
        let swept = self.sweep().await?;
        if let Some(keys) = &self.keys {
            keys.ensure_keys(Utc::now())
                .await
                .map_err(JobError::retryable)?;
        }
        tracing::info!(
            jobs = swept.jobs,
            outbox = swept.outbox,
            idempotency_keys = swept.idempotency_keys,
            timers = swept.timers,
            "maintenance sweep finished"
        );
        Ok(())
    }
}

fn count(table: &'static str, deleted: u64) {
    metrics::counter!(SWEEP_DELETED_TOTAL, "table" => table).increment(deleted);
}

/// Runs `delete_batch` until a batch comes back short, resting between full ones.
async fn drain<F, Fut>(table: &'static str, mut delete_batch: F) -> Result<u64, JobError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<u64, sqlx::Error>>,
{
    let mut total = 0;
    loop {
        let deleted = delete_batch().await?;
        count(table, deleted);
        total += deleted;
        if deleted < BATCH as u64 {
            return Ok(total);
        }
        tokio::time::sleep(PAUSE).await;
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    #[tokio::test(start_paused = true)]
    async fn draining_stops_at_the_first_short_batch() {
        let left = AtomicU64::new(BATCH as u64 * 2 + 17);
        let calls = AtomicU64::new(0);

        let total = drain("test", || async {
            calls.fetch_add(1, Ordering::SeqCst);
            let batch = left.load(Ordering::SeqCst).min(BATCH as u64);
            left.fetch_sub(batch, Ordering::SeqCst);
            Ok(batch)
        })
        .await
        .unwrap();

        assert_eq!(total, BATCH as u64 * 2 + 17);
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn a_batch_that_is_exactly_full_asks_once_more() {
        let left = AtomicU64::new(BATCH as u64);
        let calls = AtomicU64::new(0);

        let total = drain("test", || async {
            calls.fetch_add(1, Ordering::SeqCst);
            let batch = left.load(Ordering::SeqCst).min(BATCH as u64);
            left.fetch_sub(batch, Ordering::SeqCst);
            Ok(batch)
        })
        .await
        .unwrap();

        assert_eq!(total, BATCH as u64);
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn a_database_error_is_worth_another_attempt() {
        let failed = drain("test", || async { Err(sqlx::Error::PoolTimedOut) }).await;
        assert!(matches!(failed, Err(JobError::Retryable(_))));
    }
}
