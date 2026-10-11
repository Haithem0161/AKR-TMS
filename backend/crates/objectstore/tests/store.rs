//! The client against a running store. Run with `cargo test -p akr-objectstore -- --ignored` once the
//! development store is up (`make up`).
//!
//! `AKR_TEST_S3_ENDPOINT` names the store (default: the development store on loopback). The credential is
//! the development root credential under `deploy/dev/secrets/`, or `AKR_TEST_S3_ACCESS_KEY_ID` and
//! `AKR_TEST_S3_SECRET_ACCESS_KEY`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;
use std::time::Duration;

use akr_common::config::S3Credential;
use akr_objectstore::conformance::{self, Options};
use akr_objectstore::{LockMode, ObjectStore, ObjectStoreError, S3Client, VersionId};
use bytes::Bytes;
use sha2::{Digest, Sha256};
use url::Url;
use zeroize::Zeroizing;

const PLAIN_BUCKET: &str = "akr-test-plain";

fn dev_secret(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../deploy/dev/secrets")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
        .trim()
        .to_owned()
}

fn client() -> S3Client {
    let endpoint = std::env::var("AKR_TEST_S3_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:9470".to_owned());
    let access_key_id = std::env::var("AKR_TEST_S3_ACCESS_KEY_ID")
        .unwrap_or_else(|_| dev_secret("store_root_user"));
    let secret_access_key = std::env::var("AKR_TEST_S3_SECRET_ACCESS_KEY")
        .unwrap_or_else(|_| dev_secret("store_root_password"));
    S3Client::new(
        &Url::parse(&endpoint).unwrap(),
        S3Credential {
            access_key_id,
            secret_access_key: Zeroizing::new(secret_access_key),
        },
    )
    .unwrap()
}

fn digest(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn unique(name: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("it-{nanos}/{name}")
}

#[tokio::test]
#[ignore = "needs the dev object store (make up)"]
async fn an_object_is_written_read_and_described() {
    let store = client();
    store.create_bucket(PLAIN_BUCKET, false).await.unwrap();
    store.head_bucket(PLAIN_BUCKET).await.unwrap();

    let key = unique("ملف a b+c (1).bin");
    let body = Bytes::from_static(b"synthetic evidence bytes");
    let version = store
        .put(
            PLAIN_BUCKET,
            &key,
            body.clone(),
            digest(&body),
            LockMode::BucketDefault,
        )
        .await
        .unwrap();
    assert_eq!(version.as_str(), VersionId::UNVERSIONED);

    assert_eq!(store.get(PLAIN_BUCKET, &key).await.unwrap(), body);
    let meta = store.head(PLAIN_BUCKET, &key).await.unwrap().unwrap();
    assert_eq!(meta.size, body.len() as u64);
    assert_eq!(meta.sha256, Some(digest(&body)));
    assert_eq!(meta.retention, None);
    assert!(!meta.legal_hold);

    store
        .delete_object(PLAIN_BUCKET, &key, None, false)
        .await
        .unwrap();
    assert_eq!(store.head(PLAIN_BUCKET, &key).await.unwrap(), None);
    assert!(matches!(
        store.get(PLAIN_BUCKET, &key).await,
        Err(ObjectStoreError::NotFound(_))
    ));
}

#[tokio::test]
#[ignore = "needs the dev object store (make up)"]
async fn a_missing_bucket_and_a_wrong_secret_are_told_apart() {
    let store = client();
    assert!(matches!(
        store.head_bucket("akr-test-absent").await,
        Err(ObjectStoreError::NotFound(_))
    ));
    assert!(matches!(
        store
            .put(
                "akr-test-absent",
                "k",
                Bytes::from_static(b"x"),
                digest(b"x"),
                LockMode::BucketDefault
            )
            .await,
        Err(ObjectStoreError::NotFound(_))
    ));

    let endpoint = std::env::var("AKR_TEST_S3_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:9470".to_owned());
    let stranger = S3Client::new(
        &Url::parse(&endpoint).unwrap(),
        S3Credential {
            access_key_id: dev_secret("store_root_user"),
            secret_access_key: Zeroizing::new("not-the-secret".to_owned()),
        },
    )
    .unwrap();
    match stranger.get(PLAIN_BUCKET, "anything").await {
        Err(ObjectStoreError::Refused {
            status: 403, code, ..
        }) => {
            assert_eq!(code, "SignatureDoesNotMatch");
        }
        other => panic!("{other:?}"),
    }
}

#[tokio::test]
#[ignore = "needs the dev object store (make up)"]
async fn versions_are_listed_newest_first() {
    let store = client();
    let options = Options::default();
    store.create_bucket(&options.bucket, true).await.unwrap();

    let key = unique("versions.bin");
    let mut written = Vec::new();
    for body in ["one", "two", "three"] {
        written.push(
            store
                .put(
                    &options.bucket,
                    &key,
                    Bytes::from(body),
                    digest(body.as_bytes()),
                    LockMode::Governance {
                        until: chrono::Utc::now() + chrono::Duration::seconds(5),
                    },
                )
                .await
                .unwrap(),
        );
    }
    let listed = store.list_versions(&options.bucket, &key).await.unwrap();
    assert_eq!(listed.len(), 3);
    assert!(
        listed
            .iter()
            .all(|version| version.key == key && !version.delete_marker)
    );
    assert!(listed[0].is_latest);
    assert_eq!(listed[0].version_id, written[2]);
    assert_eq!(listed[2].version_id, written[0]);
    assert_eq!(listed[2].size, 3);
    assert_eq!(
        store
            .get_version(&options.bucket, &key, &written[0])
            .await
            .unwrap(),
        "one"
    );
}

#[tokio::test]
#[ignore = "needs the dev object store (make up)"]
async fn the_worm_suite_passes_against_the_store_itself() {
    let store = client();
    let options = Options {
        lock_for: Duration::from_secs(60),
        ..Options::default()
    };
    let report = conformance::run(&store, &options).await;
    for check in &report.checks {
        println!(
            "{} {:<32} {}",
            if check.passed { "ok  " } else { "FAIL" },
            check.name,
            check.detail
        );
    }
    assert!(report.passed());
    assert!(report.store_exercised());
    assert_eq!(report.checks.len(), 15);
    assert_eq!(report.manifest.len(), 2);

    let restored = conformance::verify_restore(&store, &report.manifest).await;
    assert!(restored.passed(), "{restored:?}");
}
