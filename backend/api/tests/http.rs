//! The request pipeline against a real database: the idempotency ledger (DR-21, DR-412), the auth slot,
//! the body limit, the probes and what a production build serves.

#![allow(clippy::unwrap_used)]

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use akr_api::config::MAX_BODY_BYTES;
use akr_api::middleware::principal::DEV_PRINCIPAL_HEADER;
use akr_api::{ApiConfig, AppState, Dependencies, build_app, build_app_with, valkey};
use akr_audit::TracingSink;
use akr_authz::{AllowAllAuthorizer, Authorizer, AuthzError, AuthzRequest, Decision, ResourceKind};
use akr_common::AppError;
use akr_common::config::{Binary, CoreConfig, DEV_SEED};
use akr_common::correlation::CORRELATION_HEADER;
use akr_common::idempotency::{IDEMPOTENCY_KEY_HEADER, REPLAY_HEADER, RequestTx};
use akr_crypto::InMemoryCrypto;
use akr_domain::audit::Effect;
use akr_objectstore::MemoryObjectStore;
use async_trait::async_trait;
use axum::body::Body;
use axum::routing::post;
use axum::{Json, Router};
use bytes::Bytes;
use http::{HeaderMap, Method, Request, StatusCode, header};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::PgPool;
use tokio::task::JoinSet;
use tower::ServiceExt;
use utoipa_axum::router::OpenApiRouter;
use uuid::Uuid;
use zeroize::Zeroizing;

const ECHO: &str = "/api/v1/_echo";
const COUNT: &str = "/api/v1/_count";
const REFUSE: &str = "/api/v1/_refuse";
const SLOW: &str = "/api/v1/_slow";
const NOTE: &str = "/api/v1/_note";

/// How long the slow route holds its request open.
const SLOW_FOR: Duration = Duration::from_secs(2);

const DEV_VALKEY: &str = "redis://127.0.0.1:6470";
/// A port nothing listens on.
const NO_VALKEY: &str = "redis://127.0.0.1:1";

/// An authorizer a production state can be built with: it allows nothing.
struct DenyAll;

#[async_trait]
impl Authorizer for DenyAll {
    async fn authorize(&self, _req: AuthzRequest<'_>) -> Result<Decision, AuthzError> {
        Ok(Decision {
            effect: Effect::Deny,
            policy_ids: Vec::new(),
        })
    }

    async fn invalidate_resource(&self, _kind: ResourceKind, _id: Uuid) {}
}

fn state_for(pool: &PgPool, env: &str, valkey_url: &str) -> AppState {
    let vars = HashMap::from([("AKR_ENV".to_owned(), env.to_owned())]);
    let core = CoreConfig::from_map(Binary::Api, &vars).unwrap();
    let authz: Arc<dyn Authorizer> = match AllowAllAuthorizer::new(core.env) {
        Ok(allow_all) => Arc::new(allow_all),
        Err(_) => Arc::new(DenyAll),
    };
    AppState::new(Dependencies {
        pool: pool.clone(),
        valkey: valkey::pool(valkey_url, 1).unwrap(),
        crypto: Arc::new(InMemoryCrypto::from_seed(DEV_SEED)),
        store: Arc::new(MemoryObjectStore::new()),
        audit: Arc::new(TracingSink::new()),
        authz,
        cursor_key: Zeroizing::new(DEV_SEED.to_vec()),
        config: ApiConfig::new(core),
    })
    .unwrap()
}

fn state(pool: &PgPool) -> AppState {
    state_for(pool, "test", NO_VALKEY)
}

/// Counts the runs of the test handlers.
#[derive(Clone, Default)]
struct Runs(Arc<AtomicUsize>);

impl Runs {
    fn add(&self) -> usize {
        self.0.fetch_add(1, Ordering::SeqCst) + 1
    }

    fn count(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }
}

/// Routes that exist for these tests only, mounted under `/api/v1` behind the whole pipeline.
fn probes(runs: &Runs) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .route(
            "/_count",
            post({
                let runs = runs.clone();
                move || counted(runs.clone())
            }),
        )
        .route(
            "/_refuse",
            post({
                let runs = runs.clone();
                move || refused(runs.clone())
            }),
        )
        .route(
            "/_slow",
            post({
                let runs = runs.clone();
                move || slow(runs.clone())
            }),
        )
        .route("/_note", post(note))
}

async fn counted(runs: Runs) -> Json<Value> {
    Json(json!({ "run": runs.add() }))
}

async fn refused(runs: Runs) -> AppError {
    runs.add();
    AppError::Forbidden
}

async fn slow(runs: Runs) -> Json<Value> {
    let run = runs.add();
    tokio::time::sleep(SLOW_FOR).await;
    Json(json!({ "run": run }))
}

/// Writes one row on the request transaction, then fails when the body asks for it.
async fn note(tx: RequestTx, Json(body): Json<Value>) -> Result<Json<Value>, AppError> {
    {
        let mut tx = tx.lock()?;
        sqlx::query("INSERT INTO http_test_notes (note) VALUES ($1)")
            .bind(body["note"].as_str().unwrap_or_default())
            .execute(&mut **tx)
            .await?;
    }
    if body["fail"] == json!(true) {
        return Err(AppError::Forbidden);
    }
    Ok(Json(json!({ "noted": true })))
}

async fn create_notes(pool: &PgPool) {
    sqlx::query("CREATE TABLE http_test_notes (note TEXT NOT NULL)")
        .execute(pool)
        .await
        .unwrap();
}

async fn notes(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar("SELECT note FROM http_test_notes ORDER BY note")
        .fetch_all(pool)
        .await
        .unwrap()
}

async fn ledger_rows(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM admin.idempotency_keys")
        .fetch_one(pool)
        .await
        .unwrap()
}

fn post_as(path: &str, principal: Uuid, key: &str, body: impl Into<Body>) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(path)
        .header(DEV_PRINCIPAL_HEADER, principal.to_string())
        .header(IDEMPOTENCY_KEY_HEADER, key)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body.into())
        .unwrap()
}

fn get(path: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(path)
        .body(Body::empty())
        .unwrap()
}

struct Answer {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl Answer {
    fn json(&self) -> Value {
        serde_json::from_slice(&self.body).unwrap()
    }

    fn code(&self) -> String {
        self.json()["code"].as_str().unwrap_or_default().to_owned()
    }

    fn is_replay(&self) -> bool {
        self.headers
            .get(REPLAY_HEADER)
            .is_some_and(|value| value == "true")
    }
}

async fn send(app: &Router, request: Request<Body>) -> Answer {
    let response = app.clone().oneshot(request).await.unwrap();
    let (parts, body) = response.into_parts();
    Answer {
        status: parts.status,
        headers: parts.headers,
        body: body.collect().await.unwrap().to_bytes(),
    }
}

#[sqlx::test(migrations = "../migrations")]
async fn a_repeated_request_gets_the_first_answer_byte_for_byte(pool: PgPool) {
    let app = build_app(state(&pool));
    let principal = Uuid::now_v7();
    let body = r#"{"amount":"1500.000","currency":"IQD"}"#;

    let first = send(&app, post_as(ECHO, principal, "transfer-0001", body)).await;
    let second = send(&app, post_as(ECHO, principal, "transfer-0001", body)).await;

    assert_eq!(first.status, StatusCode::OK);
    assert!(!first.is_replay());
    assert_eq!(first.json()["echo"]["amount"], "1500.000");

    assert_eq!(second.status, StatusCode::OK);
    assert!(second.is_replay());
    assert_eq!(second.body, first.body);
    assert_eq!(
        second.headers[header::CONTENT_TYPE],
        first.headers[header::CONTENT_TYPE]
    );
    assert_eq!(ledger_rows(&pool).await, 1);
}

#[sqlx::test(migrations = "../migrations")]
async fn a_key_reused_with_another_body_is_refused(pool: PgPool) {
    let app = build_app(state(&pool));
    let principal = Uuid::now_v7();

    let first = send(
        &app,
        post_as(ECHO, principal, "transfer-0002", r#"{"a":1}"#),
    )
    .await;
    let reused = send(
        &app,
        post_as(ECHO, principal, "transfer-0002", r#"{"a":2}"#),
    )
    .await;

    assert_eq!(first.status, StatusCode::OK);
    assert_eq!(reused.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(reused.code(), "E_IDEMPOTENCY_REUSE");
    assert_eq!(
        reused.headers[header::CONTENT_TYPE],
        "application/problem+json"
    );
    assert_eq!(ledger_rows(&pool).await, 1);
}

#[sqlx::test(migrations = "../migrations")]
async fn a_key_reused_on_another_path_is_refused(pool: PgPool) {
    let app = build_app_with(state(&pool), probes(&Runs::default()));
    let principal = Uuid::now_v7();

    let first = send(&app, post_as(COUNT, principal, "shared-key-01", "{}")).await;
    let other_query = send(
        &app,
        post_as("/api/v1/_count?again=1", principal, "shared-key-01", "{}"),
    )
    .await;

    assert_eq!(first.status, StatusCode::OK);
    assert_eq!(other_query.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(other_query.code(), "E_IDEMPOTENCY_REUSE");
}

#[sqlx::test(migrations = "../migrations")]
async fn the_same_key_under_another_principal_runs_on_its_own(pool: PgPool) {
    let app = build_app(state(&pool));
    let body = r#"{"a":1}"#;

    let mine = send(&app, post_as(ECHO, Uuid::now_v7(), "shared-key-02", body)).await;
    let theirs = send(&app, post_as(ECHO, Uuid::now_v7(), "shared-key-02", body)).await;

    assert_eq!(mine.status, StatusCode::OK);
    assert_eq!(theirs.status, StatusCode::OK);
    assert!(!theirs.is_replay());
    assert_ne!(
        mine.json()["execution_id"],
        theirs.json()["execution_id"],
        "each principal ran the handler"
    );
    assert_eq!(ledger_rows(&pool).await, 2);
}

#[sqlx::test(migrations = "../migrations")]
async fn twelve_identical_requests_run_the_handler_once(pool: PgPool) {
    let runs = Runs::default();
    let app = build_app_with(state(&pool), probes(&runs));
    let principal = Uuid::now_v7();

    let mut requests = JoinSet::new();
    for _ in 0..12 {
        let app = app.clone();
        requests.spawn(async move {
            send(
                &app,
                post_as(COUNT, principal, "burst-key-0001", r#"{"n":1}"#),
            )
            .await
        });
    }
    let answers = requests.join_all().await;

    assert_eq!(runs.count(), 1);
    assert_eq!(ledger_rows(&pool).await, 1);
    for answer in &answers {
        assert_eq!(answer.status, StatusCode::OK);
        assert_eq!(answer.body, answers[0].body);
    }
    assert_eq!(answers.iter().filter(|a| a.is_replay()).count(), 11);
}

#[sqlx::test(migrations = "../migrations")]
async fn a_refused_request_leaves_no_row_and_runs_again_on_a_retry(pool: PgPool) {
    let runs = Runs::default();
    let app = build_app_with(state(&pool), probes(&runs));
    let principal = Uuid::now_v7();

    let first = send(&app, post_as(REFUSE, principal, "refused-key-01", "{}")).await;
    assert_eq!(first.status, StatusCode::FORBIDDEN);
    assert_eq!(first.code(), "E_FORBIDDEN");
    assert_eq!(ledger_rows(&pool).await, 0);

    let retry = send(&app, post_as(REFUSE, principal, "refused-key-01", "{}")).await;
    assert_eq!(retry.status, StatusCode::FORBIDDEN);
    assert!(!retry.is_replay());
    assert_eq!(runs.count(), 2);
    assert_eq!(ledger_rows(&pool).await, 0);
}

#[sqlx::test(migrations = "../migrations")]
async fn a_handler_write_commits_and_rolls_back_with_the_claim(pool: PgPool) {
    create_notes(&pool).await;
    let app = build_app_with(state(&pool), probes(&Runs::default()));
    let principal = Uuid::now_v7();

    let kept = send(
        &app,
        post_as(NOTE, principal, "note-key-0001", r#"{"note":"kept"}"#),
    )
    .await;
    assert_eq!(kept.status, StatusCode::OK);
    assert_eq!(notes(&pool).await, ["kept"]);
    assert_eq!(ledger_rows(&pool).await, 1);

    let lost = send(
        &app,
        post_as(
            NOTE,
            principal,
            "note-key-0002",
            r#"{"note":"lost","fail":true}"#,
        ),
    )
    .await;
    assert_eq!(lost.status, StatusCode::FORBIDDEN);
    assert_eq!(notes(&pool).await, ["kept"]);
    assert_eq!(ledger_rows(&pool).await, 1);

    let again = send(
        &app,
        post_as(NOTE, principal, "note-key-0001", r#"{"note":"kept"}"#),
    )
    .await;
    assert!(again.is_replay());
    assert_eq!(notes(&pool).await, ["kept"], "a replay writes nothing");
}

#[sqlx::test(migrations = "../migrations")]
async fn a_request_that_dies_before_its_commit_leaves_nothing_behind(pool: PgPool) {
    create_notes(&pool).await;
    let state = state(&pool);
    let app = build_app_with(state.clone(), probes(&Runs::default()));
    let principal = Uuid::now_v7();
    let request = || post_as(NOTE, principal, "crash-key-0001", r#"{"note":"once"}"#);

    state.faults.die_before_next_commit();
    let died = send(&app, request()).await;
    assert_eq!(died.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(died.code(), "E_INTERNAL");
    assert!(notes(&pool).await.is_empty());
    assert_eq!(ledger_rows(&pool).await, 0);

    let retry = send(&app, request()).await;
    assert_eq!(retry.status, StatusCode::OK);
    assert!(!retry.is_replay());
    assert_eq!(notes(&pool).await, ["once"]);
    assert_eq!(ledger_rows(&pool).await, 1);

    let replay = send(&app, request()).await;
    assert!(replay.is_replay());
    assert_eq!(replay.body, retry.body);
    assert_eq!(notes(&pool).await, ["once"]);
}

#[sqlx::test(migrations = "../migrations")]
async fn a_second_request_does_not_run_while_the_first_holds_the_key(pool: PgPool) {
    sqlx::query(
        "UPDATE admin.settings SET value = '1' WHERE key = 'idempotency.in_flight_lease_s'",
    )
    .execute(&pool)
    .await
    .unwrap();
    let runs = Runs::default();
    let app = build_app_with(state(&pool), probes(&runs));
    let principal = Uuid::now_v7();
    let request = || post_as(SLOW, principal, "slow-key-0001", "{}");

    let first = tokio::spawn({
        let app = app.clone();
        let request = request();
        async move { send(&app, request).await }
    });
    tokio::time::sleep(Duration::from_millis(300)).await;

    let asked = Instant::now();
    let second = send(&app, request()).await;
    assert_eq!(second.status, StatusCode::CONFLICT);
    assert_eq!(second.code(), "E_IDEMPOTENCY_IN_FLIGHT");
    assert!(
        asked.elapsed() < SLOW_FOR,
        "the second request gave up at the lease, before the first finished"
    );

    let first = first.await.unwrap();
    assert_eq!(first.status, StatusCode::OK);
    assert_eq!(runs.count(), 1);

    let third = send(&app, request()).await;
    assert!(third.is_replay());
    assert_eq!(third.body, first.body);
    assert_eq!(runs.count(), 1);
    assert_eq!(ledger_rows(&pool).await, 1);
}

#[sqlx::test(migrations = "../migrations")]
async fn the_stored_answer_cannot_be_read_in_the_table(pool: PgPool) {
    let app = build_app(state(&pool));
    let marker = "plain-marker-7f3a91";
    let body = format!(r#"{{"note":"{marker}"}}"#);

    let answer = send(&app, post_as(ECHO, Uuid::now_v7(), "sealed-key-0001", body)).await;
    assert_eq!(answer.status, StatusCode::OK);

    let (endpoint, status, content_type, stored): (String, i32, String, Vec<u8>) = sqlx::query_as(
        "SELECT endpoint, response_status, response_content_type, response_body
           FROM admin.idempotency_keys",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(endpoint, "POST /api/v1/_echo");
    assert_eq!(status, 200);
    assert_eq!(content_type, "application/json");
    assert!(stored.len() > answer.body.len());
    assert!(
        !stored
            .windows(marker.len())
            .any(|window| window == marker.as_bytes()),
        "the answer is stored sealed"
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn a_mutation_needs_a_principal_and_a_key(pool: PgPool) {
    let app = build_app(state(&pool));
    let principal = Uuid::now_v7().to_string();
    let request = |principal: Option<&str>, key: Option<&str>| {
        let mut request = Request::builder().method(Method::POST).uri(ECHO);
        if let Some(principal) = principal {
            request = request.header(DEV_PRINCIPAL_HEADER, principal);
        }
        if let Some(key) = key {
            request = request.header(IDEMPOTENCY_KEY_HEADER, key);
        }
        request.body(Body::from("{}")).unwrap()
    };

    let nobody = send(&app, request(None, Some("some-key-0001"))).await;
    assert_eq!(nobody.status, StatusCode::UNAUTHORIZED);
    assert_eq!(nobody.code(), "E_UNAUTHENTICATED");

    let not_a_uuid = send(&app, request(Some("somebody"), Some("some-key-0001"))).await;
    assert_eq!(not_a_uuid.status, StatusCode::BAD_REQUEST);
    assert_eq!(not_a_uuid.code(), "E_BAD_REQUEST");

    let no_key = send(&app, request(Some(&principal), None)).await;
    assert_eq!(no_key.status, StatusCode::BAD_REQUEST);
    assert_eq!(no_key.code(), "E_IDEMPOTENCY_REQUIRED");

    let short_key = send(&app, request(Some(&principal), Some("short"))).await;
    assert_eq!(short_key.status, StatusCode::BAD_REQUEST);
    assert_eq!(short_key.code(), "E_BAD_REQUEST");

    assert_eq!(ledger_rows(&pool).await, 0);
}

#[sqlx::test(migrations = "../migrations")]
async fn a_body_that_is_not_a_json_object_is_a_validation_error(pool: PgPool) {
    let app = build_app(state(&pool));

    let answer = send(
        &app,
        post_as(ECHO, Uuid::now_v7(), "invalid-key-01", "[1,2]"),
    )
    .await;

    assert_eq!(answer.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(answer.code(), "E_VALIDATION");
    assert_eq!(answer.json()["errors"][0]["code"], "not_a_json_object");
    assert_eq!(ledger_rows(&pool).await, 0);
}

#[sqlx::test(migrations = "../migrations")]
async fn a_body_over_the_limit_is_refused(pool: PgPool) {
    let app = build_app(state(&pool));
    let too_long = vec![b' '; MAX_BODY_BYTES + 1];

    let answer = send(
        &app,
        post_as(ECHO, Uuid::now_v7(), "large-key-0001", too_long),
    )
    .await;

    assert_eq!(answer.status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(answer.code(), "E_PAYLOAD_TOO_LARGE");
    assert_eq!(
        answer.headers[header::CONTENT_TYPE],
        "application/problem+json"
    );
    assert_eq!(ledger_rows(&pool).await, 0);
}

#[sqlx::test(migrations = "../migrations")]
async fn every_answer_carries_the_correlation_id(pool: PgPool) {
    let app = build_app(state(&pool));
    let sent = Uuid::now_v7();

    let mut named = get("/healthz");
    named
        .headers_mut()
        .insert(CORRELATION_HEADER, sent.to_string().parse().unwrap());
    let named = send(&app, named).await;
    assert_eq!(named.status, StatusCode::OK);
    assert_eq!(named.headers[CORRELATION_HEADER], sent.to_string());

    let unnamed = send(&app, get("/api/v1/nothing-here")).await;
    assert_eq!(unnamed.status, StatusCode::NOT_FOUND);
    assert_eq!(unnamed.code(), "E_NOT_FOUND");
    let given = unnamed.headers[CORRELATION_HEADER].to_str().unwrap();
    assert!(Uuid::parse_str(given).is_ok());
    assert_eq!(unnamed.json()["correlation_id"], given);
}

#[sqlx::test(migrations = "../migrations")]
async fn a_wrong_method_is_a_problem_not_an_empty_answer(pool: PgPool) {
    let app = build_app(state(&pool));

    let answer = send(&app, get(ECHO)).await;

    assert_eq!(answer.status, StatusCode::BAD_REQUEST);
    assert_eq!(answer.code(), "E_BAD_REQUEST");
}

#[sqlx::test(migrations = "../migrations")]
async fn a_development_build_serves_the_document_and_its_viewer(pool: PgPool) {
    let app = build_app(state(&pool));

    let document = send(&app, get("/api/v1/openapi.json")).await;
    assert_eq!(document.status, StatusCode::OK);
    assert!(document.json()["paths"].get(ECHO).is_some());

    let viewer = send(&app, get("/swagger-ui/")).await;
    assert_eq!(viewer.status, StatusCode::OK);
}

#[sqlx::test(migrations = "../migrations")]
async fn a_production_build_serves_no_development_route(pool: PgPool) {
    let state = state_for(&pool, "production", NO_VALKEY);
    let app = build_app(state.clone());
    let principal = Uuid::now_v7();

    let echo = send(&app, post_as(ECHO, principal, "prod-key-0001", "{}")).await;
    assert_eq!(echo.status, StatusCode::NOT_FOUND);
    assert_eq!(echo.code(), "E_NOT_FOUND");

    for path in ["/swagger-ui", "/swagger-ui/", "/metrics"] {
        assert_eq!(
            send(&app, get(path)).await.status,
            StatusCode::NOT_FOUND,
            "{path}"
        );
    }

    let document = send(&app, get("/api/v1/openapi.json")).await;
    assert_eq!(document.status, StatusCode::OK);
    let paths = document.json()["paths"].as_object().unwrap().clone();
    assert!(paths.contains_key("/healthz"));
    assert!(!paths.keys().any(|path| path.contains("_echo")));

    assert_eq!(send(&app, get("/healthz")).await.status, StatusCode::OK);

    // The development header names nobody in production, on any route.
    let runs = Runs::default();
    let app = build_app_with(state, probes(&runs));
    let refused = send(&app, post_as(COUNT, principal, "prod-key-0002", "{}")).await;
    assert_eq!(refused.status, StatusCode::UNAUTHORIZED);
    assert_eq!(refused.code(), "E_UNAUTHENTICATED");
    assert_eq!(runs.count(), 0);
    assert_eq!(ledger_rows(&pool).await, 0);
}

#[sqlx::test(migrations = "../migrations")]
async fn readiness_fails_and_names_valkey_when_it_cannot_be_reached(pool: PgPool) {
    let state = state(&pool);
    valkey::connect(&state.valkey);
    let app = build_app(state);

    let answer = send(&app, get("/readyz")).await;

    assert_eq!(answer.status, StatusCode::SERVICE_UNAVAILABLE);
    let body = answer.json();
    assert_eq!(body["status"], "not_ready");
    let check = |name: &str| {
        body["checks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|check| check["name"] == name)
            .map(|check| check["ok"].clone())
    };
    assert_eq!(check("database"), Some(json!(true)));
    assert_eq!(check("migrations"), Some(json!(true)));
    assert_eq!(check("valkey"), Some(json!(false)));
}

#[sqlx::test(migrations = "../migrations")]
#[ignore = "needs the dev Valkey (make up)"]
async fn readiness_passes_when_every_dependency_answers(pool: PgPool) {
    let state = state_for(&pool, "test", DEV_VALKEY);
    valkey::connect(&state.valkey);
    let app = build_app(state);

    let answer = send(&app, get("/readyz")).await;

    assert_eq!(answer.status, StatusCode::OK);
    let body = answer.json();
    assert_eq!(body["status"], "ready");
    assert_eq!(body["checks"].as_array().unwrap().len(), 3);
}
