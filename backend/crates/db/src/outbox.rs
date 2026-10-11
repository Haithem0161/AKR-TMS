//! `admin.outbox`: the transactional outbox.

use std::time::Duration;

use akr_jobs::store::{OutboxCounts, OutboxRow, OutboxStore};
use async_trait::async_trait;
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct PgOutboxStore {
    pool: PgPool,
}

impl PgOutboxStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OutboxStore for PgOutboxStore {
    async fn begin(&self) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
        self.pool.begin().await
    }

    async fn append(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        topic: &str,
        payload: &Value,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO admin.outbox (id, topic, payload) VALUES ($1, $2, $3)",
            id,
            topic,
            payload
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn claim(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        topics: &[String],
        max_attempts: i32,
    ) -> Result<Option<OutboxRow>, sqlx::Error> {
        sqlx::query_as!(
            OutboxRow,
            "SELECT id, topic, payload, created_at, available_at, attempts
             FROM admin.outbox
             WHERE dispatched_at IS NULL AND available_at <= now() AND attempts < $2 AND topic = ANY($1)
             ORDER BY available_at
             LIMIT 1
             FOR UPDATE SKIP LOCKED",
            topics,
            max_attempts
        )
        .fetch_optional(&mut **tx)
        .await
    }

    async fn mark_dispatched(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE admin.outbox SET dispatched_at = now() WHERE id = $1",
            id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn defer(&self, id: Uuid, error: &str, backoff: Duration) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE admin.outbox
             SET attempts = attempts + 1, last_error = $2, available_at = now() + make_interval(secs => $3)
             WHERE id = $1 AND dispatched_at IS NULL",
            id,
            error,
            backoff.as_secs_f64()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_dispatched_older_than(
        &self,
        days: i64,
        batch: i64,
    ) -> Result<u64, sqlx::Error> {
        let deleted = sqlx::query!(
            "DELETE FROM admin.outbox
             WHERE id IN (
                 SELECT id FROM admin.outbox
                 WHERE dispatched_at IS NOT NULL AND dispatched_at < now() - $1::bigint * interval '1 day'
                 ORDER BY dispatched_at
                 LIMIT $2
                 FOR UPDATE SKIP LOCKED)",
            days,
            batch
        )
        .execute(&self.pool)
        .await?;
        Ok(deleted.rows_affected())
    }

    async fn counts(&self, max_attempts: i32) -> Result<OutboxCounts, sqlx::Error> {
        sqlx::query_as!(
            OutboxCounts,
            r#"SELECT count(*) FILTER (WHERE attempts < $1) AS "pending!",
                      count(*) FILTER (WHERE attempts >= $1) AS "stuck!"
               FROM admin.outbox
               WHERE dispatched_at IS NULL"#,
            max_attempts
        )
        .fetch_one(&self.pool)
        .await
    }
}
