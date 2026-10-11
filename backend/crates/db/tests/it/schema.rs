//! Checks on the schema itself, run after the migrations. Every later migration is held to them.

use akr_db::{
    AKR_SCHEMAS, MigrationState, PgClassificationRepository, PgRetentionAssignmentRepository,
    migration_state, migrations_current, ping,
};
use akr_domain::classification::Classification;
use akr_domain::repositories::{ClassificationRepository, RetentionAssignmentRepository};
use akr_domain::retention::{RetentionClass, RetentionClock};
use sqlx::PgPool;

fn schemas() -> Vec<String> {
    AKR_SCHEMAS.iter().map(|s| (*s).to_owned()).collect()
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_twelve_schemas_exist(pool: PgPool) {
    let found: Vec<String> = sqlx::query_scalar(
        "SELECT nspname::text FROM pg_catalog.pg_namespace WHERE nspname = ANY($1) ORDER BY 1",
    )
    .bind(schemas())
    .fetch_all(&pool)
    .await
    .unwrap();
    let mut expected = schemas();
    expected.sort_unstable();
    assert_eq!(found, expected);
}

#[sqlx::test(migrations = "../../migrations")]
async fn every_column_has_a_classification(pool: PgPool) {
    let repo = PgClassificationRepository::new(pool.clone());
    assert_eq!(repo.uncovered_columns().await.unwrap(), []);

    let ledger = repo.for_table("admin", "idempotency_keys").await.unwrap();
    let body = ledger
        .iter()
        .find(|c| c.column_name == "response_body")
        .unwrap();
    assert_eq!(body.classification, Classification::SecretStr);
    assert!(body.encrypted);
    assert!(repo.list().await.unwrap().len() > ledger.len());

    sqlx::query("ALTER TABLE admin.timers ADD COLUMN test_extra TEXT")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        repo.uncovered_columns().await.unwrap(),
        [(
            "admin".to_owned(),
            "timers".to_owned(),
            "test_extra".to_owned()
        )]
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn every_table_has_a_retention_class_and_a_partition_needs_none(pool: PgPool) {
    let repo = PgRetentionAssignmentRepository::new(pool.clone());
    assert_eq!(repo.uncovered_tables().await.unwrap(), []);

    let rows = repo.list().await.unwrap();
    let jobs = rows.iter().find(|r| r.table_name == "jobs").unwrap();
    assert_eq!(jobs.retention_class, RetentionClass::R12);
    assert_eq!(jobs.clock, RetentionClock::Age);
    assert_eq!(jobs.clock_column.as_deref(), Some("finished_at"));

    sqlx::query(
        "CREATE TABLE audit.test_events (id UUID NOT NULL, occurred_at TIMESTAMPTZ NOT NULL)
         PARTITION BY RANGE (occurred_at)",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "CREATE TABLE audit.test_events_2026_10 PARTITION OF audit.test_events
         FOR VALUES FROM ('2026-10-01') TO ('2026-11-01')",
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        repo.uncovered_tables().await.unwrap(),
        [("audit".to_owned(), "test_events".to_owned())],
        "the parent is held to the rule, its partition is not"
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn no_column_is_a_timestamp_without_time_zone(pool: PgPool) {
    let found: Vec<String> = sqlx::query_scalar(
        "SELECT table_schema || '.' || table_name || '.' || column_name
         FROM information_schema.columns
         WHERE table_schema = ANY($1) AND data_type = 'timestamp without time zone'",
    )
    .bind(schemas())
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(found, Vec::<String>::new());
}

#[sqlx::test(migrations = "../../migrations")]
async fn no_id_column_has_a_database_default(pool: PgPool) {
    let found: Vec<String> = sqlx::query_scalar(
        "SELECT table_schema || '.' || table_name
         FROM information_schema.columns
         WHERE table_schema = ANY($1) AND column_name = 'id' AND column_default IS NOT NULL",
    )
    .bind(schemas())
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(found, Vec::<String>::new());
}

#[sqlx::test(migrations = "../../migrations")]
async fn no_schema_table_or_column_name_is_a_reserved_word(pool: PgPool) {
    let found: Vec<String> = sqlx::query_scalar(
        "WITH names AS (
             SELECT nspname::text AS name FROM pg_catalog.pg_namespace WHERE nspname = ANY($1)
             UNION
             SELECT table_name::text FROM information_schema.tables WHERE table_schema = ANY($1)
             UNION
             SELECT column_name::text FROM information_schema.columns WHERE table_schema = ANY($1)
         )
         SELECT name FROM names JOIN pg_get_keywords() k ON k.word = name WHERE k.catcode = 'R'
         ORDER BY 1",
    )
    .bind(schemas())
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(found, Vec::<String>::new());
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_read_only_role_has_no_privilege_on_the_operational_tables(pool: PgPool) {
    let held: Vec<String> = sqlx::query_scalar(
        "SELECT t || ':' || p
         FROM unnest(ARRAY['admin.idempotency_keys', 'admin.jobs', 'admin.outbox', 'admin.settings']) AS t,
              unnest(ARRAY['SELECT', 'INSERT', 'UPDATE', 'DELETE', 'TRUNCATE', 'REFERENCES', 'TRIGGER']) AS p
         WHERE has_table_privilege('akr_readonly', t, p)",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(held, Vec::<String>::new());

    let may_read_the_registry: bool = sqlx::query_scalar(
        "SELECT has_table_privilege('akr_readonly', 'admin.column_classification', 'SELECT')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(may_read_the_registry);
}

#[sqlx::test(migrations = "../../migrations")]
async fn every_binary_role_may_read_the_classification_register(pool: PgPool) {
    let refused: Vec<String> = sqlx::query_scalar(
        "SELECT r FROM unnest(ARRAY['akr_app', 'akr_ingest', 'akr_assess']) AS r
         WHERE NOT has_table_privilege(r, 'admin.column_classification', 'SELECT')",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(refused, Vec::<String>::new());

    let assess_reads_retention: bool = sqlx::query_scalar(
        "SELECT has_table_privilege('akr_assess', 'admin.retention_assignment', 'SELECT')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(!assess_reads_retention);
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_application_role_cannot_change_or_delete_a_data_key_or_insert_a_setting(pool: PgPool) {
    let held: Vec<String> = sqlx::query_scalar(
        "SELECT t || ':' || p
         FROM (VALUES ('admin.data_keys', 'UPDATE'), ('admin.data_keys', 'DELETE'),
                      ('admin.settings', 'INSERT'), ('admin.settings', 'DELETE'),
                      ('admin.feature_flags', 'INSERT'), ('admin.feature_flags', 'DELETE')) AS v(t, p)
         WHERE has_table_privilege('akr_app', t, p)",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(held, Vec::<String>::new());
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_display_collations_sort_digits_as_numbers(pool: PgPool) {
    let sorted: Vec<String> = sqlx::query_scalar(
        "SELECT v FROM unnest(ARRAY['فرع 10', 'فرع 2', 'فرع 1']) AS v ORDER BY v COLLATE admin.akr_ar",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(sorted, ["فرع 1", "فرع 2", "فرع 10"]);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_database_with_every_migration_is_current_and_one_behind_is_not(pool: PgPool) {
    ping(&pool).await.unwrap();
    assert_eq!(
        migration_state(&pool).await.unwrap(),
        MigrationState::Current
    );
    assert!(migrations_current(&pool).await.unwrap());

    sqlx::query(
        "DELETE FROM _sqlx_migrations WHERE version = (SELECT max(version) FROM _sqlx_migrations)",
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        migration_state(&pool).await.unwrap(),
        MigrationState::Behind
    );
    assert!(!migrations_current(&pool).await.unwrap());

    sqlx::query(
        "INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time)
         VALUES (9999, 'from a newer build', TRUE, '\\x00', 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(migration_state(&pool).await.unwrap(), MigrationState::Ahead);
    assert!(migrations_current(&pool).await.unwrap());
}

#[sqlx::test]
async fn a_database_with_no_migrations_is_behind(pool: PgPool) {
    assert_eq!(
        migration_state(&pool).await.unwrap(),
        MigrationState::Behind
    );
}
