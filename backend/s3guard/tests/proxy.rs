//! The guard as a process sees it: a listener in front of an upstream.
//!
//! The first half runs against a stand-in upstream that reports what reached it. The second half
//! (`--ignored`) runs the product's own client through the guard to the development store.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::net::SocketAddr;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};

use akr_common::config::S3Credential;
use akr_common::infra;
use akr_objectstore::conformance::{self, Options};
use akr_objectstore::{LockMode, ObjectStore, ObjectStoreError, S3Client, buckets};
use akr_s3guard::cidr::parse_list;
use akr_s3guard::{Guard, GuardConfig, register_metrics};
use akr_telemetry::{MetricsHandle, SharedRedactor};
use axum::Router;
use axum::extract::{Request, State};
use axum::response::IntoResponse;
use bytes::Bytes;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use url::Url;
use zeroize::Zeroizing;

const EMPTY: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

fn metrics() -> &'static MetricsHandle {
    static HANDLE: OnceLock<MetricsHandle> = OnceLock::new();
    HANDLE.get_or_init(|| {
        let handle = akr_telemetry::metrics::install(SharedRedactor::new()).unwrap();
        register_metrics();
        handle
    })
}

fn rejected(rule: &str) -> u64 {
    let needle = format!("akr_s3guard_rejected_total{{rule=\"{rule}\"}} ");
    metrics()
        .render()
        .lines()
        .find_map(|line| line.strip_prefix(&needle)?.trim().parse().ok())
        .unwrap_or(0)
}

/// Answers every request with what it received.
async fn echo(State(hits): State<Arc<AtomicUsize>>, request: Request) -> impl IntoResponse {
    hits.fetch_add(1, Ordering::SeqCst);
    let (parts, body) = request.into_parts();
    let header = |name: &str| {
        parts
            .headers
            .get(name)
            .map(|value| value.to_str().unwrap().to_owned())
    };
    let report = json!({
        "method": parts.method.as_str(),
        "path": parts.uri.path(),
        "query": parts.uri.query(),
        "host": header("host"),
        "content_length": header("content-length"),
        "transfer_encoding": header("transfer-encoding"),
        "forwarded_for": header("x-forwarded-for"),
        "real_ip": header("x-real-ip"),
        "authorization": header("authorization"),
        "body": String::from_utf8_lossy(&axum::body::to_bytes(body, 1 << 20).await.unwrap()),
    });
    ([("x-amz-version-id", "v-echo")], axum::Json(report))
}

async fn spawn(router: Router) -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(infra::serve(listener, router, std::future::pending()));
    addr
}

async fn guard_before(upstream: &str, installer: &str, health_path: &str) -> SocketAddr {
    metrics();
    let guard = Guard::new(GuardConfig {
        upstream: Url::parse(upstream).unwrap(),
        installer_cidrs: parse_list(installer).unwrap(),
        health_path: health_path.to_owned(),
    })
    .unwrap();
    spawn(Arc::new(guard).router()).await
}

async fn stand_in() -> (SocketAddr, Arc<AtomicUsize>) {
    let hits = Arc::new(AtomicUsize::new(0));
    let upstream = spawn(Router::new().fallback(echo).with_state(hits.clone())).await;
    (upstream, hits)
}

fn authorization(signed: &str) -> String {
    format!(
        "AWS4-HMAC-SHA256 Credential=akr-writer/20261011/us-east-1/s3/aws4_request, \
         SignedHeaders={signed}, Signature={EMPTY}"
    )
}

/// One request written by hand, so no client library tidies the path first.
async fn raw(addr: SocketAddr, head: &str) -> String {
    let mut stream = TcpStream::connect(addr).await.unwrap();
    stream.write_all(head.as_bytes()).await.unwrap();
    let mut answer = Vec::new();
    stream.read_to_end(&mut answer).await.unwrap();
    String::from_utf8_lossy(&answer).into_owned()
}

#[tokio::test]
async fn a_passed_request_reaches_the_store_as_it_was_sent() {
    let (upstream, hits) = stand_in().await;
    let guard = guard_before(&format!("http://{upstream}"), "", "/health").await;

    let body = "synthetic bytes";
    let answer = reqwest::Client::new()
        .put(format!(
            "http://{guard}/akr-evidence/cases/1/a%20b.pdf?retention=&versionId=v%2B1"
        ))
        .header("host", "s3guard:9000")
        .header(
            "x-amz-content-sha256",
            hex(&Sha256::digest(body.as_bytes())),
        )
        .header("x-amz-date", "20261011T080000Z")
        .header(
            "authorization",
            authorization("host;x-amz-content-sha256;x-amz-date"),
        )
        .header("x-forwarded-for", "203.0.113.9")
        .header("x-real-ip", "203.0.113.9")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(answer.status(), 200);
    assert_eq!(answer.headers()["x-amz-version-id"], "v-echo");
    let seen: Value = answer.json().await.unwrap();

    assert_eq!(seen["method"], "PUT");
    assert_eq!(seen["path"], "/akr-evidence/cases/1/a%20b.pdf");
    assert_eq!(seen["query"], "retention=&versionId=v%2B1");
    assert_eq!(seen["host"], "s3guard:9000");
    assert_eq!(seen["content_length"], "15");
    assert_eq!(seen["transfer_encoding"], Value::Null);
    assert_eq!(seen["forwarded_for"], "127.0.0.1");
    assert_eq!(seen["real_ip"], Value::Null);
    assert_eq!(seen["body"], body);
    assert!(
        seen["authorization"]
            .as_str()
            .unwrap()
            .starts_with("AWS4-HMAC-SHA256")
    );
    assert_eq!(hits.load(Ordering::SeqCst), 1);
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[tokio::test]
async fn a_refused_request_never_reaches_the_store_and_is_counted() {
    let (upstream, hits) = stand_in().await;
    let guard = guard_before(&format!("http://{upstream}"), "", "/health").await;
    let client = reqwest::Client::new();
    let signed = |request: reqwest::RequestBuilder| {
        request
            .header("x-amz-content-sha256", EMPTY)
            .header("x-amz-date", "20261011T080000Z")
            .header(
                "authorization",
                authorization("host;x-amz-content-sha256;x-amz-date"),
            )
    };

    let select_before = rejected("s3_select");
    let answer = signed(client.post(format!(
        "http://{guard}/akr-evidence/a?select&select-type=2"
    )))
    .send()
    .await
    .unwrap();
    assert_eq!(answer.status(), 403);
    assert_eq!(answer.headers()["x-akr-s3guard-rule"], "s3_select");
    assert_eq!(answer.headers()["content-type"], "application/xml");
    assert!(
        answer
            .text()
            .await
            .unwrap()
            .contains("<Code>AccessDenied</Code>")
    );
    assert_eq!(rejected("s3_select"), select_before + 1);

    let internal_before = rejected("store_internal_header");
    let answer = signed(client.put(format!("http://{guard}/akr-evidence/a")))
        .header("x-minio-source-replication-request", "true")
        .body("x")
        .send()
        .await
        .unwrap();
    assert_eq!(answer.status(), 403);
    assert_eq!(
        answer.headers()["x-akr-s3guard-rule"],
        "store_internal_header"
    );
    assert_eq!(rejected("store_internal_header"), internal_before + 1);

    for (request, rule) in [
        (signed(client.get(format!("http://{guard}/minio/admin/v3/info"))), "admin_path"),
        (signed(client.get(format!("http://{guard}/minio/health/live"))), "admin_path"),
        (signed(client.delete(format!("http://{guard}/akr-evidence/a"))), "delete_outside_quarantine"),
        (signed(client.put(format!("http://{guard}/akr-evidence"))), "bucket_write_outside_installer"),
        (signed(client.get(format!("http://{guard}/akr-evidence?list-type=2"))), "operation_not_allowed"),
        (
            client.put(format!("http://{guard}/akr-exports/r.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Signature=ab")),
            "query_credentials_on_write",
        ),
    ] {
        let answer = request.send().await.unwrap();
        assert_eq!(answer.status(), 403, "{rule}");
        assert_eq!(answer.headers()["x-akr-s3guard-rule"], rule);
    }

    for head in [
        "DELETE /akr-quarantine/../akr-evidence/a HTTP/1.1\r\nHost: s3guard:9000\r\nConnection: close\r\n\r\n",
        "DELETE /akr-quarantine/%2e%2e/akr-evidence/a HTTP/1.1\r\nHost: s3guard:9000\r\nConnection: close\r\n\r\n",
        "GET /akr-evidence/a HTTP/1.1\r\nHost: s3guard:9000\r\nConnection: close, authorization\r\n\r\n",
        "GET http://objectstore:9000/akr-evidence/a HTTP/1.1\r\nHost: s3guard:9000\r\nConnection: close\r\n\r\n",
    ] {
        let answer = raw(guard, head).await;
        assert!(answer.starts_with("HTTP/1.1 403"), "{head} -> {answer}");
        assert!(
            answer.contains("x-akr-s3guard-rule: operation_not_allowed"),
            "{answer}"
        );
    }

    assert_eq!(hits.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn the_guard_reports_its_own_health_and_the_stores() {
    let (upstream, hits) = stand_in().await;
    let guard = guard_before(&format!("http://{upstream}"), "", "/health").await;
    let client = reqwest::Client::new();

    let health = client
        .get(format!("http://{guard}/healthz"))
        .send()
        .await
        .unwrap();
    assert_eq!(health.text().await.unwrap(), r#"{"status":"ok"}"#);
    assert_eq!(hits.load(Ordering::SeqCst), 0);

    let ready = client
        .get(format!("http://{guard}/readyz"))
        .send()
        .await
        .unwrap();
    assert_eq!(ready.status(), 200);
    assert_eq!(
        ready.text().await.unwrap(),
        r#"{"status":"ready","checks":[{"name":"upstream","ok":true}]}"#
    );
    assert_eq!(hits.load(Ordering::SeqCst), 1);

    let nobody = TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap()
        .local_addr()
        .unwrap();
    let alone = guard_before(&format!("http://{nobody}"), "", "/health").await;
    let ready = client
        .get(format!("http://{alone}/readyz"))
        .send()
        .await
        .unwrap();
    assert_eq!(ready.status(), 503);
    assert_eq!(
        ready.text().await.unwrap(),
        r#"{"status":"not_ready","checks":[{"name":"upstream","ok":false}]}"#
    );

    let answer = client
        .get(format!("http://{alone}/akr-evidence/a"))
        .send()
        .await
        .unwrap();
    assert_eq!(answer.status(), 503);
    assert!(
        answer
            .text()
            .await
            .unwrap()
            .contains("<Code>ServiceUnavailable</Code>")
    );
}

fn dev_secret(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../deploy/dev/secrets")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
        .trim()
        .to_owned()
}

fn store_endpoint() -> String {
    std::env::var("AKR_TEST_S3_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:9470".to_owned())
}

fn client_through(guard: SocketAddr) -> S3Client {
    S3Client::new(
        &Url::parse(&format!("http://{guard}")).unwrap(),
        S3Credential {
            access_key_id: dev_secret("store_root_user"),
            secret_access_key: Zeroizing::new(dev_secret("store_root_password")),
        },
    )
    .unwrap()
}

fn digest(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

#[tokio::test]
#[ignore = "needs the dev object store (make up)"]
async fn the_products_client_works_through_the_guard() {
    let installer = guard_before(&store_endpoint(), "127.0.0.1", "/minio/health/live").await;
    let application = guard_before(&store_endpoint(), "", "/minio/health/live").await;

    let ready = reqwest::get(format!("http://{application}/readyz"))
        .await
        .unwrap();
    assert_eq!(ready.status(), 200);

    let as_app = client_through(application);
    assert!(matches!(
        as_app.create_bucket(buckets::QUARANTINE, false).await,
        Err(ObjectStoreError::Refused { status: 403, code, .. })
            if code == "s3guard:bucket_write_outside_installer"
    ));
    let as_installer = client_through(installer);
    as_installer
        .create_bucket(buckets::QUARANTINE, false)
        .await
        .unwrap();
    as_installer
        .create_bucket("akr-test-guarded", true)
        .await
        .unwrap();
    assert!(
        as_installer
            .object_lock("akr-test-guarded")
            .await
            .unwrap()
            .enabled
    );

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let key = format!("staging/{nanos}/ملف a b.bin");
    let body = Bytes::from_static(b"synthetic bytes through the guard");
    as_app.head_bucket(buckets::QUARANTINE).await.unwrap();
    as_app
        .put(
            buckets::QUARANTINE,
            &key,
            body.clone(),
            digest(&body),
            LockMode::BucketDefault,
        )
        .await
        .unwrap();
    assert_eq!(as_app.get(buckets::QUARANTINE, &key).await.unwrap(), body);
    let meta = as_app
        .head(buckets::QUARANTINE, &key)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(meta.size, body.len() as u64);
    assert_eq!(meta.sha256, Some(digest(&body)));
    let listed = as_app
        .list_versions(buckets::QUARANTINE, &format!("staging/{nanos}/"))
        .await
        .unwrap();
    assert_eq!(listed.len(), 1);

    let url = as_app
        .presign_get(
            buckets::QUARANTINE,
            &key,
            std::time::Duration::from_secs(60),
        )
        .unwrap();
    assert_eq!(
        reqwest::get(url).await.unwrap().bytes().await.unwrap(),
        body
    );

    as_app
        .delete_object(buckets::QUARANTINE, &key, None, false)
        .await
        .unwrap();
    assert_eq!(as_app.head(buckets::QUARANTINE, &key).await.unwrap(), None);

    let locked = format!("guarded/{nanos}.bin");
    let version = as_app
        .put(
            "akr-test-guarded",
            &locked,
            body.clone(),
            digest(&body),
            LockMode::Governance {
                until: chrono::Utc::now() + chrono::Duration::seconds(5),
            },
        )
        .await
        .unwrap();
    assert!(matches!(
        as_app.delete_object("akr-test-guarded", &locked, Some(&version), false).await,
        Err(ObjectStoreError::Refused { status: 403, code, .. })
            if code == "s3guard:delete_outside_quarantine"
    ));
    assert!(
        as_app
            .retention("akr-test-guarded", &locked, Some(&version))
            .await
            .unwrap()
            .is_some()
    );
}

#[tokio::test]
#[ignore = "needs the dev object store (make up)"]
async fn the_worm_suite_through_the_guard_says_who_answered() {
    let installer = guard_before(&store_endpoint(), "127.0.0.1", "/minio/health/live").await;
    let report = conformance::run(&client_through(installer), &Options::default()).await;
    for check in &report.checks {
        println!(
            "{} {:<32} {}",
            if check.passed { "ok  " } else { "FAIL" },
            check.name,
            check.detail
        );
    }
    assert!(report.passed());
    assert!(!report.store_exercised());
    let by_guard: Vec<&str> = report
        .checks
        .iter()
        .filter(|check| check.answered_by_guard)
        .map(|check| check.name)
        .collect();
    assert_eq!(
        by_guard,
        ["locked_version_delete_refused", "bucket_deletion_refused"]
    );
}
