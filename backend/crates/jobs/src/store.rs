//! The storage ports of the harness, and the rows they return. The PostgreSQL adapters are in `akr-db`.

use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{FromRow, Postgres, Transaction};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobStatus {
    Queued,
    Running,
    Done,
    Failed,
    Dead,
}

/// One row of `admin.jobs`.
#[derive(Debug, Clone, FromRow)]
pub struct Job {
    pub id: Uuid,
    pub kind: String,
    pub payload: Value,
    pub status: JobStatus,
    pub run_at: DateTime<Utc>,
    /// The number of claims so far. A running job's `attempts` names its claim.
    pub attempts: i32,
    pub max_attempts: i32,
    pub last_error: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<String>,
    pub coalesce_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

/// A job row to insert.
#[derive(Debug, Clone)]
pub struct JobInsert<'a> {
    pub id: Uuid,
    pub kind: &'a str,
    pub payload: &'a Value,
    pub run_at: Option<DateTime<Utc>>,
    pub max_attempts: i32,
    pub coalesce_key: Option<&'a str>,
}

/// A job whose lease ran out.
#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Reaped {
    pub id: Uuid,
    pub attempts: i32,
    /// The job had used its last attempt and is dead; otherwise it is queued again.
    pub dead: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRow)]
pub struct JobCounts {
    pub queued: i64,
    pub running: i64,
}

#[async_trait]
pub trait JobStore: Send + Sync {
    async fn begin(&self) -> Result<Transaction<'static, Postgres>, sqlx::Error>;

    /// Inserts a queued job and notifies `akr_jobs`. `None` when a queued job with the same kind and
    /// coalesce key already exists.
    async fn enqueue(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        job: JobInsert<'_>,
    ) -> Result<Option<Uuid>, sqlx::Error>;

    /// Serialises the schedulers of one kind for the rest of the transaction, and returns the database
    /// clock read after the wait.
    async fn lock_schedule(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        kind: &str,
    ) -> Result<DateTime<Utc>, sqlx::Error>;

    /// Inserts the job unless a row with its kind and coalesce key exists in any status. The caller holds
    /// [`JobStore::lock_schedule`].
    async fn enqueue_slot(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        job: JobInsert<'_>,
    ) -> Result<Option<Uuid>, sqlx::Error>;

    /// Claims up to `limit` due jobs of `kind`, never letting more than `concurrency` run across the
    /// installation. Each claim adds one to `attempts`.
    async fn claim(
        &self,
        kind: &str,
        concurrency: i64,
        limit: i64,
        worker_id: &str,
    ) -> Result<Vec<Job>, sqlx::Error>;

    /// Refreshes the lease of a running job. `false`: the claim is gone.
    async fn heartbeat(&self, id: Uuid, worker_id: &str) -> Result<bool, sqlx::Error>;

    /// Marks the claim `attempt` done. `false`: the claim is gone, nothing changed.
    async fn complete(&self, id: Uuid, attempt: i32) -> Result<bool, sqlx::Error>;

    /// [`JobStore::complete`] inside the handler's transaction (DR-412).
    async fn complete_in(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        attempt: i32,
    ) -> Result<bool, sqlx::Error>;

    /// Queues the claim `attempt` again after `backoff`, with its error.
    async fn fail(
        &self,
        id: Uuid,
        attempt: i32,
        error: &str,
        backoff: Duration,
    ) -> Result<bool, sqlx::Error>;

    /// Marks the claim `attempt` dead.
    async fn bury(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        attempt: i32,
        error: &str,
    ) -> Result<bool, sqlx::Error>;

    /// Takes back the running jobs of `kind` whose lease ran out: queued again, or dead at the last
    /// attempt. `locked_by` is left as it was.
    async fn reap_expired(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        kind: &str,
        lease: Duration,
    ) -> Result<Vec<Reaped>, sqlx::Error>;

    /// Deletes up to `batch` done or dead jobs that finished more than `days` days ago.
    async fn delete_finished_older_than(&self, days: i64, batch: i64) -> Result<u64, sqlx::Error>;

    async fn counts(&self) -> Result<JobCounts, sqlx::Error>;
}

/// One claimed row of `admin.outbox`.
#[derive(Debug, Clone, FromRow)]
pub struct OutboxRow {
    pub id: Uuid,
    pub topic: String,
    pub payload: Value,
    pub created_at: DateTime<Utc>,
    pub available_at: DateTime<Utc>,
    pub attempts: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRow)]
pub struct OutboxCounts {
    /// Undispatched rows that still have attempts left.
    pub pending: i64,
    /// Undispatched rows at the attempt limit: lost to their consumers until someone acts.
    pub stuck: i64,
}

#[async_trait]
pub trait OutboxStore: Send + Sync {
    async fn begin(&self) -> Result<Transaction<'static, Postgres>, sqlx::Error>;

    async fn append(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        topic: &str,
        payload: &Value,
    ) -> Result<(), sqlx::Error>;

    /// Locks and returns the oldest due row of `topics` with attempts left, skipping rows another
    /// dispatcher holds. The lock lasts until the transaction ends.
    async fn claim(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        topics: &[String],
        max_attempts: i32,
    ) -> Result<Option<OutboxRow>, sqlx::Error>;

    async fn mark_dispatched(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<(), sqlx::Error>;

    /// Counts a failed attempt and makes the row due again after `backoff`.
    async fn defer(&self, id: Uuid, error: &str, backoff: Duration) -> Result<(), sqlx::Error>;

    /// Deletes up to `batch` rows dispatched more than `days` days ago. Never an undispatched row.
    async fn delete_dispatched_older_than(&self, days: i64, batch: i64)
    -> Result<u64, sqlx::Error>;

    async fn counts(&self, max_attempts: i32) -> Result<OutboxCounts, sqlx::Error>;
}

/// One row of `admin.timers`.
#[derive(Debug, Clone, FromRow)]
pub struct Timer {
    pub id: Uuid,
    pub owner_kind: String,
    pub owner_id: Uuid,
    pub kind: String,
    pub fire_at: DateTime<Utc>,
    pub calendar_id: Option<Uuid>,
    pub payload: Value,
    pub created_at: DateTime<Utc>,
    pub fired_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct TimerInsert<'a> {
    pub id: Uuid,
    pub owner_kind: &'a str,
    pub owner_id: Uuid,
    pub kind: &'a str,
    pub fire_at: DateTime<Utc>,
    pub calendar_id: Option<Uuid>,
    pub payload: &'a Value,
}

#[async_trait]
pub trait TimerStore: Send + Sync {
    async fn begin(&self) -> Result<Transaction<'static, Postgres>, sqlx::Error>;

    async fn schedule(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        timer: TimerInsert<'_>,
    ) -> Result<(), sqlx::Error>;

    /// Cancels the timer if it is still pending; returns the number of rows changed (0 or 1).
    async fn cancel(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<u64, sqlx::Error>;

    /// Cancels the pending timers of one kind of one owner; returns the number of rows changed.
    async fn cancel_for_owner(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        owner_kind: &str,
        owner_id: Uuid,
        kind: &str,
    ) -> Result<u64, sqlx::Error>;

    /// Marks up to `limit` due pending timers of `kinds` fired and returns them, skipping rows another
    /// sweep holds.
    async fn claim_due(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        kinds: &[String],
        limit: i64,
    ) -> Result<Vec<Timer>, sqlx::Error>;

    /// Deletes up to `batch` timers fired or cancelled more than `days` days ago. Never a pending one.
    async fn delete_terminal_older_than(&self, days: i64, batch: i64) -> Result<u64, sqlx::Error>;

    /// How many pending timers are due now.
    async fn due_count(&self) -> Result<i64, sqlx::Error>;
}
