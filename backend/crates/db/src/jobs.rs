//! `admin.jobs`: the queue behind the worker harness.
//!
//! Every write the harness makes on a claimed job tests `status = 'running' AND attempts = $attempt`: a
//! claim adds one to `attempts`, so the pair names one claim, and a worker whose claim was taken back
//! changes nothing (phase 00, 7.18).

use std::sync::Arc;
use std::time::Duration;

use akr_jobs::store::{Job, JobCounts, JobInsert, JobStatus, JobStore, Reaped};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgListener;
use sqlx::{PgPool, Postgres, Transaction};
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

/// The channel an enqueue notifies. A notification is a hint to poll now; the poll loop finds the job
/// without it.
pub const JOBS_CHANNEL: &str = "akr_jobs";

const LISTEN_RETRY: Duration = Duration::from_secs(5);

#[derive(Clone)]
pub struct PgJobStore {
    pool: PgPool,
}

impl PgJobStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobStore for PgJobStore {
    async fn begin(&self) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
        self.pool.begin().await
    }

    async fn enqueue(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        job: JobInsert<'_>,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let id = sqlx::query_scalar!(
            "INSERT INTO admin.jobs (id, kind, payload, run_at, max_attempts, coalesce_key)
             VALUES ($1, $2, $3, COALESCE($4, now()), $5, $6)
             ON CONFLICT (kind, coalesce_key) WHERE status = 'queued' AND coalesce_key IS NOT NULL
             DO NOTHING
             RETURNING id",
            job.id,
            job.kind,
            job.payload,
            job.run_at,
            job.max_attempts,
            job.coalesce_key
        )
        .fetch_optional(&mut **tx)
        .await?;
        if id.is_some() {
            sqlx::query!("SELECT pg_notify($1, $2)", JOBS_CHANNEL, job.kind)
                .fetch_one(&mut **tx)
                .await?;
        }
        Ok(id)
    }

    async fn lock_schedule(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        kind: &str,
    ) -> Result<DateTime<Utc>, sqlx::Error> {
        sqlx::query!(
            "SELECT pg_advisory_xact_lock(hashtext('schedule:' || $1))",
            kind
        )
        .fetch_one(&mut **tx)
        .await?;
        sqlx::query_scalar!(r#"SELECT clock_timestamp() AS "now!""#)
            .fetch_one(&mut **tx)
            .await
    }

    async fn enqueue_slot(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        job: JobInsert<'_>,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        sqlx::query_scalar!(
            "INSERT INTO admin.jobs (id, kind, payload, run_at, max_attempts, coalesce_key)
             SELECT $1::uuid, $2::text, $3::jsonb, COALESCE($4::timestamptz, now()), $5::int, $6::text
             WHERE NOT EXISTS (
                 SELECT 1 FROM admin.jobs WHERE kind = $2::text AND coalesce_key = $6::text)
             ON CONFLICT (kind, coalesce_key) WHERE status = 'queued' AND coalesce_key IS NOT NULL
             DO NOTHING
             RETURNING id",
            job.id,
            job.kind,
            job.payload,
            job.run_at,
            job.max_attempts,
            job.coalesce_key
        )
        .fetch_optional(&mut **tx)
        .await
    }

    async fn claim(
        &self,
        kind: &str,
        concurrency: i64,
        limit: i64,
        worker_id: &str,
    ) -> Result<Vec<Job>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!("SELECT pg_advisory_xact_lock(hashtext($1))", kind)
            .fetch_one(&mut *tx)
            .await?;
        let running = sqlx::query_scalar!(
            r#"SELECT count(*) AS "running!" FROM admin.jobs WHERE kind = $1 AND status = 'running'"#,
            kind
        )
        .fetch_one(&mut *tx)
        .await?;
        let free = (concurrency - running).min(limit);
        if free <= 0 {
            tx.commit().await?;
            return Ok(Vec::new());
        }
        let jobs = sqlx::query_as!(
            Job,
            r#"UPDATE admin.jobs
               SET status = 'running', locked_at = now(), locked_by = $2, attempts = attempts + 1
               WHERE id IN (
                   SELECT id FROM admin.jobs
                   WHERE kind = $1 AND status = 'queued' AND run_at <= now()
                   ORDER BY run_at
                   LIMIT $3
                   FOR UPDATE SKIP LOCKED)
               RETURNING id, kind, payload, status AS "status: JobStatus", run_at, attempts, max_attempts,
                         last_error, locked_at, locked_by, coalesce_key, created_at, finished_at"#,
            kind,
            worker_id,
            free
        )
        .fetch_all(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(jobs)
    }

    async fn heartbeat(&self, id: Uuid, worker_id: &str) -> Result<bool, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE admin.jobs SET locked_at = now()
             WHERE id = $1 AND status = 'running' AND locked_by = $2",
            id,
            worker_id
        )
        .execute(&self.pool)
        .await?;
        Ok(updated.rows_affected() == 1)
    }

    async fn complete(&self, id: Uuid, attempt: i32) -> Result<bool, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE admin.jobs SET status = 'done', finished_at = now()
             WHERE id = $1 AND status = 'running' AND attempts = $2",
            id,
            attempt
        )
        .execute(&self.pool)
        .await?;
        Ok(updated.rows_affected() == 1)
    }

    async fn complete_in(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        attempt: i32,
    ) -> Result<bool, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE admin.jobs SET status = 'done', finished_at = now()
             WHERE id = $1 AND status = 'running' AND attempts = $2",
            id,
            attempt
        )
        .execute(&mut **tx)
        .await?;
        Ok(updated.rows_affected() == 1)
    }

    async fn fail(
        &self,
        id: Uuid,
        attempt: i32,
        error: &str,
        backoff: Duration,
    ) -> Result<bool, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE admin.jobs
             SET status = 'queued', run_at = now() + make_interval(secs => $4), last_error = $3
             WHERE id = $1 AND status = 'running' AND attempts = $2",
            id,
            attempt,
            error,
            backoff.as_secs_f64()
        )
        .execute(&self.pool)
        .await?;
        Ok(updated.rows_affected() == 1)
    }

    async fn bury(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        attempt: i32,
        error: &str,
    ) -> Result<bool, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE admin.jobs SET status = 'dead', finished_at = now(), last_error = $3
             WHERE id = $1 AND status = 'running' AND attempts = $2",
            id,
            attempt,
            error
        )
        .execute(&mut **tx)
        .await?;
        Ok(updated.rows_affected() == 1)
    }

    async fn reap_expired(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        kind: &str,
        lease: Duration,
    ) -> Result<Vec<Reaped>, sqlx::Error> {
        sqlx::query_as!(
            Reaped,
            r#"UPDATE admin.jobs
               SET status = CASE WHEN attempts >= max_attempts THEN 'dead'::admin.job_status
                                 ELSE 'queued'::admin.job_status END,
                   finished_at = CASE WHEN attempts >= max_attempts THEN now() END,
                   last_error = 'lease expired'
               WHERE id IN (
                   SELECT id FROM admin.jobs
                   WHERE kind = $1 AND status = 'running'
                     AND locked_at < now() - make_interval(secs => $2)
                   FOR UPDATE SKIP LOCKED)
               RETURNING id, attempts, (status = 'dead') AS "dead!""#,
            kind,
            lease.as_secs_f64()
        )
        .fetch_all(&mut **tx)
        .await
    }

    async fn delete_finished_older_than(&self, days: i64, batch: i64) -> Result<u64, sqlx::Error> {
        let deleted = sqlx::query!(
            "DELETE FROM admin.jobs
             WHERE id IN (
                 SELECT id FROM admin.jobs
                 WHERE status IN ('done', 'dead') AND finished_at < now() - $1::bigint * interval '1 day'
                 ORDER BY finished_at
                 LIMIT $2
                 FOR UPDATE SKIP LOCKED)",
            days,
            batch
        )
        .execute(&self.pool)
        .await?;
        Ok(deleted.rows_affected())
    }

    async fn counts(&self) -> Result<JobCounts, sqlx::Error> {
        sqlx::query_as!(
            JobCounts,
            r#"SELECT count(*) FILTER (WHERE status = 'queued') AS "queued!",
                      count(*) FILTER (WHERE status = 'running') AS "running!"
               FROM admin.jobs"#
        )
        .fetch_one(&self.pool)
        .await
    }
}

/// Wakes the poll loop when a job is enqueued. It holds one connection for `LISTEN`; when that connection
/// is lost it reconnects, and until then the loop's own interval finds the jobs.
pub async fn listen_for_jobs(pool: PgPool, wake: Arc<Notify>, shutdown: CancellationToken) {
    loop {
        let listening = async {
            let mut listener = PgListener::connect_with(&pool).await?;
            listener.listen(JOBS_CHANNEL).await?;
            loop {
                listener.recv().await?;
                wake.notify_one();
            }
        };
        let outcome: Result<(), sqlx::Error> = tokio::select! {
            () = shutdown.cancelled() => return,
            outcome = listening => outcome,
        };
        if let Err(error) = outcome {
            tracing::warn!(%error, "the job listener lost its connection");
        }
        // Jobs enqueued while nobody listened are found by a poll.
        wake.notify_one();
        tokio::select! {
            () = shutdown.cancelled() => return,
            () = tokio::time::sleep(LISTEN_RETRY) => {}
        }
    }
}
