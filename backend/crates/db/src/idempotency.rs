//! `admin.idempotency_keys`: the ledger behind `Idempotency-Key` (phase 00, 7.17).

use std::time::Duration;

use akr_common::idempotency::{ClaimError, IdempotencyStore, LedgerRow};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

/// SQLSTATE `lock_not_available`: the wait for another transaction's row passed `lock_timeout`.
const LOCK_NOT_AVAILABLE: &str = "55P03";
/// SQLSTATE `query_canceled`: the role's `statement_timeout` ended the wait before `lock_timeout` did.
const QUERY_CANCELED: &str = "57014";

#[derive(Clone)]
pub struct PgIdempotencyStore {
    pool: PgPool,
}

impl PgIdempotencyStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn claim_error(error: sqlx::Error) -> ClaimError {
    let timed_out = error
        .as_database_error()
        .and_then(|e| e.code())
        .is_some_and(|code| code == LOCK_NOT_AVAILABLE || code == QUERY_CANCELED);
    if timed_out {
        ClaimError::InFlight
    } else {
        ClaimError::Database(error)
    }
}

#[async_trait]
impl IdempotencyStore for PgIdempotencyStore {
    async fn claim(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        principal_id: Uuid,
        key: &str,
        endpoint: &str,
        request_hash: &[u8; 32],
        lease: Duration,
    ) -> Result<bool, ClaimError> {
        let timeout = format!("{}ms", lease.as_millis().max(1));
        sqlx::query!("SELECT set_config('lock_timeout', $1, true)", timeout)
            .fetch_one(&mut **tx)
            .await
            .map_err(ClaimError::Database)?;
        let inserted = sqlx::query!(
            "INSERT INTO admin.idempotency_keys (principal_id, key, endpoint, request_hash)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (principal_id, key) DO NOTHING",
            principal_id,
            key,
            endpoint,
            request_hash.as_slice()
        )
        .execute(&mut **tx)
        .await
        .map_err(claim_error)?;
        Ok(inserted.rows_affected() == 1)
    }

    async fn find(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        principal_id: Uuid,
        key: &str,
    ) -> Result<Option<LedgerRow>, sqlx::Error> {
        sqlx::query_as!(
            LedgerRow,
            "SELECT request_hash, response_status, response_content_type, response_body
             FROM admin.idempotency_keys
             WHERE principal_id = $1 AND key = $2",
            principal_id,
            key
        )
        .fetch_optional(&mut **tx)
        .await
    }

    async fn complete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        principal_id: Uuid,
        key: &str,
        status: i32,
        content_type: Option<&str>,
        sealed_body: Option<&[u8]>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE admin.idempotency_keys
             SET response_status = $3, response_content_type = $4, response_body = $5, completed_at = now()
             WHERE principal_id = $1 AND key = $2",
            principal_id,
            key,
            status,
            content_type,
            sealed_body
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn delete_older_than(&self, days: i64, batch: i64) -> Result<u64, sqlx::Error> {
        let deleted = sqlx::query!(
            "DELETE FROM admin.idempotency_keys
             WHERE (principal_id, key) IN (
                 SELECT principal_id, key FROM admin.idempotency_keys
                 WHERE created_at < now() - $1::bigint * interval '1 day'
                 ORDER BY created_at
                 LIMIT $2
                 FOR UPDATE SKIP LOCKED)",
            days,
            batch
        )
        .execute(&self.pool)
        .await?;
        Ok(deleted.rows_affected())
    }
}
