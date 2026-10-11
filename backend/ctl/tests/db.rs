//! What the commands leave in a real PostgreSQL: a password that signs in, and a drill on record.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use akr_common::settings_registry::keys::OPS_LAST_RESTORE_DRILL_AT;
use akr_ctl::operator::{Operator, Trail};
use akr_ctl::ops::{DrillResult, record_restore_drill};
use akr_ctl::scram;
use akr_db::provision::{LoginRole, RoleOutcome, RoleSettings, ensure_login_role};
use akr_domain::audit::{Canon, RecordingSink};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{ConnectOptions, PgPool};

const PASSWORD: &str = "correct-horse-battery-staple-41";

fn at(text: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(text)
        .unwrap()
        .with_timezone(&Utc)
}

fn evidence(name: &str, bytes: &[u8]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("akr-ctl-db-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, bytes).unwrap();
    path
}

fn trail(sink: &Arc<RecordingSink>) -> Trail {
    Trail::new(
        sink.clone(),
        Operator {
            os_user: "restore.operator".to_owned(),
            os_session: "42".to_owned(),
        },
    )
}

async fn drill_setting(pool: &PgPool) -> Value {
    sqlx::query_scalar("SELECT value FROM admin.settings WHERE key = $1")
        .bind(OPS_LAST_RESTORE_DRILL_AT)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test(migrations = false)]
async fn a_password_sent_as_a_verifier_signs_in_as_typed(pool: PgPool) {
    let name: &'static str =
        Box::leak(format!("akr_ctl_scram_{}", std::process::id()).into_boxed_str());
    let role = LoginRole {
        name,
        settings: RoleSettings {
            statement_timeout: Duration::from_secs(5),
            idle_in_transaction_session_timeout: Duration::from_secs(5),
            read_only: false,
        },
        created_here: true,
        member_of: &[],
    };
    let verifier = scram::verifier(PASSWORD).unwrap();
    assert!(verifier.starts_with("SCRAM-SHA-256$4096:"));

    let mut admin = pool.acquire().await.unwrap();
    sqlx::query(&format!("DROP ROLE IF EXISTS {name}"))
        .execute(&mut *admin)
        .await
        .unwrap();
    let outcome = ensure_login_role(&mut admin, &role, &verifier)
        .await
        .unwrap();
    assert_eq!(outcome, RoleOutcome::Ready { created: true });

    let stored: String = sqlx::query_scalar("SELECT rolpassword FROM pg_authid WHERE rolname = $1")
        .bind(name)
        .fetch_one(&mut *admin)
        .await
        .unwrap();
    assert_eq!(stored, verifier, "the server keeps the verifier as sent");

    let as_role = (*pool.connect_options()).clone().username(name);
    let mut signed_in = as_role.clone().password(PASSWORD).connect().await.unwrap();
    let who: String = sqlx::query_scalar("SELECT current_user::text")
        .fetch_one(&mut signed_in)
        .await
        .unwrap();
    assert_eq!(who, name);
    drop(signed_in);

    assert!(
        as_role
            .password("not the password")
            .connect()
            .await
            .is_err()
    );

    sqlx::query(&format!("DROP ROLE {name}"))
        .execute(&mut *admin)
        .await
        .unwrap();
}

#[sqlx::test(migrations = "../migrations")]
async fn a_drill_that_passed_moves_the_date_and_is_on_record(pool: PgPool) {
    let sink = Arc::new(RecordingSink::new());
    let report = evidence(
        "passed.txt",
        b"restored 2026-10-11, 41 minutes, checksums equal\n",
    );

    let lines = record_restore_drill(
        &pool,
        &trail(&sink),
        DrillResult::Passed,
        &report,
        at("2026-10-11T09:30:00Z"),
    )
    .await
    .unwrap();

    assert_eq!(drill_setting(&pool).await, "2026-10-11T09:30:00Z");
    assert!(lines[0].ends_with("passed"));

    let events = sink.events();
    let drill = events
        .iter()
        .find(|event| event.kind.name == "restore_drill.recorded")
        .expect("the drill is not on record");
    assert_eq!(drill.details["result"], Canon::Str("passed".to_owned()));
    assert_eq!(
        drill.details["os_user"],
        Canon::Str("restore.operator".to_owned())
    );
    assert_eq!(drill.details["os_session"], Canon::Str("42".to_owned()));
    let Canon::Str(sha256) = &drill.details["evidence_sha256"] else {
        panic!("the evidence digest is not text");
    };
    assert_eq!(sha256.len(), 64);
    assert!(sink.names().contains(&"setting.updated"));
}

#[sqlx::test(migrations = "../migrations")]
async fn a_drill_that_failed_is_on_record_and_moves_nothing(pool: PgPool) {
    let sink = Arc::new(RecordingSink::new());
    let before = drill_setting(&pool).await;

    record_restore_drill(
        &pool,
        &trail(&sink),
        DrillResult::Failed,
        &evidence(
            "failed.txt",
            b"restore stopped at WAL segment 000000010000000000000042\n",
        ),
        at("2026-10-11T09:30:00Z"),
    )
    .await
    .unwrap();

    assert_eq!(drill_setting(&pool).await, before);
    assert_eq!(sink.names(), ["restore_drill.recorded"]);
    assert_eq!(
        sink.events()[0].details["result"],
        Canon::Str("failed".to_owned())
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn a_drill_without_evidence_is_not_recorded(pool: PgPool) {
    let sink = Arc::new(RecordingSink::new());
    let refused = record_restore_drill(
        &pool,
        &trail(&sink),
        DrillResult::Passed,
        &evidence("empty.txt", b""),
        at("2026-10-11T09:30:00Z"),
    )
    .await;

    assert!(refused.is_err());
    assert!(sink.events().is_empty());
    assert_eq!(drill_setting(&pool).await, Value::Null);
}
