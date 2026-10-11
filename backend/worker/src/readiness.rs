//! What `akr-worker` needs before it is ready: its database, a schema as new as this build, an OpenBao
//! that is unsealed and still accepts its token, and the object store behind the guard.
//!
//! Under the development key provider there is no OpenBao session, and the list carries no `openbao`
//! entry: a check names a dependency the process has.

use std::sync::Arc;

use akr_common::infra::{self, ReadinessCheck, ReadinessProbe};
use akr_crypto::BaoClient;
use akr_objectstore::{ObjectStore, buckets};
use async_trait::async_trait;
use sqlx::PgPool;

pub const OPENBAO_CHECK: &str = "openbao";
pub const OBJECT_STORE_CHECK: &str = "object_store";

pub struct WorkerProbe {
    pool: PgPool,
    bao: Option<BaoClient>,
    store: Arc<dyn ObjectStore>,
}

impl WorkerProbe {
    pub fn new(pool: PgPool, bao: Option<BaoClient>, store: Arc<dyn ObjectStore>) -> Self {
        Self { pool, bao, store }
    }
}

#[async_trait]
impl ReadinessProbe for WorkerProbe {
    async fn checks(&self) -> Vec<ReadinessCheck> {
        let openbao = async {
            match &self.bao {
                Some(bao) => Some(infra::check(OPENBAO_CHECK, usable(bao)).await),
                None => None,
            }
        };
        let (database, migrations, openbao, object_store) = tokio::join!(
            akr_db::database_check(&self.pool),
            akr_db::migrations_check(&self.pool),
            openbao,
            infra::check(
                OBJECT_STORE_CHECK,
                self.store.head_bucket(buckets::EVIDENCE)
            ),
        );
        [
            Some(database),
            Some(migrations),
            openbao,
            Some(object_store),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

/// Unsealed, and the token of this process is still accepted.
async fn usable(bao: &BaoClient) -> Result<(), String> {
    let status = bao.seal_status().await.map_err(|e| e.to_string())?;
    if !status.initialized {
        return Err("OpenBao is not initialised".to_owned());
    }
    if status.sealed {
        return Err("OpenBao is sealed".to_owned());
    }
    bao.lookup_self().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use akr_objectstore::MemoryObjectStore;

    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn the_development_worker_checks_its_database_and_its_store(pool: PgPool) {
        let probe = WorkerProbe::new(pool, None, Arc::new(MemoryObjectStore::new()));

        let checks = probe.checks().await;

        let names: Vec<&str> = checks.iter().map(|check| check.name.as_str()).collect();
        assert_eq!(names, ["database", "migrations", OBJECT_STORE_CHECK]);
        assert!(checks.iter().all(|check| check.ok));
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn a_store_without_the_evidence_bucket_is_not_ready(pool: PgPool) {
        let store = MemoryObjectStore::with_buckets(["some-other-bucket"]);
        let probe = WorkerProbe::new(pool, None, Arc::new(store));

        let checks = probe.checks().await;

        let store = checks
            .iter()
            .find(|check| check.name == OBJECT_STORE_CHECK)
            .unwrap();
        assert!(!store.ok);
    }
}
