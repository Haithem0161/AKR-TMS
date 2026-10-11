//! The jobs of the platform against a real database: what the sweep deletes and keeps, what the backup
//! check finds, and both kinds running through the harness.

#![allow(clippy::unwrap_used)]

use std::sync::Arc;
use std::time::{Duration, SystemTime};

use akr_audit::TracingSink;
use akr_common::idempotency::IdempotencyService;
use akr_common::settings::{NoNotifier, SettingsService};
use akr_crypto::InMemoryCrypto;
use akr_db::{PgIdempotencyStore, PgJobStore, PgOutboxStore, PgSettingsRepository, PgTimerStore};
use akr_jobs::{JobQueue, JobRegistry, NewJob, Worker, kinds};
use akr_worker::handlers::maintenance_sweep::Swept;
use akr_worker::handlers::{BackupCheck, MaintenanceSweep};
use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

const HOUR: Duration = Duration::from_secs(3_600);

fn settings(pool: &PgPool) -> SettingsService {
    SettingsService::new(
        Arc::new(PgSettingsRepository::new(pool.clone())),
        Arc::new(TracingSink::new()),
        Arc::new(NoNotifier),
    )
}

fn sweep(pool: &PgPool) -> MaintenanceSweep {
    let settings = settings(pool);
    MaintenanceSweep {
        jobs: Arc::new(PgJobStore::new(pool.clone())),
        outbox: Arc::new(PgOutboxStore::new(pool.clone())),
        timers: Arc::new(PgTimerStore::new(pool.clone())),
        idempotency: IdempotencyService::new(
            Arc::new(PgIdempotencyStore::new(pool.clone())),
            Arc::new(InMemoryCrypto::from_seed([7; 32])),
            settings.clone(),
        ),
        settings,
        keys: None,
    }
}

async fn run(pool: &PgPool, sql: &str) {
    sqlx::query(sql).execute(pool).await.unwrap();
}

async fn count(pool: &PgPool, table: &str) -> i64 {
    sqlx::query_scalar(&format!("SELECT count(*) FROM admin.{table}"))
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn set(pool: &PgPool, key: &str, value: serde_json::Value) {
    sqlx::query("UPDATE admin.settings SET value = $2 WHERE key = $1")
        .bind(key)
        .bind(value)
        .execute(pool)
        .await
        .unwrap();
}

#[sqlx::test(migrations = "../migrations")]
async fn the_sweep_deletes_what_is_past_its_retention_and_nothing_else(pool: PgPool) {
    run(
        &pool,
        "INSERT INTO admin.jobs (id, kind, status, created_at, finished_at) VALUES
           ('0190c000-0000-7000-8000-000000000001', 'test_kind', 'done', now() - interval '400 days', now() - interval '400 days'),
           ('0190c000-0000-7000-8000-000000000002', 'test_kind', 'done', now() - interval '2 days', now() - interval '1 day'),
           ('0190c000-0000-7000-8000-000000000003', 'test_kind', 'queued', now() - interval '400 days', NULL)",
    )
    .await;
    run(
        &pool,
        "INSERT INTO admin.outbox (id, topic, payload, created_at, available_at, dispatched_at, attempts) VALUES
           ('0190c000-0000-7000-8000-000000000011', 'test.ping', '{}', now() - interval '400 days', now() - interval '400 days', now() - interval '400 days', 1),
           ('0190c000-0000-7000-8000-000000000012', 'test.ping', '{}', now() - interval '400 days', now() - interval '400 days', NULL, 10)",
    )
    .await;
    run(
        &pool,
        "INSERT INTO admin.idempotency_keys
             (principal_id, key, endpoint, request_hash, response_status, created_at, completed_at) VALUES
           ('0190c000-0000-7000-8000-000000000021', 'old-key-0001', 'POST /x', decode(repeat('00', 32), 'hex'), 204, now() - interval '400 days', now() - interval '400 days'),
           ('0190c000-0000-7000-8000-000000000021', 'new-key-0001', 'POST /x', decode(repeat('00', 32), 'hex'), 204, now(), now())",
    )
    .await;
    run(
        &pool,
        "INSERT INTO admin.timers (id, owner_kind, owner_id, kind, fire_at, created_at, fired_at) VALUES
           ('0190c000-0000-7000-8000-000000000031', 'test_owner', '0190c000-0000-7000-8000-0000000000aa', 'test_due', now() - interval '400 days', now() - interval '400 days', now() - interval '400 days'),
           ('0190c000-0000-7000-8000-000000000032', 'test_owner', '0190c000-0000-7000-8000-0000000000aa', 'test_due', now() - interval '400 days', now() - interval '400 days', NULL)",
    )
    .await;
    let sweep = sweep(&pool);

    let first = sweep.sweep().await.unwrap();

    assert_eq!(
        first,
        Swept {
            jobs: 1,
            outbox: 1,
            idempotency_keys: 1,
            timers: 1
        }
    );
    assert_eq!(count(&pool, "jobs").await, 2, "a queued job is never swept");
    assert_eq!(
        count(&pool, "outbox").await,
        1,
        "an undispatched row is never swept"
    );
    assert_eq!(count(&pool, "idempotency_keys").await, 1);
    assert_eq!(
        count(&pool, "timers").await,
        1,
        "a pending timer is never swept"
    );

    assert_eq!(sweep.sweep().await.unwrap(), Swept::default());
}

#[sqlx::test(migrations = "../migrations")]
async fn the_backup_check_finds_a_fresh_a_stale_and_a_missing_marker(pool: PgPool) {
    let dir = std::env::temp_dir().join(format!("akr-worker-it-{}", Uuid::now_v7()));
    std::fs::create_dir_all(&dir).unwrap();
    let marker = dir.join("heartbeat");
    std::fs::write(&marker, b"").unwrap();
    let touched = std::fs::metadata(&marker).unwrap().modified().unwrap();
    set(
        &pool,
        "ops.backup_marker_path",
        json!(marker.to_str().unwrap()),
    )
    .await;
    let check = BackupCheck {
        settings: settings(&pool),
    };

    let fresh = check.check(touched + HOUR).await;
    assert_eq!(fresh.backup_age, Some(HOUR));
    assert!(!fresh.backup_stale);

    let stale = check.check(touched + 27 * HOUR).await;
    assert_eq!(stale.backup_age, Some(27 * HOUR));
    assert!(stale.backup_stale, "the limit is 26 hours");

    std::fs::remove_file(&marker).unwrap();
    let missing = check.check(touched + HOUR).await;
    assert_eq!(missing.backup_age, None);
    assert!(missing.backup_stale);

    std::fs::remove_dir_all(&dir).unwrap();
}

#[sqlx::test(migrations = "../migrations")]
async fn the_backup_check_ages_the_last_restore_drill(pool: PgPool) {
    let check = BackupCheck {
        settings: settings(&pool),
    };

    let never = check.check(SystemTime::now()).await;
    assert_eq!(never.drill_age_days, None);
    assert!(never.drill_stale);

    let ten_days_ago = Utc::now() - chrono::Duration::days(10);
    set(
        &pool,
        "ops.last_restore_drill_at",
        json!(ten_days_ago.to_rfc3339()),
    )
    .await;
    check.settings.invalidate().await;
    let recent = check.check(SystemTime::now()).await;
    let age = recent.drill_age_days.unwrap();
    assert!((10.0..10.1).contains(&age), "{age}");
    assert!(!recent.drill_stale, "the limit is 31 days");

    let forty_days_ago = Utc::now() - chrono::Duration::days(40);
    set(
        &pool,
        "ops.last_restore_drill_at",
        json!(forty_days_ago.to_rfc3339()),
    )
    .await;
    check.settings.invalidate().await;
    assert!(check.check(SystemTime::now()).await.drill_stale);
}

fn worker(pool: &PgPool) -> Worker {
    let mut registry = JobRegistry::new();
    registry
        .register(&kinds::MAINTENANCE_SWEEP, sweep(pool))
        .unwrap();
    registry
        .register(
            &kinds::BACKUP_CHECK,
            BackupCheck {
                settings: settings(pool),
            },
        )
        .unwrap();
    Worker::new(
        Arc::new(PgJobStore::new(pool.clone())),
        settings(pool),
        Arc::new(TracingSink::new()),
        registry,
        "worker-test",
    )
}

#[sqlx::test(migrations = "../migrations")]
async fn both_jobs_run_to_done_through_the_harness(pool: PgPool) {
    let queue = JobQueue::new(Arc::new(PgJobStore::new(pool.clone())));
    let mut tx = pool.begin().await.unwrap();
    for kind in kinds::PHASE_00 {
        queue.enqueue(&mut tx, NewJob::new(kind)).await.unwrap();
    }
    tx.commit().await.unwrap();
    let worker = worker(&pool);

    assert_eq!(worker.poll_once().await, 2);
    worker.drain().await;

    let done: Vec<String> =
        sqlx::query_scalar("SELECT kind FROM admin.jobs WHERE status = 'done' ORDER BY kind")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(done, ["backup_check", "maintenance_sweep"]);
}

#[sqlx::test(migrations = "../migrations")]
async fn the_scheduler_enqueues_one_slot_of_each_periodic_job(pool: PgPool) {
    let worker = worker(&pool);

    assert_eq!(worker.schedule_once().await, 2);
    assert_eq!(worker.schedule_once().await, 0);

    let queued: Vec<String> =
        sqlx::query_scalar("SELECT kind FROM admin.jobs WHERE status = 'queued' ORDER BY kind")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(queued, ["backup_check", "maintenance_sweep"]);
}
