use std::sync::Arc;

use akr_common::AppError;
use akr_common::flags::FlagsService;
use akr_db::PgFlagsRepository;
use akr_domain::audit::RecordingSink;
use akr_domain::flags::{FlagContext, FlagScope, UpdateFlag};
use akr_domain::text::RoleCode;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::support::{actor, request};

const FLAG: &str = "dev.echo_route";

fn service(pool: &PgPool, audit: Arc<RecordingSink>) -> FlagsService {
    FlagsService::new(Arc::new(PgFlagsRepository::new(pool.clone())), audit)
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_flag_change_is_versioned_scoped_and_recorded(pool: PgPool) {
    let audit = Arc::new(RecordingSink::new());
    let flags = service(&pool, audit.clone());
    let anyone = FlagContext::default();
    assert!(!flags.is_enabled(FLAG, &anyone).await);

    let branch = Uuid::now_v7();
    let mut tx = pool.begin().await.unwrap();
    let after = flags
        .update(
            &mut tx,
            FLAG,
            UpdateFlag {
                enabled: Some(true),
                scope: Some(FlagScope {
                    branches: vec![branch],
                    roles: vec![RoleCode::try_from("mlro".to_owned()).unwrap()],
                }),
            },
            1,
            &actor(),
            &request(),
        )
        .await
        .unwrap();
    tx.commit().await.unwrap();
    flags.invalidate().await;

    assert_eq!(after.version, 2);
    assert!(after.enabled);
    assert!(!flags.is_enabled(FLAG, &anyone).await);
    let in_branch = FlagContext {
        branch_id: Some(branch),
        roles: vec![],
    };
    assert!(flags.is_enabled(FLAG, &in_branch).await);

    assert_eq!(audit.names(), ["flag.updated"]);
    let event = &audit.events()[0];
    assert_eq!(
        event.details["before"].to_json(),
        json!({"enabled": false, "scope": {"branches": [], "roles": []}})
    );
    assert_eq!(
        event.details["after"].to_json(),
        json!({"enabled": true, "scope": {"branches": [branch.to_string()], "roles": ["mlro"]}})
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_field_left_out_keeps_its_value_and_a_stale_version_is_a_conflict(pool: PgPool) {
    let flags = service(&pool, Arc::new(RecordingSink::new()));
    let on = UpdateFlag {
        enabled: Some(true),
        scope: None,
    };

    let mut tx = pool.begin().await.unwrap();
    let after = flags
        .update(&mut tx, FLAG, on.clone(), 1, &actor(), &request())
        .await
        .unwrap();
    tx.commit().await.unwrap();
    assert!(after.enabled);
    assert!(after.scope.is_everyone());

    let mut tx = pool.begin().await.unwrap();
    let stale = flags
        .update(&mut tx, FLAG, on.clone(), 1, &actor(), &request())
        .await
        .unwrap_err();
    assert_eq!(stale, AppError::VersionConflict { current: 2 });
    tx.rollback().await.unwrap();

    let mut tx = pool.begin().await.unwrap();
    let unknown = flags
        .update(&mut tx, "no.such_flag", on, 1, &actor(), &request())
        .await
        .unwrap_err();
    assert_eq!(unknown, AppError::NotFound);
}
