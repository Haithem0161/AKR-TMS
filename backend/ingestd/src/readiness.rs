//! What `akr-ingestd` needs before it is ready: its database and a schema as new as this build. Phase 05
//! adds `source_listener` and `adapters`.

use akr_common::infra::{ReadinessCheck, ReadinessProbe};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct IngestdProbe {
    pool: PgPool,
}

impl IngestdProbe {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReadinessProbe for IngestdProbe {
    async fn checks(&self) -> Vec<ReadinessCheck> {
        let (database, migrations) = tokio::join!(
            akr_db::database_check(&self.pool),
            akr_db::migrations_check(&self.pool),
        );
        vec![database, migrations]
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn a_migrated_database_is_ready(pool: PgPool) {
        let checks = IngestdProbe::new(pool).checks().await;

        let names: Vec<&str> = checks.iter().map(|check| check.name.as_str()).collect();
        assert_eq!(names, ["database", "migrations"]);
        assert!(checks.iter().all(|check| check.ok));
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn a_schema_older_than_the_build_is_not_ready(pool: PgPool) {
        sqlx::query(
            "DELETE FROM _sqlx_migrations
              WHERE version = (SELECT max(version) FROM _sqlx_migrations)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let checks = IngestdProbe::new(pool).checks().await;

        let migrations = checks.iter().find(|check| check.name == "migrations");
        assert!(migrations.is_some_and(|check| !check.ok));
    }
}
