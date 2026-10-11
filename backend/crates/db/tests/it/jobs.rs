use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use akr_db::{PgJobStore, listen_for_jobs};
use akr_domain::audit::RecordingSink;
use akr_jobs::{
    JobContext, JobError, JobHandler, JobKind, JobQueue, JobRegistry, JobStore, NewJob, Worker,
    kinds,
};
use serde_json::{Value, json};
use sqlx::PgPool;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::support::{count, run, settings};

static COUNTED: JobKind = JobKind::new("test_counted");
static FAILING: JobKind = JobKind::new("test_failing").max_attempts(2);
static HOPELESS: JobKind = JobKind::new("test_hopeless");
static ONE_SHOT: JobKind = JobKind::new("test_one_shot").max_attempts(1);
static SINGLE: JobKind = JobKind::new("test_single");
static IN_TX: JobKind = JobKind::new("test_in_tx");

#[derive(Clone, Default)]
struct Runs(Arc<AtomicUsize>);

impl Runs {
    fn add(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }

    fn get(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }
}

struct Counted(Runs);

impl JobHandler for Counted {
    const KIND: &'static str = "test_counted";

    async fn run(&self, _ctx: JobContext, _payload: Value) -> Result<(), JobError> {
        self.0.add();
        Ok(())
    }
}

struct Failing(Runs);

impl JobHandler for Failing {
    const KIND: &'static str = "test_failing";

    async fn run(&self, _ctx: JobContext, _payload: Value) -> Result<(), JobError> {
        self.0.add();
        Err(JobError::retryable("the source did not answer"))
    }
}

struct Hopeless(Runs);

impl JobHandler for Hopeless {
    const KIND: &'static str = "test_hopeless";

    async fn run(&self, _ctx: JobContext, _payload: Value) -> Result<(), JobError> {
        self.0.add();
        Err(JobError::permanent("the payload names no item"))
    }
}

struct OneShot;

impl JobHandler for OneShot {
    const KIND: &'static str = "test_one_shot";

    async fn run(&self, _ctx: JobContext, _payload: Value) -> Result<(), JobError> {
        Ok(())
    }
}

/// Writes its effect and its completion in one transaction (DR-412).
struct InTx(PgPool);

impl JobHandler for InTx {
    const KIND: &'static str = "test_in_tx";

    async fn run(&self, ctx: JobContext, _payload: Value) -> Result<(), JobError> {
        let mut tx = self.0.begin().await?;
        sqlx::query(
            "INSERT INTO admin.outbox (id, topic, payload) VALUES ($1, 'test.effect', '{}')",
        )
        .bind(Uuid::now_v7())
        .execute(&mut *tx)
        .await?;
        ctx.complete_in(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }
}

struct Sweep;

impl JobHandler for Sweep {
    const KIND: &'static str = "maintenance_sweep";

    async fn run(&self, _ctx: JobContext, _payload: Value) -> Result<(), JobError> {
        Ok(())
    }
}

fn store(pool: &PgPool) -> Arc<PgJobStore> {
    Arc::new(PgJobStore::new(pool.clone()))
}

fn worker(pool: &PgPool, audit: Arc<RecordingSink>, registry: JobRegistry, id: &str) -> Worker {
    Worker::new(
        store(pool),
        settings(pool, audit.clone()),
        audit,
        registry,
        id,
    )
}

async fn enqueue(pool: &PgPool, job: NewJob) -> Option<Uuid> {
    let queue = JobQueue::new(store(pool));
    let mut tx = pool.begin().await.unwrap();
    let id = queue.enqueue(&mut tx, job).await.unwrap();
    tx.commit().await.unwrap();
    id
}

async fn status(pool: &PgPool, id: Uuid) -> (String, i32) {
    sqlx::query_as("SELECT status::text, attempts FROM admin.jobs WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn jobs(pool: &PgPool) -> i64 {
    count(pool, "SELECT count(*) FROM admin.jobs").await
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_job_enqueued_in_a_transaction_that_rolls_back_does_not_exist(pool: PgPool) {
    let queue = JobQueue::new(store(&pool));
    let mut tx = pool.begin().await.unwrap();
    let id = queue
        .enqueue(
            &mut tx,
            NewJob::new(&COUNTED).payload(json!({"item_id": Uuid::now_v7()})),
        )
        .await
        .unwrap();
    assert!(id.is_some());
    tx.rollback().await.unwrap();
    assert_eq!(jobs(&pool).await, 0);

    assert!(enqueue(&pool, NewJob::new(&COUNTED)).await.is_some());
    assert_eq!(jobs(&pool).await, 1);
}

#[sqlx::test(migrations = "../../migrations")]
async fn two_enqueues_with_one_coalesce_key_make_one_queued_job(pool: PgPool) {
    let keyed = || NewJob::new(&SINGLE).coalesce_key("customer:42");
    assert!(enqueue(&pool, keyed()).await.is_some());
    assert!(enqueue(&pool, keyed()).await.is_none());
    assert!(
        enqueue(&pool, NewJob::new(&SINGLE).coalesce_key("customer:43"))
            .await
            .is_some()
    );
    assert_eq!(jobs(&pool).await, 2);

    let running = store(&pool)
        .claim(SINGLE.name, 1, 1, "worker-a")
        .await
        .unwrap();
    assert_eq!(running.len(), 1);
    assert!(
        enqueue(&pool, keyed()).await.is_some(),
        "a change after the job started needs a fresh run"
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_job_runs_once_and_is_done(pool: PgPool) {
    let runs = Runs::default();
    let mut registry = JobRegistry::new();
    registry.register(&COUNTED, Counted(runs.clone())).unwrap();
    let worker = worker(&pool, Arc::new(RecordingSink::new()), registry, "worker-a");

    let id = enqueue(&pool, NewJob::new(&COUNTED)).await.unwrap();
    let later = enqueue(
        &pool,
        NewJob::new(&COUNTED).run_at(chrono::Utc::now() + chrono::Duration::hours(1)),
    )
    .await
    .unwrap();

    assert_eq!(worker.poll_once().await, 1);
    worker.drain().await;
    assert_eq!(runs.get(), 1);
    assert_eq!(status(&pool, id).await, ("done".to_owned(), 1));
    assert_eq!(status(&pool, later).await, ("queued".to_owned(), 0));
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.jobs WHERE status = 'done' AND finished_at IS NOT NULL"
        )
        .await,
        1
    );

    assert_eq!(worker.poll_once().await, 0);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_failing_job_backs_off_and_dies_at_its_last_attempt(pool: PgPool) {
    let runs = Runs::default();
    let audit = Arc::new(RecordingSink::new());
    let mut registry = JobRegistry::new();
    registry.register(&FAILING, Failing(runs.clone())).unwrap();
    let worker = worker(&pool, audit.clone(), registry, "worker-a");
    let id = enqueue(&pool, NewJob::new(&FAILING)).await.unwrap();

    assert_eq!(worker.poll_once().await, 1);
    worker.drain().await;
    assert_eq!(status(&pool, id).await, ("queued".to_owned(), 1));
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.jobs
             WHERE run_at > now() + interval '2 seconds' AND last_error = 'the source did not answer'"
        )
        .await,
        1,
        "the job waits out its back-off"
    );
    assert_eq!(worker.poll_once().await, 0);
    assert!(audit.names().is_empty());

    run(&pool, "UPDATE admin.jobs SET run_at = now()").await;
    assert_eq!(worker.poll_once().await, 1);
    worker.drain().await;
    assert_eq!(runs.get(), 2);
    assert_eq!(status(&pool, id).await, ("dead".to_owned(), 2));
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.jobs WHERE finished_at IS NOT NULL"
        )
        .await,
        1
    );

    assert_eq!(audit.names(), ["job.dead"]);
    let event = &audit.events()[0];
    assert_eq!(event.resource.id, Some(id));
    assert_eq!(event.details["kind"].to_json(), json!("test_failing"));
    assert_eq!(event.details["attempts"].to_json(), json!(2));

    run(&pool, "UPDATE admin.jobs SET run_at = now()").await;
    assert_eq!(worker.poll_once().await, 0, "a dead job never runs again");
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_permanent_error_kills_the_job_at_once(pool: PgPool) {
    let runs = Runs::default();
    let audit = Arc::new(RecordingSink::new());
    let mut registry = JobRegistry::new();
    registry
        .register(&HOPELESS, Hopeless(runs.clone()))
        .unwrap();
    let worker = worker(&pool, audit.clone(), registry, "worker-a");
    let id = enqueue(&pool, NewJob::new(&HOPELESS)).await.unwrap();

    worker.poll_once().await;
    worker.drain().await;
    assert_eq!(runs.get(), 1);
    assert_eq!(status(&pool, id).await, ("dead".to_owned(), 1));
    assert_eq!(audit.names(), ["job.dead"]);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_job_whose_worker_died_is_taken_back_after_the_lease_and_completes_once(pool: PgPool) {
    let runs = Runs::default();
    let mut registry = JobRegistry::new();
    registry.register(&COUNTED, Counted(runs.clone())).unwrap();
    let worker = worker(&pool, Arc::new(RecordingSink::new()), registry, "worker-b");
    let store = store(&pool);
    let id = enqueue(&pool, NewJob::new(&COUNTED)).await.unwrap();

    let lost = store.claim(COUNTED.name, 1, 1, "worker-a").await.unwrap();
    assert_eq!(lost[0].attempts, 1);
    assert_eq!(worker.poll_once().await, 0, "worker-a holds the job");
    assert_eq!(worker.reap_once().await, 0, "the lease has not run out");

    run(
        &pool,
        "UPDATE admin.jobs SET locked_at = now() - interval '11 minutes'",
    )
    .await;
    assert_eq!(worker.reap_once().await, 1);
    assert_eq!(status(&pool, id).await, ("queued".to_owned(), 1));
    assert!(
        !store.heartbeat(id, "worker-a").await.unwrap(),
        "a late heartbeat changes nothing"
    );

    assert_eq!(worker.poll_once().await, 1);
    worker.drain().await;
    assert_eq!(runs.get(), 1);
    assert_eq!(status(&pool, id).await, ("done".to_owned(), 2));
    assert!(
        !store.complete(id, 1).await.unwrap(),
        "the first claim is gone"
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_reaper_buries_a_job_that_used_its_last_attempt(pool: PgPool) {
    let audit = Arc::new(RecordingSink::new());
    let mut registry = JobRegistry::new();
    registry.register(&ONE_SHOT, OneShot).unwrap();
    let worker = worker(&pool, audit.clone(), registry, "worker-b");
    let id = enqueue(&pool, NewJob::new(&ONE_SHOT)).await.unwrap();

    store(&pool)
        .claim(ONE_SHOT.name, 1, 1, "worker-a")
        .await
        .unwrap();
    run(
        &pool,
        "UPDATE admin.jobs SET locked_at = now() - interval '11 minutes'",
    )
    .await;

    assert_eq!(worker.reap_once().await, 1);
    assert_eq!(status(&pool, id).await, ("dead".to_owned(), 1));
    assert_eq!(audit.names(), ["job.dead"]);
    assert_eq!(worker.poll_once().await, 0);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_handler_commits_its_effect_and_its_completion_together(pool: PgPool) {
    let mut registry = JobRegistry::new();
    registry.register(&IN_TX, InTx(pool.clone())).unwrap();
    let worker = worker(&pool, Arc::new(RecordingSink::new()), registry, "worker-a");
    let id = enqueue(&pool, NewJob::new(&IN_TX)).await.unwrap();

    worker.poll_once().await;
    worker.drain().await;
    assert_eq!(status(&pool, id).await, ("done".to_owned(), 1));
    assert_eq!(count(&pool, "SELECT count(*) FROM admin.outbox").await, 1);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_completion_by_a_claim_that_was_lost_changes_nothing(pool: PgPool) {
    let store = store(&pool);
    let id = enqueue(&pool, NewJob::new(&COUNTED)).await.unwrap();

    store.claim(COUNTED.name, 1, 1, "worker-a").await.unwrap();
    run(
        &pool,
        "UPDATE admin.jobs SET locked_at = now() - interval '11 minutes'",
    )
    .await;
    let mut tx = pool.begin().await.unwrap();
    store
        .reap_expired(&mut tx, COUNTED.name, Duration::from_secs(600))
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let again = store.claim(COUNTED.name, 1, 1, "worker-b").await.unwrap();
    assert_eq!(again[0].attempts, 2);

    let mut tx = pool.begin().await.unwrap();
    assert!(!store.complete_in(&mut tx, id, 1).await.unwrap());
    assert!(
        !store
            .fail(id, 1, "late", Duration::from_secs(1))
            .await
            .unwrap()
    );
    assert!(!store.bury(&mut tx, id, 1, "late").await.unwrap());
    assert!(store.complete_in(&mut tx, id, 2).await.unwrap());
    tx.commit().await.unwrap();
    assert_eq!(status(&pool, id).await, ("done".to_owned(), 2));
}

#[sqlx::test(migrations = "../../migrations")]
async fn two_workers_enqueue_one_job_per_slot(pool: PgPool) {
    let build = |id: &str| {
        let mut registry = JobRegistry::new();
        registry.register(&kinds::MAINTENANCE_SWEEP, Sweep).unwrap();
        worker(&pool, Arc::new(RecordingSink::new()), registry, id)
    };
    let (a, b) = (build("worker-a"), build("worker-b"));

    let (first, second) = tokio::join!(a.schedule_once(), b.schedule_once());
    assert_eq!(first + second, 1);
    let (first, second) = tokio::join!(a.schedule_once(), b.schedule_once());
    assert_eq!(first + second, 0);

    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.jobs
             WHERE kind = 'maintenance_sweep' AND status = 'queued' AND run_at > now()
               AND coalesce_key = 'maintenance_sweep:' || (payload->>'slot')"
        )
        .await,
        1
    );

    // The slot's job ran. The partial unique index no longer covers the row, and the slot is still
    // enqueued once.
    run(
        &pool,
        "UPDATE admin.jobs SET status = 'done', finished_at = now()",
    )
    .await;
    assert_eq!(a.schedule_once().await, 0);
    assert_eq!(jobs(&pool).await, 1);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_kind_never_runs_more_than_its_concurrency_across_workers(pool: PgPool) {
    let store = store(&pool);
    enqueue(&pool, NewJob::new(&SINGLE)).await.unwrap();
    enqueue(&pool, NewJob::new(&SINGLE)).await.unwrap();

    let first = store.claim(SINGLE.name, 1, 5, "worker-a").await.unwrap();
    assert_eq!(first.len(), 1);
    assert!(
        store
            .claim(SINGLE.name, 1, 5, "worker-b")
            .await
            .unwrap()
            .is_empty()
    );
    let counts = store.counts().await.unwrap();
    assert_eq!((counts.queued, counts.running), (1, 1));

    assert!(store.complete(first[0].id, 1).await.unwrap());
    assert_eq!(
        store
            .claim(SINGLE.name, 1, 5, "worker-b")
            .await
            .unwrap()
            .len(),
        1
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_worker_claims_only_the_kinds_it_knows(pool: PgPool) {
    let mut registry = JobRegistry::new();
    registry
        .register(&FAILING, Failing(Runs::default()))
        .unwrap();
    let worker = worker(&pool, Arc::new(RecordingSink::new()), registry, "worker-a");
    let id = enqueue(&pool, NewJob::new(&COUNTED)).await.unwrap();

    assert_eq!(worker.poll_once().await, 0);
    assert_eq!(status(&pool, id).await, ("queued".to_owned(), 0));
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_sweep_deletes_only_finished_jobs_past_their_retention(pool: PgPool) {
    run(
        &pool,
        "INSERT INTO admin.jobs (id, kind, status, created_at, finished_at) VALUES
           ('0190c000-0000-7000-8000-000000000001', 'test_counted', 'done', now() - interval '40 days', now() - interval '40 days'),
           ('0190c000-0000-7000-8000-000000000002', 'test_counted', 'dead', now() - interval '40 days', now() - interval '31 days'),
           ('0190c000-0000-7000-8000-000000000003', 'test_counted', 'done', now() - interval '40 days', now() - interval '29 days'),
           ('0190c000-0000-7000-8000-000000000004', 'test_counted', 'queued', now() - interval '400 days', NULL)",
    )
    .await;

    let store = store(&pool);
    assert_eq!(store.delete_finished_older_than(30, 1).await.unwrap(), 1);
    assert_eq!(
        store.delete_finished_older_than(30, 5_000).await.unwrap(),
        1
    );
    assert_eq!(
        store.delete_finished_older_than(30, 5_000).await.unwrap(),
        0
    );
    assert_eq!(jobs(&pool).await, 2);
}

#[sqlx::test(migrations = "../../migrations")]
async fn an_enqueue_wakes_the_listener(pool: PgPool) {
    let wake = Arc::new(Notify::new());
    let shutdown = CancellationToken::new();
    let listener = tokio::spawn(listen_for_jobs(
        pool.clone(),
        wake.clone(),
        shutdown.clone(),
    ));

    let woken = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            enqueue(&pool, NewJob::new(&COUNTED)).await;
            if tokio::time::timeout(Duration::from_millis(300), wake.notified())
                .await
                .is_ok()
            {
                break;
            }
        }
    })
    .await;
    assert!(woken.is_ok(), "no notification arrived");

    shutdown.cancel();
    listener.await.unwrap();
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_ingest_role_sees_and_writes_only_its_own_kinds(pool: PgPool) {
    run(
        &pool,
        "INSERT INTO admin.jobs (id, kind) VALUES
           ('0190c000-0000-7000-8000-000000000001', 'maintenance_sweep'),
           ('0190c000-0000-7000-8000-000000000002', 'ingest_pull'),
           ('0190c000-0000-7000-8000-000000000003', 'discovery_scan'),
           ('0190c000-0000-7000-8000-000000000004', 'map_batch')",
    )
    .await;

    let mut conn = pool.acquire().await.unwrap();
    sqlx::query("SET ROLE akr_ingest")
        .execute(&mut *conn)
        .await
        .unwrap();

    let visible: Vec<String> = sqlx::query_scalar("SELECT kind FROM admin.jobs ORDER BY kind")
        .fetch_all(&mut *conn)
        .await
        .unwrap();
    assert_eq!(visible, ["discovery_scan", "ingest_pull"]);

    let refused = sqlx::query(
        "INSERT INTO admin.jobs (id, kind) VALUES ('0190c000-0000-7000-8000-000000000005', 'maintenance_sweep')",
    )
    .execute(&mut *conn)
    .await
    .unwrap_err();
    assert_eq!(
        refused.as_database_error().unwrap().code().as_deref(),
        Some("42501"),
        "{refused}"
    );

    sqlx::query(
        "INSERT INTO admin.jobs (id, kind) VALUES ('0190c000-0000-7000-8000-000000000006', 'map_batch')",
    )
    .execute(&mut *conn)
    .await
    .unwrap();

    let touched = sqlx::query("UPDATE admin.jobs SET run_at = now()")
        .execute(&mut *conn)
        .await
        .unwrap();
    assert_eq!(
        touched.rows_affected(),
        2,
        "only its own two rows are in reach"
    );

    let deleted = sqlx::query("DELETE FROM admin.jobs")
        .execute(&mut *conn)
        .await
        .unwrap_err();
    assert_eq!(
        deleted.as_database_error().unwrap().code().as_deref(),
        Some("42501")
    );

    sqlx::query("RESET ROLE").execute(&mut *conn).await.unwrap();
}
