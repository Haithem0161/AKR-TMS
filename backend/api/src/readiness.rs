//! What `akr-api` needs before it takes requests: its database, a schema as new as this build, and Valkey.

use akr_common::infra::{self, ReadinessCheck, ReadinessProbe};
use async_trait::async_trait;
use fred::clients::Pool as ValkeyPool;
use sqlx::PgPool;

use crate::valkey;

pub const VALKEY_CHECK: &str = "valkey";

pub struct ApiProbe {
    pool: PgPool,
    valkey: ValkeyPool,
}

impl ApiProbe {
    pub fn new(pool: PgPool, valkey: ValkeyPool) -> Self {
        Self { pool, valkey }
    }
}

#[async_trait]
impl ReadinessProbe for ApiProbe {
    async fn checks(&self) -> Vec<ReadinessCheck> {
        let (database, migrations, valkey) = tokio::join!(
            akr_db::database_check(&self.pool),
            akr_db::migrations_check(&self.pool),
            infra::check(VALKEY_CHECK, valkey::ping(&self.valkey)),
        );
        vec![database, migrations, valkey]
    }
}
