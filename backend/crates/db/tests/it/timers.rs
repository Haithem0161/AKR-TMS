use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use akr_db::{PgJobStore, PgTimerStore};
use akr_domain::audit::RecordingSink;
use akr_jobs::{
    JobContext, JobError, JobHandler, JobKind, JobQueue, JobRegistry, NewTimer, TimerKind,
    TimerRegistry, TimerService, TimerStore, TimerSweeper, Worker,
};
use chrono::{Duration, Utc};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::support::{count, run, settings};

static REVIEW_DUE: TimerKind = TimerKind {
    name: "test_review_due",
    owner_kinds: &["test_owner"],
    job: JobKind::new("timer_test_review_due"),
};

const OWNER: &str = "test_owner";

fn service(pool: &PgPool) -> TimerService {
    TimerService::new(Arc::new(PgTimerStore::new(pool.clone())))
}

fn sweeper(pool: &PgPool) -> TimerSweeper {
    let mut registry = TimerRegistry::new();
    registry.register(&REVIEW_DUE).unwrap();
    TimerSweeper::new(
        Arc::new(PgTimerStore::new(pool.clone())),
        JobQueue::new(Arc::new(PgJobStore::new(pool.clone()))),
        settings(pool, Arc::new(RecordingSink::new())),
        registry,
    )
}

async fn schedule(pool: &PgPool, owner: Uuid, due_in: Duration) -> Uuid {
    let mut tx = pool.begin().await.unwrap();
    let id = service(pool)
        .schedule(
            &mut tx,
            NewTimer::new(&REVIEW_DUE, OWNER, owner, Utc::now() + due_in)
                .payload(json!({"review_id": owner})),
        )
        .await
        .unwrap();
    tx.commit().await.unwrap();
    id
}

async fn timer_jobs(pool: &PgPool) -> i64 {
    count(
        pool,
        "SELECT count(*) FROM admin.jobs WHERE kind = 'timer_test_review_due'",
    )
    .await
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_due_timer_fires_once_under_two_sweeps(pool: PgPool) {
    let owner = Uuid::now_v7();
    let due = schedule(&pool, owner, Duration::seconds(-5)).await;
    schedule(&pool, Uuid::now_v7(), Duration::hours(1)).await;
    let (a, b) = (sweeper(&pool), sweeper(&pool));

    let (first, second) = tokio::join!(a.sweep_once(), b.sweep_once());
    assert_eq!(first.unwrap() + second.unwrap(), 1);
    assert_eq!(a.sweep_once().await.unwrap(), 0);

    assert_eq!(timer_jobs(&pool).await, 1);
    let payload: Value = sqlx::query_scalar("SELECT payload FROM admin.jobs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(payload["timer_id"], json!(due));
    assert_eq!(payload["owner_kind"], json!(OWNER));
    assert_eq!(payload["owner_id"], json!(owner));
    assert_eq!(payload["review_id"], json!(owner));
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.timers WHERE fired_at IS NOT NULL"
        )
        .await,
        1
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_timer_scheduled_in_a_transaction_that_rolls_back_does_not_exist(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();
    service(&pool)
        .schedule(
            &mut tx,
            NewTimer::new(&REVIEW_DUE, OWNER, Uuid::now_v7(), Utc::now()),
        )
        .await
        .unwrap();
    tx.rollback().await.unwrap();
    assert_eq!(count(&pool, "SELECT count(*) FROM admin.timers").await, 0);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_cancelled_timer_never_fires(pool: PgPool) {
    let timers = service(&pool);
    let by_id = schedule(&pool, Uuid::now_v7(), Duration::seconds(-5)).await;
    let owner = Uuid::now_v7();
    schedule(&pool, owner, Duration::seconds(-5)).await;
    schedule(&pool, owner, Duration::seconds(-4)).await;

    let mut tx = pool.begin().await.unwrap();
    assert_eq!(timers.cancel(&mut tx, by_id).await.unwrap(), 1);
    assert_eq!(
        timers
            .cancel_for_owner(&mut tx, OWNER, owner, &REVIEW_DUE)
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        timers.cancel(&mut tx, by_id).await.unwrap(),
        0,
        "cancelled once"
    );
    tx.commit().await.unwrap();

    assert_eq!(sweeper(&pool).sweep_once().await.unwrap(), 0);
    assert_eq!(timer_jobs(&pool).await, 0);
}

#[sqlx::test(migrations = "../../migrations")]
async fn cancelling_a_timer_that_fired_changes_nothing_and_the_owner_commits(pool: PgPool) {
    let timers = service(&pool);
    let owner = Uuid::now_v7();
    let fired = schedule(&pool, owner, Duration::seconds(-5)).await;
    assert_eq!(sweeper(&pool).sweep_once().await.unwrap(), 1);

    let mut tx = pool.begin().await.unwrap();
    assert_eq!(
        timers
            .cancel_for_owner(&mut tx, OWNER, owner, &REVIEW_DUE)
            .await
            .unwrap(),
        0
    );
    assert_eq!(timers.cancel(&mut tx, fired).await.unwrap(), 0);
    tx.commit().await.unwrap();

    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.timers WHERE fired_at IS NOT NULL AND cancelled_at IS NULL"
        )
        .await,
        1
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_timer_of_a_kind_nobody_registered_stays_pending(pool: PgPool) {
    run(
        &pool,
        "INSERT INTO admin.timers (id, owner_kind, owner_id, kind, fire_at)
         VALUES ('0190c000-0000-7000-8000-000000000001', 'test_owner',
                 '0190c000-0000-7000-8000-000000000002', 'test_unknown', now() - interval '1 hour')",
    )
    .await;
    assert_eq!(sweeper(&pool).sweep_once().await.unwrap(), 0);
    assert_eq!(
        PgTimerStore::new(pool.clone()).due_count().await.unwrap(),
        1
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_sweep_deletes_fired_and_cancelled_timers_past_retention_and_never_a_pending_one(
    pool: PgPool,
) {
    run(
        &pool,
        "INSERT INTO admin.timers (id, owner_kind, owner_id, kind, fire_at, created_at, fired_at, cancelled_at) VALUES
           ('0190c000-0000-7000-8000-000000000001', 'test_owner', '0190c000-0000-7000-8000-0000000000aa', 'test_review_due', now() - interval '100 days', now() - interval '120 days', now() - interval '91 days', NULL),
           ('0190c000-0000-7000-8000-000000000002', 'test_owner', '0190c000-0000-7000-8000-0000000000aa', 'test_review_due', now() - interval '100 days', now() - interval '120 days', NULL, now() - interval '95 days'),
           ('0190c000-0000-7000-8000-000000000003', 'test_owner', '0190c000-0000-7000-8000-0000000000aa', 'test_review_due', now() - interval '100 days', now() - interval '120 days', now() - interval '89 days', NULL),
           ('0190c000-0000-7000-8000-000000000004', 'test_owner', '0190c000-0000-7000-8000-0000000000aa', 'test_unknown', now() - interval '400 days', now() - interval '400 days', NULL, NULL)",
    )
    .await;

    let store = PgTimerStore::new(pool.clone());
    assert_eq!(
        store.delete_terminal_older_than(90, 5_000).await.unwrap(),
        2
    );
    assert_eq!(
        store.delete_terminal_older_than(90, 5_000).await.unwrap(),
        0
    );
    assert_eq!(count(&pool, "SELECT count(*) FROM admin.timers").await, 2);
}

/// The handler of a timer re-reads its owner: a timer that fires after its owner was closed does nothing.
struct ReviewDue {
    owner_closed: Arc<AtomicBool>,
    reminders: Arc<AtomicUsize>,
}

impl JobHandler for ReviewDue {
    const KIND: &'static str = "timer_test_review_due";

    async fn run(&self, _ctx: JobContext, payload: Value) -> Result<(), JobError> {
        if payload.get("owner_id").is_none() {
            return Err(JobError::permanent("the timer names no owner"));
        }
        if !self.owner_closed.load(Ordering::SeqCst) {
            self.reminders.fetch_add(1, Ordering::SeqCst);
        }
        Ok(())
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_handler_of_a_timer_whose_owner_is_closed_does_nothing(pool: PgPool) {
    let owner_closed = Arc::new(AtomicBool::new(false));
    let reminders = Arc::new(AtomicUsize::new(0));
    let mut registry = JobRegistry::new();
    registry
        .register(
            &REVIEW_DUE.job,
            ReviewDue {
                owner_closed: owner_closed.clone(),
                reminders: reminders.clone(),
            },
        )
        .unwrap();
    let audit = Arc::new(RecordingSink::new());
    let worker = Worker::new(
        Arc::new(PgJobStore::new(pool.clone())),
        settings(&pool, audit.clone()),
        audit,
        registry,
        "worker-a",
    );
    let sweeper = sweeper(&pool);

    schedule(&pool, Uuid::now_v7(), Duration::seconds(-5)).await;
    sweeper.sweep_once().await.unwrap();
    assert_eq!(worker.poll_once().await, 1);
    worker.drain().await;
    assert_eq!(reminders.load(Ordering::SeqCst), 1);

    schedule(&pool, Uuid::now_v7(), Duration::seconds(-5)).await;
    sweeper.sweep_once().await.unwrap();
    owner_closed.store(true, Ordering::SeqCst);
    assert_eq!(worker.poll_once().await, 1);
    worker.drain().await;
    assert_eq!(reminders.load(Ordering::SeqCst), 1);
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.jobs WHERE status = 'done'"
        )
        .await,
        2
    );
}
