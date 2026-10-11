use std::sync::Arc;

use akr_common::AppError;
use akr_common::settings::SettingsService;
use akr_common::settings_registry::{REGISTRY, keys};
use akr_db::PgSettingsRepository;
use akr_domain::audit::RecordingSink;
use akr_domain::repositories::SettingsRepository;
use akr_domain::settings::ChangeSource;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::support::{actor, request, run, settings};

#[sqlx::test(migrations = "../../migrations")]
async fn the_registry_and_the_seeded_rows_agree(pool: PgPool) {
    let rows = PgSettingsRepository::new(pool).list().await.unwrap();
    let seeded: Vec<&str> = rows.iter().map(|r| r.key.as_str()).collect();
    let mut declared: Vec<&str> = REGISTRY.iter().map(|d| d.key).collect();
    declared.sort_unstable();
    assert_eq!(seeded, declared);

    for row in &rows {
        let def = REGISTRY.iter().find(|d| d.key == row.key).unwrap();
        assert_eq!(row.value_type, def.value_type, "{}", row.key);
        assert_eq!(row.governed, def.governed, "{}", row.key);
        assert_eq!(row.value, def.default_value(), "{}", row.key);
        assert_eq!(row.version, 1, "{}", row.key);
        assert!(def.validate(&row.value).is_ok(), "{}", row.key);
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_change_writes_the_value_adds_one_to_the_version_and_records_the_event(pool: PgPool) {
    let audit = Arc::new(RecordingSink::new());
    let service = settings(&pool, audit.clone());
    assert_eq!(service.idempotency_ttl_days().await, 7);

    let who = actor();
    let mut tx = pool.begin().await.unwrap();
    let after = service
        .apply(
            &mut tx,
            keys::IDEMPOTENCY_TTL_DAYS,
            json!(14),
            1,
            &who,
            &request(),
            ChangeSource::Direct,
        )
        .await
        .unwrap();
    tx.commit().await.unwrap();

    assert_eq!(after.value, json!(14));
    assert_eq!(after.version, 2);
    assert_eq!(after.updated_by, who.id());
    assert_eq!(audit.names(), ["setting.updated"]);
    let event = &audit.events()[0];
    assert_eq!(event.details["before"].to_json(), json!(7));
    assert_eq!(event.details["after"].to_json(), json!(14));
    assert_eq!(event.details["source"].to_json(), json!({"type": "direct"}));

    assert_eq!(
        service.idempotency_ttl_days().await,
        7,
        "the cached value until the change is announced"
    );
    service.notify_changed(keys::IDEMPOTENCY_TTL_DAYS).await;
    assert_eq!(service.idempotency_ttl_days().await, 14);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_value_outside_its_floor_is_refused(pool: PgPool) {
    run(
        &pool,
        r#"UPDATE admin.settings SET floor = '{"min": 5, "basis": "test rule"}', governed = TRUE
           WHERE key = 'idempotency.ttl_days'"#,
    )
    .await;
    let audit = Arc::new(RecordingSink::new());
    let service = settings(&pool, audit.clone());
    let source = ChangeSource::ChangeRequest(Uuid::now_v7());

    let mut tx = pool.begin().await.unwrap();
    let refused = service
        .apply(
            &mut tx,
            keys::IDEMPOTENCY_TTL_DAYS,
            json!(3),
            1,
            &actor(),
            &request(),
            source,
        )
        .await;
    assert_eq!(refused.unwrap_err(), AppError::BelowFloor);
    assert_eq!(AppError::BelowFloor.code(), "E_BELOW_FLOOR");
    tx.rollback().await.unwrap();

    let mut tx = pool.begin().await.unwrap();
    let allowed = service
        .apply(
            &mut tx,
            keys::IDEMPOTENCY_TTL_DAYS,
            json!(5),
            1,
            &actor(),
            &request(),
            source,
        )
        .await
        .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(allowed.value, json!(5));
    assert_eq!(audit.names(), ["setting.updated"]);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_direct_change_of_a_governed_key_is_refused(pool: PgPool) {
    let audit = Arc::new(RecordingSink::new());
    let service = settings(&pool, audit.clone());

    let mut tx = pool.begin().await.unwrap();
    let refused = service
        .apply(
            &mut tx,
            keys::BANK_TIMEZONE,
            json!("Asia/Riyadh"),
            1,
            &actor(),
            &request(),
            ChangeSource::Direct,
        )
        .await
        .unwrap_err();
    tx.rollback().await.unwrap();
    match refused {
        AppError::NotAllowedInState(reason) => assert_eq!(reason.code, "governed"),
        other => panic!("expected a refusal, got {other:?}"),
    }
    assert!(audit.names().is_empty());

    let mut tx = pool.begin().await.unwrap();
    let through_a_change_request = service
        .apply(
            &mut tx,
            keys::BANK_TIMEZONE,
            json!("Asia/Riyadh"),
            1,
            &actor(),
            &request(),
            ChangeSource::ChangeRequest(Uuid::now_v7()),
        )
        .await
        .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(through_a_change_request.version, 2);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_stale_version_is_a_conflict_that_names_the_current_one(pool: PgPool) {
    let service = settings(&pool, Arc::new(RecordingSink::new()));
    for (expected_version, value) in [(1, 10), (2, 11)] {
        let mut tx = pool.begin().await.unwrap();
        service
            .apply(
                &mut tx,
                keys::IDEMPOTENCY_TTL_DAYS,
                json!(value),
                expected_version,
                &actor(),
                &request(),
                ChangeSource::Direct,
            )
            .await
            .unwrap();
        tx.commit().await.unwrap();
    }

    let mut tx = pool.begin().await.unwrap();
    let stale = service
        .apply(
            &mut tx,
            keys::IDEMPOTENCY_TTL_DAYS,
            json!(12),
            1,
            &actor(),
            &request(),
            ChangeSource::Direct,
        )
        .await
        .unwrap_err();
    assert_eq!(stale, AppError::VersionConflict { current: 3 });
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_value_of_the_wrong_type_or_outside_its_rule_is_a_validation_error(pool: PgPool) {
    let service = settings(&pool, Arc::new(RecordingSink::new()));
    for value in [json!("seven"), json!(0), json!(91)] {
        let mut tx = pool.begin().await.unwrap();
        let refused = service
            .apply(
                &mut tx,
                keys::IDEMPOTENCY_TTL_DAYS,
                value.clone(),
                1,
                &actor(),
                &request(),
                ChangeSource::Direct,
            )
            .await
            .unwrap_err();
        assert!(
            matches!(refused, AppError::Validation(_)),
            "{value}: {refused:?}"
        );
    }
    let mut tx = pool.begin().await.unwrap();
    let unknown = service
        .apply(
            &mut tx,
            "no.such_key",
            json!(1),
            1,
            &actor(),
            &request(),
            ChangeSource::Direct,
        )
        .await
        .unwrap_err();
    assert_eq!(unknown, AppError::NotFound);
}

#[sqlx::test(migrations = "../../migrations")]
async fn only_the_command_line_writes_the_drill_date_directly(pool: PgPool) {
    let audit = Arc::new(RecordingSink::new());
    let repo = Arc::new(PgSettingsRepository::new(pool.clone()));
    let drill = json!("2026-10-11T08:00:00Z");

    let server = settings(&pool, audit.clone());
    let mut tx = pool.begin().await.unwrap();
    let refused = server
        .apply(
            &mut tx,
            keys::OPS_LAST_RESTORE_DRILL_AT,
            drill.clone(),
            1,
            &actor(),
            &request(),
            ChangeSource::Direct,
        )
        .await;
    assert!(matches!(refused, Err(AppError::NotAllowedInState(_))));
    tx.rollback().await.unwrap();

    let cli = SettingsService::for_cli(repo, audit.clone());
    let mut tx = pool.begin().await.unwrap();
    cli.apply(
        &mut tx,
        keys::OPS_LAST_RESTORE_DRILL_AT,
        drill,
        1,
        &actor(),
        &request(),
        ChangeSource::Direct,
    )
    .await
    .unwrap();
    let still_governed = cli
        .apply(
            &mut tx,
            keys::OPS_BACKUP_MAX_AGE_H,
            json!(48),
            1,
            &actor(),
            &request(),
            ChangeSource::Direct,
        )
        .await;
    assert!(matches!(
        still_governed,
        Err(AppError::NotAllowedInState(_))
    ));
    tx.commit().await.unwrap();

    assert!(cli.ops_last_restore_drill_at().await.is_some());
    assert_eq!(audit.names(), ["setting.updated"]);
}
