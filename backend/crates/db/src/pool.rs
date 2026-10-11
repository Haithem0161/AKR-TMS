//! The connection pool, the embedded migrations and the readiness probes.

use std::time::Duration;

use akr_common::config::{DbSecret, SslMode};
use akr_common::infra::{self, ReadinessCheck};
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgSslMode};

/// The migration chain this build was compiled with.
pub static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

pub fn connect_options(secret: &DbSecret, application: &str) -> PgConnectOptions {
    let options = PgConnectOptions::new_without_pgpass()
        .host(&secret.host)
        .port(secret.port)
        .database(&secret.database)
        .username(&secret.username)
        .password(&secret.password)
        .ssl_mode(ssl_mode(secret.sslmode))
        .application_name(application);
    match &secret.sslrootcert {
        Some(ca) => options.ssl_root_cert(ca),
        None => options,
    }
}

/// Opens the pool of one binary. `max_connections` is a bound on purpose: PostgreSQL serves every connection
/// with a process, and several binaries draw on one server.
pub async fn connect(
    secret: &DbSecret,
    application: &str,
    max_connections: u32,
) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect_with(connect_options(secret, application))
        .await
}

fn ssl_mode(mode: SslMode) -> PgSslMode {
    match mode {
        SslMode::Disable => PgSslMode::Disable,
        SslMode::Allow => PgSslMode::Allow,
        SslMode::Prefer => PgSslMode::Prefer,
        SslMode::Require => PgSslMode::Require,
        SslMode::VerifyCa => PgSslMode::VerifyCa,
        SslMode::VerifyFull => PgSslMode::VerifyFull,
    }
}

/// The readiness check `database`.
pub async fn ping(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query_scalar!("SELECT 1").fetch_one(pool).await?;
    Ok(())
}

/// How the applied migration chain compares with the one this build embeds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationState {
    Current,
    /// The database has not had every migration of this build.
    Behind,
    /// The database is newer than this build: a rollback inside the bundle's compatibility window.
    Ahead,
}

/// The name of the migration ledger, when the database has one. An installation keeps it in `admin` (the
/// search path of akr_migrate); a test database keeps it in the default schema.
async fn ledger(pool: &PgPool) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT coalesce(to_regclass('admin._sqlx_migrations'), to_regclass('_sqlx_migrations'))::text",
    )
    .fetch_one(pool)
    .await
}

/// The versions applied with success, oldest first.
pub async fn applied_versions(pool: &PgPool) -> Result<Vec<i64>, sqlx::Error> {
    match ledger(pool).await? {
        Some(ledger) => {
            sqlx::query_scalar(&format!(
                "SELECT version FROM {ledger} WHERE success ORDER BY version"
            ))
            .fetch_all(pool)
            .await
        }
        None => Ok(Vec::new()),
    }
}

pub async fn migration_state(pool: &PgPool) -> Result<MigrationState, sqlx::Error> {
    let embedded = MIGRATOR
        .iter()
        .filter(|m| !m.migration_type.is_down_migration())
        .map(|m| m.version)
        .max();
    let applied = applied_versions(pool).await?.into_iter().max();
    Ok(match (applied, embedded) {
        (applied, embedded) if applied == embedded => MigrationState::Current,
        (Some(applied), Some(embedded)) if applied > embedded => MigrationState::Ahead,
        (Some(_), None) => MigrationState::Ahead,
        _ => MigrationState::Behind,
    })
}

/// The readiness check `migrations`: behind is not ready; ahead is ready, with a warning.
pub async fn migrations_current(pool: &PgPool) -> Result<bool, sqlx::Error> {
    match migration_state(pool).await? {
        MigrationState::Current => Ok(true),
        MigrationState::Ahead => {
            tracing::warn!("the database schema is newer than this build");
            Ok(true)
        }
        MigrationState::Behind => Ok(false),
    }
}

pub const DATABASE_CHECK: &str = "database";
pub const MIGRATIONS_CHECK: &str = "migrations";

/// The readiness check `database`, as a line of a readiness body.
pub async fn database_check(pool: &PgPool) -> ReadinessCheck {
    infra::check(DATABASE_CHECK, ping(pool)).await
}

/// The readiness check `migrations`, as a line of a readiness body.
pub async fn migrations_check(pool: &PgPool) -> ReadinessCheck {
    infra::check(MIGRATIONS_CHECK, async {
        match migrations_current(pool).await {
            Ok(true) => Ok(()),
            Ok(false) => Err("the database schema is behind this build".to_owned()),
            Err(e) => Err(e.to_string()),
        }
    })
    .await
}
