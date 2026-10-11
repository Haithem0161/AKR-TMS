use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use akr_db::PgOutboxStore;
use akr_domain::audit::RecordingSink;
use akr_jobs::{
    JobError, Outbox, OutboxDispatcher, OutboxRow, OutboxStore, TopicHandler, TopicRegistry,
};
use async_trait::async_trait;
use serde_json::json;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::support::{count, run, settings};

const TOPIC: &str = "test.ping";

/// Writes one timer row per delivery, in the dispatcher's transaction, so a test can see that the effect and
/// the dispatch mark commit together.
struct Delivering;

#[async_trait]
impl TopicHandler for Delivering {
    async fn handle(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        row: &OutboxRow,
    ) -> Result<(), JobError> {
        sqlx::query(
            "INSERT INTO admin.timers (id, owner_kind, owner_id, kind, fire_at)
             VALUES ($1, 'test_owner', $2, 'test_delivered', now() + interval '1 day')",
        )
        .bind(Uuid::now_v7())
        .bind(row.id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}

#[derive(Default)]
struct Refusing(AtomicUsize);

#[async_trait]
impl TopicHandler for Refusing {
    async fn handle(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        row: &OutboxRow,
    ) -> Result<(), JobError> {
        self.0.fetch_add(1, Ordering::SeqCst);
        sqlx::query(
            "INSERT INTO admin.timers (id, owner_kind, owner_id, kind, fire_at)
             VALUES ($1, 'test_owner', $2, 'test_delivered', now())",
        )
        .bind(Uuid::now_v7())
        .bind(row.id)
        .execute(&mut **tx)
        .await?;
        Err(JobError::retryable("the consumer is down"))
    }
}

fn dispatcher(pool: &PgPool, handler: Arc<dyn TopicHandler>) -> OutboxDispatcher {
    let mut registry = TopicRegistry::new();
    registry.register(TOPIC, handler);
    OutboxDispatcher::new(
        Arc::new(PgOutboxStore::new(pool.clone())),
        settings(pool, Arc::new(RecordingSink::new())),
        registry,
    )
}

async fn append(pool: &PgPool, topic: &str) -> Uuid {
    let outbox = Outbox::new(Arc::new(PgOutboxStore::new(pool.clone())));
    let mut tx = pool.begin().await.unwrap();
    let id = outbox
        .append(&mut tx, topic, json!({"item_id": Uuid::now_v7()}))
        .await
        .unwrap();
    tx.commit().await.unwrap();
    id
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_row_is_delivered_once_with_its_effect(pool: PgPool) {
    let dispatcher = dispatcher(&pool, Arc::new(Delivering));
    append(&pool, TOPIC).await;
    append(&pool, TOPIC).await;
    append(&pool, "test.unheard").await;

    assert_eq!(dispatcher.dispatch_once().await.unwrap(), 2);
    assert_eq!(dispatcher.dispatch_once().await.unwrap(), 0);
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.outbox WHERE dispatched_at IS NOT NULL"
        )
        .await,
        2
    );
    assert_eq!(count(&pool, "SELECT count(*) FROM admin.timers").await, 2);
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.outbox WHERE topic = 'test.unheard' AND dispatched_at IS NULL"
        )
        .await,
        1,
        "a topic nobody registered stays"
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_row_appended_in_a_transaction_that_rolls_back_does_not_exist(pool: PgPool) {
    let outbox = Outbox::new(Arc::new(PgOutboxStore::new(pool.clone())));
    let mut tx = pool.begin().await.unwrap();
    outbox.append(&mut tx, TOPIC, json!({})).await.unwrap();
    tx.rollback().await.unwrap();
    assert_eq!(count(&pool, "SELECT count(*) FROM admin.outbox").await, 0);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_failing_handler_backs_off_and_stops_at_the_attempt_limit(pool: PgPool) {
    run(
        &pool,
        "UPDATE admin.settings SET value = '2' WHERE key = 'outbox.max_attempts'",
    )
    .await;
    let handler = Arc::new(Refusing::default());
    let dispatcher = dispatcher(&pool, handler.clone());
    let store = PgOutboxStore::new(pool.clone());
    let id = append(&pool, TOPIC).await;

    assert_eq!(dispatcher.dispatch_once().await.unwrap(), 0);
    assert_eq!(handler.0.load(Ordering::SeqCst), 1);
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.outbox
             WHERE attempts = 1 AND available_at > now() + interval '2 seconds'
               AND last_error = 'the consumer is down' AND dispatched_at IS NULL"
        )
        .await,
        1
    );
    assert_eq!(
        count(&pool, "SELECT count(*) FROM admin.timers").await,
        0,
        "the effect of a failed delivery is rolled back"
    );
    dispatcher.dispatch_once().await.unwrap();
    assert_eq!(
        handler.0.load(Ordering::SeqCst),
        1,
        "not due again before the back-off"
    );

    run(&pool, "UPDATE admin.outbox SET available_at = now()").await;
    dispatcher.dispatch_once().await.unwrap();
    assert_eq!(handler.0.load(Ordering::SeqCst), 2);

    run(&pool, "UPDATE admin.outbox SET available_at = now()").await;
    dispatcher.dispatch_once().await.unwrap();
    assert_eq!(
        handler.0.load(Ordering::SeqCst),
        2,
        "the row is at the limit"
    );

    let counts = store.counts(2).await.unwrap();
    assert_eq!((counts.pending, counts.stuck), (0, 1));
    assert_eq!(
        count(
            &pool,
            &format!(
                "SELECT count(*) FROM admin.outbox WHERE id = '{id}' AND dispatched_at IS NULL"
            )
        )
        .await,
        1
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_sweep_deletes_dispatched_rows_past_retention_and_never_an_undispatched_one(
    pool: PgPool,
) {
    run(
        &pool,
        "INSERT INTO admin.outbox (id, topic, payload, created_at, available_at, dispatched_at, attempts) VALUES
           ('0190c000-0000-7000-8000-000000000001', 'test.ping', '{}', now() - interval '40 days', now() - interval '40 days', now() - interval '31 days', 1),
           ('0190c000-0000-7000-8000-000000000002', 'test.ping', '{}', now() - interval '40 days', now() - interval '40 days', now() - interval '29 days', 1),
           ('0190c000-0000-7000-8000-000000000003', 'test.ping', '{}', now() - interval '400 days', now() - interval '400 days', NULL, 10),
           ('0190c000-0000-7000-8000-000000000004', 'test.ping', '{}', now() - interval '400 days', now() - interval '400 days', NULL, 0)",
    )
    .await;

    let store = PgOutboxStore::new(pool.clone());
    assert_eq!(
        store.delete_dispatched_older_than(30, 5_000).await.unwrap(),
        1
    );
    assert_eq!(
        store.delete_dispatched_older_than(30, 5_000).await.unwrap(),
        0
    );
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.outbox WHERE dispatched_at IS NULL"
        )
        .await,
        2
    );
    assert_eq!(count(&pool, "SELECT count(*) FROM admin.outbox").await, 3);
}
