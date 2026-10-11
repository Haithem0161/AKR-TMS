use std::sync::Arc;
use std::time::Duration;

use akr_common::AppError;
use akr_common::config::DEV_SEED;
use akr_common::idempotency::{
    Claim, ClaimError, IdempotencyService, IdempotencyStore, StoredResponse, request_hash,
};
use akr_crypto::InMemoryCrypto;
use akr_db::PgIdempotencyStore;
use akr_domain::audit::RecordingSink;
use sqlx::PgPool;
use uuid::Uuid;

use crate::support::{count, run, settings};

const KEY: &str = "note-key-0001";
const ENDPOINT: &str = "POST /api/v1/_echo";
const LEASE: Duration = Duration::from_secs(5);

fn service(pool: &PgPool) -> IdempotencyService {
    IdempotencyService::new(
        Arc::new(PgIdempotencyStore::new(pool.clone())),
        Arc::new(InMemoryCrypto::from_seed(DEV_SEED)),
        settings(pool, Arc::new(RecordingSink::new())),
    )
}

fn hash(principal: Uuid, body: &[u8]) -> [u8; 32] {
    request_hash("POST", "/api/v1/_echo", principal, body)
}

fn created(body: &str) -> StoredResponse {
    StoredResponse {
        status: 201,
        content_type: Some("application/json".to_owned()),
        body: body.as_bytes().to_vec(),
    }
}

async fn ledger_rows(pool: &PgPool) -> i64 {
    count(pool, "SELECT count(*) FROM admin.idempotency_keys").await
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_completed_request_replays_byte_for_byte(pool: PgPool) {
    let service = service(&pool);
    let principal = Uuid::now_v7();
    let body = br#"{"a":1}"#;
    let response = created(r#"{"id":"0190c000-0000-7000-8000-000000000042","a":1}"#);

    let mut tx = pool.begin().await.unwrap();
    let first = service
        .claim(
            &mut tx,
            principal,
            KEY,
            ENDPOINT,
            &hash(principal, body),
            LEASE,
        )
        .await
        .unwrap();
    assert_eq!(first, Claim::Run);
    service
        .complete(&mut tx, principal, KEY, &response)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let mut tx = pool.begin().await.unwrap();
    let second = service
        .claim(
            &mut tx,
            principal,
            KEY,
            ENDPOINT,
            &hash(principal, body),
            LEASE,
        )
        .await
        .unwrap();
    assert_eq!(second, Claim::Replay(response));
    tx.rollback().await.unwrap();
    assert_eq!(ledger_rows(&pool).await, 1);
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_same_key_with_another_body_is_a_reuse_and_another_principal_runs_alone(pool: PgPool) {
    let service = service(&pool);
    let principal = Uuid::now_v7();

    let mut tx = pool.begin().await.unwrap();
    service
        .claim(
            &mut tx,
            principal,
            KEY,
            ENDPOINT,
            &hash(principal, br#"{"a":1}"#),
            LEASE,
        )
        .await
        .unwrap();
    service
        .complete(&mut tx, principal, KEY, &created(r#"{"a":1}"#))
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let mut tx = pool.begin().await.unwrap();
    let reuse = service
        .claim(
            &mut tx,
            principal,
            KEY,
            ENDPOINT,
            &hash(principal, br#"{"a":2}"#),
            LEASE,
        )
        .await
        .unwrap_err();
    assert_eq!(reuse, AppError::IdempotencyReuse);
    assert_eq!(reuse.code(), "E_IDEMPOTENCY_REUSE");
    tx.rollback().await.unwrap();

    let other = Uuid::now_v7();
    let mut tx = pool.begin().await.unwrap();
    let independent = service
        .claim(
            &mut tx,
            other,
            KEY,
            ENDPOINT,
            &hash(other, br#"{"a":1}"#),
            LEASE,
        )
        .await
        .unwrap();
    assert_eq!(independent, Claim::Run);
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_stored_body_is_ciphertext_and_an_empty_body_stores_nothing(pool: PgPool) {
    let service = service(&pool);
    let principal = Uuid::now_v7();
    let plaintext = r#"{"narrative":"structuring suspected on the account"}"#;

    let mut tx = pool.begin().await.unwrap();
    service
        .claim(
            &mut tx,
            principal,
            KEY,
            ENDPOINT,
            &hash(principal, b"x"),
            LEASE,
        )
        .await
        .unwrap();
    service
        .complete(&mut tx, principal, KEY, &created(plaintext))
        .await
        .unwrap();
    service
        .claim(
            &mut tx,
            principal,
            "empty-key-0001",
            ENDPOINT,
            &hash(principal, b"y"),
            LEASE,
        )
        .await
        .unwrap();
    let no_content = StoredResponse {
        status: 204,
        content_type: None,
        body: Vec::new(),
    };
    service
        .complete(&mut tx, principal, "empty-key-0001", &no_content)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let stored: Vec<u8> =
        sqlx::query_scalar("SELECT response_body FROM admin.idempotency_keys WHERE key = $1")
            .bind(KEY)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(stored.len() > plaintext.len());
    for needle in ["narrative", "structuring", "account"] {
        assert!(
            !stored.windows(needle.len()).any(|w| w == needle.as_bytes()),
            "the stored body shows `{needle}`"
        );
    }

    let empty: Option<Vec<u8>> =
        sqlx::query_scalar("SELECT response_body FROM admin.idempotency_keys WHERE key = $1")
            .bind("empty-key-0001")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(empty, None);

    let mut tx = pool.begin().await.unwrap();
    let replay = service
        .claim(
            &mut tx,
            principal,
            "empty-key-0001",
            ENDPOINT,
            &hash(principal, b"y"),
            LEASE,
        )
        .await
        .unwrap();
    assert_eq!(replay, Claim::Replay(no_content));
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_request_that_does_not_commit_leaves_no_row_and_a_retry_runs(pool: PgPool) {
    let service = service(&pool);
    let principal = Uuid::now_v7();
    let h = hash(principal, b"x");

    let mut tx = pool.begin().await.unwrap();
    assert_eq!(
        service
            .claim(&mut tx, principal, KEY, ENDPOINT, &h, LEASE)
            .await
            .unwrap(),
        Claim::Run
    );
    service
        .complete(&mut tx, principal, KEY, &created("{}"))
        .await
        .unwrap();
    tx.rollback().await.unwrap();
    assert_eq!(ledger_rows(&pool).await, 0);

    let mut tx = pool.begin().await.unwrap();
    assert_eq!(
        service
            .claim(&mut tx, principal, KEY, ENDPOINT, &h, LEASE)
            .await
            .unwrap(),
        Claim::Run
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_claim_waits_on_a_request_in_flight_and_gives_up_after_the_lease(pool: PgPool) {
    let store = PgIdempotencyStore::new(pool.clone());
    let principal = Uuid::now_v7();
    let h = hash(principal, b"x");

    let mut first = pool.begin().await.unwrap();
    assert!(
        store
            .claim(&mut first, principal, KEY, ENDPOINT, &h, LEASE)
            .await
            .unwrap()
    );

    let mut second = pool.begin().await.unwrap();
    let waited = store
        .claim(
            &mut second,
            principal,
            KEY,
            ENDPOINT,
            &h,
            Duration::from_millis(300),
        )
        .await;
    assert!(matches!(waited, Err(ClaimError::InFlight)), "{waited:?}");
    second.rollback().await.unwrap();

    first.commit().await.unwrap();
    assert_eq!(ledger_rows(&pool).await, 1);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_waiting_claim_takes_the_key_when_the_first_request_rolls_back(pool: PgPool) {
    let store = Arc::new(PgIdempotencyStore::new(pool.clone()));
    let principal = Uuid::now_v7();
    let h = hash(principal, b"x");

    let mut first = pool.begin().await.unwrap();
    assert!(
        store
            .claim(&mut first, principal, KEY, ENDPOINT, &h, LEASE)
            .await
            .unwrap()
    );

    let waiter = tokio::spawn({
        let store = store.clone();
        let pool = pool.clone();
        async move {
            let mut tx = pool.begin().await.unwrap();
            let claimed = store
                .claim(&mut tx, principal, KEY, ENDPOINT, &h, LEASE)
                .await
                .unwrap();
            tx.commit().await.unwrap();
            claimed
        }
    });
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(!waiter.is_finished(), "the second claim waits");
    first.rollback().await.unwrap();

    assert!(waiter.await.unwrap());
    assert_eq!(ledger_rows(&pool).await, 1);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_waiting_claim_replays_when_the_first_request_commits(pool: PgPool) {
    let service = service(&pool);
    let principal = Uuid::now_v7();
    let h = hash(principal, b"x");
    let response = created(r#"{"n":1}"#);

    let mut first = pool.begin().await.unwrap();
    service
        .claim(&mut first, principal, KEY, ENDPOINT, &h, LEASE)
        .await
        .unwrap();

    let waiter = tokio::spawn({
        let service = service.clone();
        let pool = pool.clone();
        async move {
            let mut tx = pool.begin().await.unwrap();
            service
                .claim(&mut tx, principal, KEY, ENDPOINT, &h, LEASE)
                .await
        }
    });
    tokio::time::sleep(Duration::from_millis(200)).await;
    service
        .complete(&mut first, principal, KEY, &response)
        .await
        .unwrap();
    first.commit().await.unwrap();

    assert_eq!(waiter.await.unwrap().unwrap(), Claim::Replay(response));
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_sweep_deletes_only_the_rows_past_their_retention(pool: PgPool) {
    run(
        &pool,
        "INSERT INTO admin.idempotency_keys
             (principal_id, key, endpoint, request_hash, response_status, created_at, completed_at)
         VALUES ('0190c000-0000-7000-8000-000000000001', 'old-key-0001', 'POST /x', decode(repeat('00', 32), 'hex'),
                 204, now() - interval '8 days', now() - interval '8 days'),
                ('0190c000-0000-7000-8000-000000000001', 'new-key-0001', 'POST /x', decode(repeat('00', 32), 'hex'),
                 204, now() - interval '6 days', now() - interval '6 days')",
    )
    .await;

    assert_eq!(service(&pool).sweep().await.unwrap(), 1);
    assert_eq!(
        count(
            &pool,
            "SELECT count(*) FROM admin.idempotency_keys WHERE key = 'new-key-0001'"
        )
        .await,
        1
    );
    assert_eq!(ledger_rows(&pool).await, 1);
}
