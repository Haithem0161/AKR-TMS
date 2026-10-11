//! Cluster-level provisioning (DR-22): the database, the login roles, their passwords and their settings.
//!
//! None of this is a migration. Roles, passwords and role settings belong to the cluster, and the first
//! migration needs the database, its owner and the `admin` schema to be there already. `akr-ctl db provision`
//! calls these functions over a superuser connection. Each one can run again on an installation that has its
//! result already.
//!
//! PostgreSQL prepares no utility statement, so these are the statements the build cannot check against a
//! schema. The server builds each one with `format()` (`%I` quotes a name, `%L` a literal); this code splices
//! no name and no value into SQL text.

use std::time::Duration;

use sqlx::PgConnection;

pub const ENCODING: &str = "UTF8";
/// Code-point order. It does not move with an ICU or glibc upgrade, so an operating-system update corrupts no
/// index (DR-26).
pub const BUILTIN_LOCALE: &str = "C.UTF-8";
pub const MIGRATE_ROLE: &str = "akr_migrate";
pub const ADMIN_SCHEMA: &str = "admin";

/// `pg_database.datlocprovider` of the builtin provider.
const BUILTIN_PROVIDER: &str = "b";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseLocale {
    pub encoding: String,
    pub provider: String,
    pub locale: Option<String>,
}

impl DatabaseLocale {
    pub fn is_builtin_c_utf8(&self) -> bool {
        self.encoding == ENCODING
            && self.provider == BUILTIN_PROVIDER
            && self.locale.as_deref() == Some(BUILTIN_LOCALE)
    }
}

/// The settings a role carries on every session it opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleSettings {
    pub statement_timeout: Duration,
    pub idle_in_transaction_session_timeout: Duration,
    pub read_only: bool,
}

impl RoleSettings {
    /// The entries of `pg_roles.rolconfig` these settings produce.
    pub fn as_config(&self) -> Vec<String> {
        let mut config = vec![
            format!("statement_timeout={}", seconds(self.statement_timeout)),
            format!(
                "idle_in_transaction_session_timeout={}",
                seconds(self.idle_in_transaction_session_timeout)
            ),
        ];
        if self.read_only {
            config.push("default_transaction_read_only=on".to_owned());
        }
        config
    }
}

/// A role a process signs in as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoginRole {
    pub name: &'static str,
    pub settings: RoleSettings,
    /// `false`: a migration creates the role with its grants, and provisioning only lets it sign in.
    pub created_here: bool,
    /// The roles a role created here is a member of. They are its only privileges.
    pub member_of: &'static [&'static str],
}

const fn settings(statement_s: u64, idle_s: u64, read_only: bool) -> RoleSettings {
    RoleSettings {
        statement_timeout: Duration::from_secs(statement_s),
        idle_in_transaction_session_timeout: Duration::from_secs(idle_s),
        read_only,
    }
}

/// Every role that signs in, except the migration role. `akr_audit_writer` is absent on purpose: no process
/// signs in as it.
pub static LOGIN_ROLES: &[LoginRole] = &[
    LoginRole {
        name: "akr_app",
        settings: settings(300, 120, false),
        created_here: false,
        member_of: &[],
    },
    LoginRole {
        name: "akr_ingest",
        settings: settings(1800, 300, false),
        created_here: false,
        member_of: &[],
    },
    LoginRole {
        name: "akr_assess",
        settings: settings(2, 10, false),
        created_here: false,
        member_of: &[],
    },
    LoginRole {
        name: "akr_readonly",
        settings: settings(60, 60, true),
        created_here: false,
        member_of: &[],
    },
    LoginRole {
        name: "akr_monitor",
        settings: settings(10, 10, false),
        created_here: true,
        member_of: &["pg_monitor"],
    },
    LoginRole {
        name: "akr_retention_login",
        settings: settings(300, 120, false),
        created_here: true,
        member_of: &["akr_retention", "akr_audit_writer"],
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleOutcome {
    /// The role signs in with the given password and carries its settings.
    Ready { created: bool },
    /// A migration that has not run yet creates the role, or a role it is to be a member of.
    Waiting,
}

fn seconds(duration: Duration) -> String {
    format!("{}s", duration.as_secs())
}

async fn utility(
    conn: &mut PgConnection,
    template: &str,
    args: &[&str],
) -> Result<(), sqlx::Error> {
    let statement: String = sqlx::query_scalar("SELECT format($1, VARIADIC $2::text[])")
        .bind(template)
        .bind(args)
        .fetch_one(&mut *conn)
        .await?;
    sqlx::raw_sql(&statement).execute(conn).await?;
    Ok(())
}

pub async fn role_exists(conn: &mut PgConnection, role: &str) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = $1)")
        .bind(role)
        .fetch_one(conn)
        .await
}

pub async fn database_locale(
    conn: &mut PgConnection,
    database: &str,
) -> Result<Option<DatabaseLocale>, sqlx::Error> {
    let row: Option<(String, String, Option<String>)> = sqlx::query_as(
        "SELECT pg_encoding_to_char(encoding)::text, datlocprovider::text, datlocale
         FROM pg_database WHERE datname = $1",
    )
    .bind(database)
    .fetch_optional(conn)
    .await?;
    Ok(row.map(|(encoding, provider, locale)| DatabaseLocale {
        encoding,
        provider,
        locale,
    }))
}

/// Creates the migration role, or brings an existing one to the same state: it signs in with `verifier`, may
/// create roles (migration 0001 creates the NOLOGIN roles) and resolves names in `admin`, so the migration
/// ledger lives there. `true` when the role was created.
pub async fn ensure_migrate_role(
    conn: &mut PgConnection,
    role: &str,
    verifier: &str,
) -> Result<bool, sqlx::Error> {
    let created = !role_exists(conn, role).await?;
    let template = if created {
        "CREATE ROLE %I LOGIN CREATEROLE PASSWORD %L"
    } else {
        "ALTER ROLE %I LOGIN CREATEROLE PASSWORD %L"
    };
    utility(conn, template, &[role, verifier]).await?;
    utility(
        conn,
        "ALTER ROLE %I SET search_path = %I",
        &[role, ADMIN_SCHEMA],
    )
    .await?;
    Ok(created)
}

/// Creates the database with the builtin `C.UTF-8` locale. The caller checks first that it is absent: an
/// existing database keeps the locale it was created with.
pub async fn create_database(
    conn: &mut PgConnection,
    database: &str,
    owner: &str,
) -> Result<(), sqlx::Error> {
    utility(
        conn,
        "CREATE DATABASE %I OWNER %I TEMPLATE template0 ENCODING %L LOCALE_PROVIDER builtin BUILTIN_LOCALE %L",
        &[database, owner, ENCODING, BUILTIN_LOCALE],
    )
    .await
}

pub async fn set_database_owner(
    conn: &mut PgConnection,
    database: &str,
    owner: &str,
) -> Result<(), sqlx::Error> {
    utility(conn, "ALTER DATABASE %I OWNER TO %I", &[database, owner]).await
}

/// Run on a connection to the product's database: the schema the migration ledger is created in.
pub async fn ensure_admin_schema(conn: &mut PgConnection, owner: &str) -> Result<(), sqlx::Error> {
    utility(
        conn,
        "CREATE SCHEMA IF NOT EXISTS %I AUTHORIZATION %I",
        &[ADMIN_SCHEMA, owner],
    )
    .await?;
    utility(conn, "ALTER SCHEMA %I OWNER TO %I", &[ADMIN_SCHEMA, owner]).await
}

/// Lets `role` sign in with `verifier` and gives it its settings and memberships.
pub async fn ensure_login_role(
    conn: &mut PgConnection,
    role: &LoginRole,
    verifier: &str,
) -> Result<RoleOutcome, sqlx::Error> {
    for parent in role.member_of {
        if !role_exists(conn, parent).await? {
            return Ok(RoleOutcome::Waiting);
        }
    }
    let exists = role_exists(conn, role.name).await?;
    if !exists && !role.created_here {
        return Ok(RoleOutcome::Waiting);
    }
    if !exists {
        utility(conn, "CREATE ROLE %I NOLOGIN", &[role.name]).await?;
    }
    for parent in role.member_of {
        utility(conn, "GRANT %I TO %I", &[parent, role.name]).await?;
    }
    utility(
        conn,
        "ALTER ROLE %I LOGIN PASSWORD %L",
        &[role.name, verifier],
    )
    .await?;
    apply_settings(conn, role.name, &role.settings).await?;
    Ok(RoleOutcome::Ready { created: !exists })
}

pub async fn apply_settings(
    conn: &mut PgConnection,
    role: &str,
    settings: &RoleSettings,
) -> Result<(), sqlx::Error> {
    utility(
        conn,
        "ALTER ROLE %I SET statement_timeout = %L",
        &[role, &seconds(settings.statement_timeout)],
    )
    .await?;
    utility(
        conn,
        "ALTER ROLE %I SET idle_in_transaction_session_timeout = %L",
        &[role, &seconds(settings.idle_in_transaction_session_timeout)],
    )
    .await?;
    let read_only = if settings.read_only {
        "ALTER ROLE %I SET default_transaction_read_only = on"
    } else {
        "ALTER ROLE %I RESET default_transaction_read_only"
    };
    utility(conn, read_only, &[role]).await
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn every_login_role_is_named_once_and_has_both_timeouts() {
        let mut seen = HashSet::new();
        for role in LOGIN_ROLES {
            assert!(seen.insert(role.name), "{} is listed twice", role.name);
            assert!(role.settings.statement_timeout > Duration::ZERO);
            assert!(role.settings.idle_in_transaction_session_timeout > Duration::ZERO);
            assert_eq!(role.created_here, !role.member_of.is_empty());
        }
        assert!(!seen.contains("akr_audit_writer"));
        assert!(!seen.contains(MIGRATE_ROLE));
    }

    #[test]
    fn only_the_read_only_role_opens_read_only_sessions() {
        let read_only: Vec<_> = LOGIN_ROLES
            .iter()
            .filter(|role| role.settings.read_only)
            .map(|role| role.name)
            .collect();
        assert_eq!(read_only, ["akr_readonly"]);
    }

    #[test]
    fn settings_read_as_the_role_configuration_postgres_stores() {
        assert_eq!(
            settings(60, 60, true).as_config(),
            [
                "statement_timeout=60s",
                "idle_in_transaction_session_timeout=60s",
                "default_transaction_read_only=on"
            ]
        );
        assert_eq!(settings(2, 10, false).as_config().len(), 2);
    }

    #[test]
    fn only_builtin_c_utf8_in_utf8_is_the_expected_locale() {
        let locale = |encoding: &str, provider: &str, locale: Option<&str>| DatabaseLocale {
            encoding: encoding.to_owned(),
            provider: provider.to_owned(),
            locale: locale.map(str::to_owned),
        };
        assert!(locale("UTF8", "b", Some("C.UTF-8")).is_builtin_c_utf8());
        assert!(!locale("UTF8", "i", Some("ar")).is_builtin_c_utf8());
        assert!(!locale("UTF8", "c", None).is_builtin_c_utf8());
        assert!(!locale("LATIN1", "b", Some("C.UTF-8")).is_builtin_c_utf8());
        assert!(!locale("UTF8", "b", Some("C")).is_builtin_c_utf8());
    }
}
