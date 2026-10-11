//! `admin.retention_assignment`.

use akr_domain::DomainError;
use akr_domain::repositories::RetentionAssignmentRepository;
use akr_domain::retention::{RetentionAssignment, RetentionClass, RetentionClock};
use async_trait::async_trait;
use sqlx::PgPool;

use crate::schema::{MIGRATION_LEDGER, akr_schemas};

#[derive(Clone)]
pub struct PgRetentionAssignmentRepository {
    pool: PgPool,
}

impl PgRetentionAssignmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RetentionAssignmentRepository for PgRetentionAssignmentRepository {
    async fn list(&self) -> Result<Vec<RetentionAssignment>, DomainError> {
        let rows = sqlx::query_as!(
            RetentionAssignment,
            r#"SELECT schema_name, table_name, retention_class AS "retention_class: RetentionClass",
                      clock AS "clock: RetentionClock", clock_column, note
               FROM admin.retention_assignment
               ORDER BY schema_name, table_name"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn uncovered_tables(&self) -> Result<Vec<(String, String)>, DomainError> {
        // A partition ages with its parent, and the parent carries the row.
        let rows = sqlx::query!(
            r#"SELECT n.nspname::text AS "schema_name!", c.relname::text AS "table_name!"
               FROM pg_catalog.pg_class c
               JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
               WHERE n.nspname = ANY($1::text[])
                 AND c.relkind IN ('r', 'p')
                 AND NOT c.relispartition
                 AND c.relname <> $2::text
                 AND NOT EXISTS (
                     SELECT 1 FROM admin.retention_assignment ra
                     WHERE ra.schema_name = n.nspname AND ra.table_name = c.relname)
               ORDER BY 1, 2"#,
            &akr_schemas(),
            MIGRATION_LEDGER
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.schema_name, r.table_name))
            .collect())
    }
}
