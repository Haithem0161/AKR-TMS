//! `admin.timers`: deadlines that fire a job.

use akr_jobs::store::{Timer, TimerInsert, TimerStore};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct PgTimerStore {
    pool: PgPool,
}

impl PgTimerStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TimerStore for PgTimerStore {
    async fn begin(&self) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
        self.pool.begin().await
    }

    async fn schedule(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        timer: TimerInsert<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO admin.timers (id, owner_kind, owner_id, kind, fire_at, calendar_id, payload)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            timer.id,
            timer.owner_kind,
            timer.owner_id,
            timer.kind,
            timer.fire_at,
            timer.calendar_id,
            timer.payload
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn cancel(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let cancelled = sqlx::query!(
            "UPDATE admin.timers SET cancelled_at = now()
             WHERE id = $1 AND fired_at IS NULL AND cancelled_at IS NULL",
            id
        )
        .execute(&mut **tx)
        .await?;
        Ok(cancelled.rows_affected())
    }

    async fn cancel_for_owner(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        owner_kind: &str,
        owner_id: Uuid,
        kind: &str,
    ) -> Result<u64, sqlx::Error> {
        let cancelled = sqlx::query!(
            "UPDATE admin.timers SET cancelled_at = now()
             WHERE owner_kind = $1 AND owner_id = $2 AND kind = $3
               AND fired_at IS NULL AND cancelled_at IS NULL",
            owner_kind,
            owner_id,
            kind
        )
        .execute(&mut **tx)
        .await?;
        Ok(cancelled.rows_affected())
    }

    async fn claim_due(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        kinds: &[String],
        limit: i64,
    ) -> Result<Vec<Timer>, sqlx::Error> {
        sqlx::query_as!(
            Timer,
            "UPDATE admin.timers SET fired_at = now()
             WHERE id IN (
                 SELECT id FROM admin.timers
                 WHERE fired_at IS NULL AND cancelled_at IS NULL AND fire_at <= now() AND kind = ANY($1)
                 ORDER BY fire_at
                 LIMIT $2
                 FOR UPDATE SKIP LOCKED)
             RETURNING id, owner_kind, owner_id, kind, fire_at, calendar_id, payload, created_at, fired_at,
                       cancelled_at",
            kinds,
            limit
        )
        .fetch_all(&mut **tx)
        .await
    }

    async fn delete_terminal_older_than(&self, days: i64, batch: i64) -> Result<u64, sqlx::Error> {
        let deleted = sqlx::query!(
            "DELETE FROM admin.timers
             WHERE id IN (
                 SELECT id FROM admin.timers
                 WHERE fired_at < now() - $1::bigint * interval '1 day'
                    OR cancelled_at < now() - $1::bigint * interval '1 day'
                 LIMIT $2
                 FOR UPDATE SKIP LOCKED)",
            days,
            batch
        )
        .execute(&self.pool)
        .await?;
        Ok(deleted.rows_affected())
    }

    async fn due_count(&self) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar!(
            r#"SELECT count(*) AS "due!" FROM admin.timers
               WHERE fired_at IS NULL AND cancelled_at IS NULL AND fire_at <= now()"#
        )
        .fetch_one(&self.pool)
        .await
    }
}
