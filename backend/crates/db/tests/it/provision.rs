use std::time::Duration;

use akr_db::AKR_SCHEMAS;
use akr_db::check::{
    ALL_PRIVILEGES, DISPLAY_COLLATIONS, ICU_PROVIDER, SORT_EXPECTED, WRITE_PRIVILEGES, arabic_sort,
    display_collations, indexes_on_icu_collations, public_grants, role_attributes, schema_usage,
    table_privileges, tables_not_owned_by, text_round_trip,
};
use akr_db::provision::{
    LoginRole, RoleOutcome, RoleSettings, create_database, database_locale, ensure_admin_schema,
    ensure_login_role, ensure_migrate_role, role_exists,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::support::run;

const PASSWORD: &str = "throwaway-Password-0001";

/// Roles belong to the cluster, not to the test's database, so every test names its own.
fn scratch_name(prefix: &str) -> &'static str {
    let name = format!("{prefix}_{}", &Uuid::now_v7().simple().to_string()[20..]);
    Box::leak(name.into_boxed_str())
}

fn settings(read_only: bool) -> RoleSettings {
    RoleSettings {
        statement_timeout: Duration::from_secs(7),
        idle_in_transaction_session_timeout: Duration::from_secs(11),
        read_only,
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_role_created_here_signs_in_with_its_settings_and_memberships(pool: PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    let role = LoginRole {
        name: scratch_name("akr_t_monitor"),
        settings: settings(true),
        created_here: true,
        member_of: &["pg_monitor"],
    };

    let first = ensure_login_role(&mut conn, &role, PASSWORD).await.unwrap();
    assert_eq!(first, RoleOutcome::Ready { created: true });
    let again = ensure_login_role(&mut conn, &role, PASSWORD).await.unwrap();
    assert_eq!(again, RoleOutcome::Ready { created: false });

    let found = role_attributes(&mut conn, role.name)
        .await
        .unwrap()
        .unwrap();
    assert!(found.can_login);
    assert!(!found.superuser && !found.create_db && !found.create_role);
    assert!(!found.bypass_rls && !found.replication);
    assert_eq!(found.config, role.settings.as_config());
    assert_eq!(found.member_of, ["pg_monitor"]);
    assert!(
        schema_usage(&mut conn, role.name, &AKR_SCHEMAS)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        table_privileges(&mut conn, role.name, &AKR_SCHEMAS, None, &ALL_PRIVILEGES)
            .await
            .unwrap()
            .is_empty()
    );

    let writable = LoginRole {
        settings: settings(false),
        ..role
    };
    ensure_login_role(&mut conn, &writable, PASSWORD)
        .await
        .unwrap();
    let found = role_attributes(&mut conn, role.name)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found.config, writable.settings.as_config());

    run(&pool, &format!("DROP ROLE {}", role.name)).await;
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_role_waits_for_the_migration_that_creates_it_or_its_parent(pool: PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    let from_a_migration = LoginRole {
        name: scratch_name("akr_t_later"),
        settings: settings(false),
        created_here: false,
        member_of: &[],
    };
    assert_eq!(
        ensure_login_role(&mut conn, &from_a_migration, PASSWORD)
            .await
            .unwrap(),
        RoleOutcome::Waiting
    );
    assert!(!role_exists(&mut conn, from_a_migration.name).await.unwrap());

    let parent: &'static [&'static str] = Box::leak(Box::new([scratch_name("akr_t_parent")]));
    let child = LoginRole {
        name: scratch_name("akr_t_child"),
        settings: settings(false),
        created_here: true,
        member_of: parent,
    };
    assert_eq!(
        ensure_login_role(&mut conn, &child, PASSWORD)
            .await
            .unwrap(),
        RoleOutcome::Waiting
    );
    assert!(!role_exists(&mut conn, child.name).await.unwrap());
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_migration_role_may_create_roles_and_resolves_names_in_admin(pool: PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    let role = scratch_name("akr_t_migrate");

    assert!(
        ensure_migrate_role(&mut conn, role, PASSWORD)
            .await
            .unwrap()
    );
    assert!(
        !ensure_migrate_role(&mut conn, role, PASSWORD)
            .await
            .unwrap()
    );

    let found = role_attributes(&mut conn, role).await.unwrap().unwrap();
    assert!(found.can_login && found.create_role);
    assert!(!found.superuser && !found.bypass_rls && !found.replication && !found.create_db);
    assert_eq!(found.config, ["search_path=admin"]);

    ensure_admin_schema(&mut conn, role).await.unwrap();
    let owner: String = sqlx::query_scalar(
        "SELECT pg_get_userbyid(nspowner) FROM pg_namespace WHERE nspname = 'admin'",
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap();
    assert_eq!(owner, role);

    run(&pool, "ALTER SCHEMA admin OWNER TO CURRENT_USER").await;
    run(&pool, &format!("DROP ROLE {role}")).await;
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_created_database_has_the_builtin_c_utf8_locale(pool: PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    let database = scratch_name("akr_t_db");
    let owner: String = sqlx::query_scalar("SELECT current_user::text")
        .fetch_one(&mut *conn)
        .await
        .unwrap();

    assert_eq!(database_locale(&mut conn, database).await.unwrap(), None);
    create_database(&mut conn, database, &owner).await.unwrap();
    let locale = database_locale(&mut conn, database).await.unwrap().unwrap();
    assert!(locale.is_builtin_c_utf8(), "{locale:?}");

    run(&pool, &format!("DROP DATABASE {database}")).await;
}

#[sqlx::test(migrations = "../../migrations")]
async fn arabic_kurdish_and_persian_text_survives_and_sorts_as_expected(pool: PgPool) {
    let mut conn = pool.acquire().await.unwrap();

    let round_trip = text_round_trip(&mut conn).await.unwrap();
    assert_eq!(round_trip.mismatches, Vec::<String>::new());
    assert!(round_trip.through_table);

    assert_eq!(arabic_sort(&mut conn).await.unwrap(), SORT_EXPECTED);

    run_on(&mut conn, "SET default_transaction_read_only = on").await;
    let read_only = text_round_trip(&mut conn).await.unwrap();
    assert_eq!(read_only.mismatches, Vec::<String>::new());
    assert!(!read_only.through_table);
}

async fn run_on(conn: &mut sqlx::PgConnection, sql: &str) {
    sqlx::query(sql).execute(conn).await.unwrap();
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_migrated_schema_passes_the_grant_and_collation_checks(pool: PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    let owner: String = sqlx::query_scalar("SELECT current_user::text")
        .fetch_one(&mut *conn)
        .await
        .unwrap();

    let collations = display_collations(&mut conn).await.unwrap();
    let expected: Vec<_> = DISPLAY_COLLATIONS
        .iter()
        .map(|(name, locale)| {
            (
                (*name).to_owned(),
                ICU_PROVIDER.to_owned(),
                Some((*locale).to_owned()),
            )
        })
        .collect();
    assert_eq!(collations, expected);
    assert_eq!(
        indexes_on_icu_collations(&mut conn, &AKR_SCHEMAS)
            .await
            .unwrap(),
        Vec::<String>::new()
    );
    assert_eq!(
        public_grants(&mut conn, &AKR_SCHEMAS).await.unwrap(),
        Vec::<String>::new()
    );
    assert_eq!(
        tables_not_owned_by(&mut conn, &AKR_SCHEMAS, &owner)
            .await
            .unwrap(),
        Vec::<String>::new()
    );
    assert_eq!(
        table_privileges(
            &mut conn,
            "akr_readonly",
            &AKR_SCHEMAS,
            None,
            &WRITE_PRIVILEGES
        )
        .await
        .unwrap(),
        Vec::<String>::new()
    );
    assert_eq!(
        table_privileges(
            &mut conn,
            "akr_readonly",
            &["admin"],
            Some(&["idempotency_keys", "jobs", "outbox"]),
            &ALL_PRIVILEGES
        )
        .await
        .unwrap(),
        Vec::<String>::new()
    );
    let app = table_privileges(
        &mut conn,
        "akr_app",
        &["admin"],
        Some(&["settings"]),
        &ALL_PRIVILEGES,
    )
    .await
    .unwrap();
    assert_eq!(app, ["admin.settings:SELECT", "admin.settings:UPDATE"]);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_grant_to_public_and_an_index_on_a_display_collation_are_found(pool: PgPool) {
    let mut conn = pool.acquire().await.unwrap();
    run(&pool, "GRANT SELECT ON admin.settings TO PUBLIC").await;
    run(&pool, "GRANT USAGE ON SCHEMA admin TO PUBLIC").await;
    run(
        &pool,
        "CREATE INDEX settings_key_ar ON admin.settings (key COLLATE admin.akr_ar)",
    )
    .await;

    assert_eq!(
        public_grants(&mut conn, &AKR_SCHEMAS).await.unwrap(),
        ["admin.settings:SELECT", "admin:USAGE"]
    );
    assert_eq!(
        indexes_on_icu_collations(&mut conn, &AKR_SCHEMAS)
            .await
            .unwrap(),
        ["admin.settings_key_ar"]
    );
}
