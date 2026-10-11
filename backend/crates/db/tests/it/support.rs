use std::sync::Arc;

use akr_common::settings::{NoNotifier, SettingsService};
use akr_db::PgSettingsRepository;
use akr_domain::audit::{Actor, RecordingSink, RequestCtx};
use sqlx::PgPool;
use uuid::Uuid;

pub fn settings(pool: &PgPool, audit: Arc<RecordingSink>) -> SettingsService {
    SettingsService::new(
        Arc::new(PgSettingsRepository::new(pool.clone())),
        audit,
        Arc::new(NoNotifier),
    )
}

pub fn actor() -> Actor {
    Actor::user(Uuid::now_v7())
}

pub fn request() -> RequestCtx {
    RequestCtx::new(Uuid::now_v7())
}

pub async fn count(pool: &PgPool, sql: &str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
}

pub async fn run(pool: &PgPool, sql: &str) {
    sqlx::query(sql).execute(pool).await.unwrap();
}
