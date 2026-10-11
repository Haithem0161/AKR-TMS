//! What `akr-assess` needs before it is ready: its database. Phase 29 adds the decision key, the list
//! indexes and the policy snapshot. Valkey is not a condition (DR-325).

use akr_common::infra::{ReadinessCheck, ReadinessProbe};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct AssessProbe {
    pool: PgPool,
}

impl AssessProbe {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReadinessProbe for AssessProbe {
    async fn checks(&self) -> Vec<ReadinessCheck> {
        vec![akr_db::database_check(&self.pool).await]
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn the_database_is_the_only_condition(pool: PgPool) {
        let checks = AssessProbe::new(pool).checks().await;

        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0].name, "database");
        assert!(checks[0].ok);
    }
}
