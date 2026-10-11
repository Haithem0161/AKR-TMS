//! `admin.column_classification`.

use akr_domain::DomainError;
use akr_domain::classification::{Classification, ColumnClassification};
use akr_domain::repositories::ClassificationRepository;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::schema::{MIGRATION_LEDGER, akr_schemas};

#[derive(Clone)]
pub struct PgClassificationRepository {
    pool: PgPool,
}

impl PgClassificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ClassificationRepository for PgClassificationRepository {
    async fn for_table(
        &self,
        schema: &str,
        table: &str,
    ) -> Result<Vec<ColumnClassification>, DomainError> {
        let rows = sqlx::query_as!(
            ColumnClassification,
            r#"SELECT schema_name, table_name, column_name,
                      classification AS "classification: Classification", encrypted, keyed_hash, note
               FROM admin.column_classification
               WHERE schema_name = $1 AND table_name = $2
               ORDER BY column_name"#,
            schema,
            table
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn list(&self) -> Result<Vec<ColumnClassification>, DomainError> {
        let rows = sqlx::query_as!(
            ColumnClassification,
            r#"SELECT schema_name, table_name, column_name,
                      classification AS "classification: Classification", encrypted, keyed_hash, note
               FROM admin.column_classification
               ORDER BY schema_name, table_name, column_name"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn uncovered_columns(&self) -> Result<Vec<(String, String, String)>, DomainError> {
        // A partition has the columns of its parent, and the parent carries the rows.
        let rows = sqlx::query!(
            r#"SELECT n.nspname::text AS "schema_name!", c.relname::text AS "table_name!",
                      a.attname::text AS "column_name!"
               FROM pg_catalog.pg_class c
               JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
               JOIN pg_catalog.pg_attribute a
                 ON a.attrelid = c.oid AND a.attnum > 0 AND NOT a.attisdropped
               WHERE n.nspname = ANY($1::text[])
                 AND c.relkind IN ('r', 'p')
                 AND NOT c.relispartition
                 AND c.relname <> $2::text
                 AND NOT EXISTS (
                     SELECT 1 FROM admin.column_classification cc
                     WHERE cc.schema_name = n.nspname
                       AND cc.table_name = c.relname
                       AND cc.column_name = a.attname)
               ORDER BY 1, 2, 3"#,
            &akr_schemas(),
            MIGRATION_LEDGER
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.schema_name, r.table_name, r.column_name))
            .collect())
    }
}
