//! `akr-ctl db`: provision the cluster, run the migrations, check what was installed.

use akr_common::config::Env;
use akr_db::provision::{
    self, DatabaseLocale, LOGIN_ROLES, LoginRole, MIGRATE_ROLE, RoleOutcome, RoleSettings,
};
use akr_db::{AKR_SCHEMAS, MIGRATOR, MigrationState, check};
use sqlx::Connection;

use crate::conn::ConnArgs;
use crate::error::CtlError;
use crate::scram;

/// Created NOLOGIN by migration 0001 and never given a password: no process signs in as it.
const AUDIT_WRITER_ROLE: &str = "akr_audit_writer";
/// Created NOLOGIN by the retention migration (phase 23).
const RETENTION_ROLE: &str = "akr_retention";
const READONLY_ROLE: &str = "akr_readonly";
const MONITOR_ROLE: &str = "akr_monitor";
/// The tables that hold stored answers and payloads. The read-only role sees none of them.
const LEDGER_TABLES: [&str; 3] = ["idempotency_keys", "jobs", "outbox"];

fn verifier(role: &str, password: &str) -> Result<String, CtlError> {
    scram::verifier(password)
        .map_err(|e| CtlError::usage(format!("the password of `{role}` cannot be used: {e}")))
}

fn describe(settings: &RoleSettings) -> String {
    settings.as_config().join(", ")
}

/// Creates what the first migration needs, then lets the service roles sign in. It can run again at any
/// time: before the migrations it leaves the roles they create alone, after them it finishes the job.
pub async fn provision(conn: &ConnArgs) -> Result<Vec<String>, CtlError> {
    let mut lines = Vec::new();
    let migrate_verifier = verifier(MIGRATE_ROLE, &conn.required_password(MIGRATE_ROLE)?)?;

    let mut admin = conn.admin(&conn.maintenance_database).await?;
    let found = provision::database_locale(&mut admin, &conn.database).await?;
    if let Some(locale) = &found
        && !locale.is_builtin_c_utf8()
    {
        return Err(wrong_locale(&conn.database, locale));
    }

    let created =
        provision::ensure_migrate_role(&mut admin, MIGRATE_ROLE, &migrate_verifier).await?;
    lines.push(format!(
        "role {MIGRATE_ROLE}: {}",
        if created { "created" } else { "updated" }
    ));

    if found.is_none() {
        provision::create_database(&mut admin, &conn.database, MIGRATE_ROLE).await?;
        lines.push(format!(
            "database {}: created ({}, builtin {})",
            conn.database,
            provision::ENCODING,
            provision::BUILTIN_LOCALE
        ));
    } else {
        provision::set_database_owner(&mut admin, &conn.database, MIGRATE_ROLE).await?;
        lines.push(format!("database {}: present", conn.database));
    }
    admin.close().await?;

    let mut target = conn.admin(&conn.database).await?;
    provision::ensure_admin_schema(&mut target, MIGRATE_ROLE).await?;
    lines.push(format!(
        "schema {}: owned by {MIGRATE_ROLE}",
        provision::ADMIN_SCHEMA
    ));

    for role in LOGIN_ROLES {
        let Some(password) = conn.password(role.name)? else {
            lines.push(format!(
                "role {}: skipped, {} is missing",
                role.name,
                conn.password_file(role.name).display()
            ));
            continue;
        };
        let verifier = verifier(role.name, &password)?;
        let line = match provision::ensure_login_role(&mut target, role, &verifier).await? {
            RoleOutcome::Ready { created } => format!(
                "role {}: {} ({})",
                role.name,
                if created {
                    "created, signs in"
                } else {
                    "signs in"
                },
                describe(&role.settings)
            ),
            RoleOutcome::Waiting => format!(
                "role {}: waiting for the migration that creates it; run `db provision` again after `db migrate`",
                role.name
            ),
        };
        lines.push(line);
    }
    target.close().await?;
    Ok(lines)
}

fn wrong_locale(database: &str, locale: &DatabaseLocale) -> CtlError {
    CtlError::refused(
        "database_locale",
        format!(
            "database `{database}` exists with encoding {}, locale provider `{}` and locale {}; the product needs \
             {} with the builtin provider and {}. A database keeps the locale it was created with: nothing was \
             changed",
            locale.encoding,
            locale.provider,
            locale.locale.as_deref().unwrap_or("(none)"),
            provision::ENCODING,
            provision::BUILTIN_LOCALE
        ),
    )
}

/// Runs the migrations this build embeds, as `akr_migrate`. A production database is migrated only under a
/// release someone approved.
pub async fn migrate(
    conn: &ConnArgs,
    env: Env,
    release_id: Option<&str>,
) -> Result<Vec<String>, CtlError> {
    if env.is_production() && release_id.is_none_or(|id| id.trim().is_empty()) {
        return Err(CtlError::refused(
            "release_id_missing",
            "a production database is migrated only under an approved release: give --release-id",
        ));
    }
    let pool = conn.pool(MIGRATE_ROLE).await?;
    let before = akr_db::applied_versions(&pool).await?;
    MIGRATOR.run(&pool).await?;
    let after = akr_db::applied_versions(&pool).await?;
    pool.close().await;

    let mut lines: Vec<String> = MIGRATOR
        .iter()
        .filter(|m| after.contains(&m.version) && !before.contains(&m.version))
        .map(|m| format!("applied {:04} {}", m.version, m.description))
        .collect();
    if lines.is_empty() {
        lines.push(format!(
            "nothing to apply: the database is at {}",
            after
                .last()
                .map_or("no migration".to_owned(), |v| format!("{v:04}"))
        ));
    }
    Ok(lines)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Fail,
    /// The check could not run here, and says why.
    Skip,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub check: String,
    pub verdict: Verdict,
    pub detail: String,
}

impl Finding {
    fn new(check: impl Into<String>, verdict: Verdict, detail: impl Into<String>) -> Self {
        Self {
            check: check.into(),
            verdict,
            detail: detail.into(),
        }
    }

    /// Passes when nothing offends.
    fn none_of(check: impl Into<String>, clean: &str, offenders: &[String]) -> Self {
        if offenders.is_empty() {
            Self::new(check, Verdict::Pass, clean)
        } else {
            Self::new(check, Verdict::Fail, offenders.join(", "))
        }
    }

    pub fn line(&self) -> String {
        let mark = match self.verdict {
            Verdict::Pass => "ok  ",
            Verdict::Fail => "FAIL",
            Verdict::Skip => "skip",
        };
        format!("{mark}  {}: {}", self.check, self.detail)
    }
}

/// Fails the command when any finding failed.
pub fn verdict(findings: &[Finding]) -> Result<(), CtlError> {
    let failed = findings
        .iter()
        .filter(|f| f.verdict == Verdict::Fail)
        .count();
    if failed == 0 {
        Ok(())
    } else {
        Err(CtlError::ChecksFailed {
            failed,
            total: findings.len(),
        })
    }
}

/// The attributes no role of the product may hold. The migration role creates roles; no other role does.
fn forbidden_attributes(role: &check::RoleAttributes, may_create_roles: bool) -> Vec<String> {
    [
        (role.superuser, "SUPERUSER"),
        (role.create_db, "CREATEDB"),
        (role.create_role && !may_create_roles, "CREATEROLE"),
        (role.bypass_rls, "BYPASSRLS"),
        (role.replication, "REPLICATION"),
    ]
    .into_iter()
    .filter(|(held, _)| *held)
    .map(|(_, name)| name.to_owned())
    .collect()
}

fn login_role_finding(role: &LoginRole, found: &check::RoleAttributes) -> Finding {
    let check = format!("role {}", role.name);
    let mut wrong = forbidden_attributes(found, false);
    if found.can_login {
        wrong.extend(
            role.settings
                .as_config()
                .into_iter()
                .filter(|setting| !found.config.contains(setting))
                .map(|setting| format!("`{setting}` is not set")),
        );
    }
    if role.created_here && found.member_of != role.member_of {
        wrong.push(format!(
            "member of [{}], expected [{}]",
            found.member_of.join(", "),
            role.member_of.join(", ")
        ));
    }
    if !wrong.is_empty() {
        Finding::new(check, Verdict::Fail, wrong.join("; "))
    } else if found.can_login {
        Finding::new(check, Verdict::Pass, describe(&role.settings))
    } else {
        Finding::new(
            check,
            Verdict::Skip,
            "cannot sign in yet: `db provision` has not given it a password",
        )
    }
}

async fn text_finding(conn: &ConnArgs, role: &str) -> Result<Finding, CtlError> {
    let check = format!("text round trip as {role}");
    if conn.password(role)?.is_none() {
        return Ok(Finding::new(
            check,
            Verdict::Skip,
            format!("{} is missing", conn.password_file(role).display()),
        ));
    }
    let mut session = match conn.connect(role, &conn.database).await {
        Ok(session) => session,
        Err(e) => {
            return Ok(Finding::new(
                check,
                Verdict::Fail,
                format!("cannot sign in: {e}"),
            ));
        }
    };
    let finding = match check::text_round_trip(&mut session).await {
        Ok(result) if result.mismatches.is_empty() => Finding::new(
            check,
            Verdict::Pass,
            if result.through_table {
                "Arabic, Kurdish and Persian came back unchanged as parameters and through a table"
            } else {
                "Arabic, Kurdish and Persian came back unchanged as parameters (the role writes nothing)"
            },
        ),
        Ok(result) => Finding::new(check, Verdict::Fail, result.mismatches.join("; ")),
        Err(e) => Finding::new(check, Verdict::Fail, e.to_string()),
    };
    session.close().await?;
    Ok(finding)
}

/// Reads the installation and reports what it found. It changes nothing.
pub async fn check(conn: &ConnArgs) -> Result<Vec<Finding>, CtlError> {
    let mut findings = Vec::new();
    let mut admin = conn.admin(&conn.database).await?;

    findings.push(
        match provision::database_locale(&mut admin, &conn.database).await? {
            Some(locale) if locale.is_builtin_c_utf8() => Finding::new(
                "database locale",
                Verdict::Pass,
                format!(
                    "{}, builtin {}",
                    provision::ENCODING,
                    provision::BUILTIN_LOCALE
                ),
            ),
            Some(locale) => Finding::new(
                "database locale",
                Verdict::Fail,
                format!(
                    "{}, provider `{}`, locale {}",
                    locale.encoding,
                    locale.provider,
                    locale.locale.as_deref().unwrap_or("(none)")
                ),
            ),
            None => Finding::new(
                "database locale",
                Verdict::Fail,
                "the database was not found",
            ),
        },
    );

    let admin_pool = conn.pool(&conn.admin_user).await?;
    findings.push(match akr_db::migration_state(&admin_pool).await? {
        MigrationState::Current => Finding::new(
            "migrations",
            Verdict::Pass,
            "the database is at this build's last migration",
        ),
        MigrationState::Behind => Finding::new(
            "migrations",
            Verdict::Fail,
            "the database has not had every migration of this build: run `db migrate`",
        ),
        MigrationState::Ahead => Finding::new(
            "migrations",
            Verdict::Fail,
            "the database is newer than this build",
        ),
    });
    admin_pool.close().await;

    let collations = check::display_collations(&mut admin).await?;
    let missing: Vec<String> = check::DISPLAY_COLLATIONS
        .iter()
        .filter(|(name, locale)| {
            !collations.iter().any(|(found, provider, found_locale)| {
                found == name
                    && provider == check::ICU_PROVIDER
                    && found_locale.as_deref() == Some(*locale)
            })
        })
        .map(|(name, locale)| format!("admin.{name} ({locale}) is missing or differs"))
        .collect();
    findings.push(Finding::none_of(
        "display collations",
        "admin.akr_ar and admin.akr_en are ICU collations with numeric ordering",
        &missing,
    ));
    findings.push(Finding::none_of(
        "indexes on display collations",
        "no index is built on an ICU collation",
        &check::indexes_on_icu_collations(&mut admin, &AKR_SCHEMAS).await?,
    ));

    findings.push(if missing.is_empty() {
        let sorted = check::arabic_sort(&mut admin).await?;
        if sorted == check::SORT_EXPECTED {
            Finding::new(
                "Arabic sort order",
                Verdict::Pass,
                "the fixture sorts in the expected order under admin.akr_ar",
            )
        } else {
            Finding::new(
                "Arabic sort order",
                Verdict::Fail,
                format!(
                    "got [{}], expected [{}]",
                    sorted.join(" | "),
                    check::SORT_EXPECTED.join(" | ")
                ),
            )
        }
    } else {
        Finding::new(
            "Arabic sort order",
            Verdict::Skip,
            "admin.akr_ar is not usable",
        )
    });

    for role in std::iter::once(MIGRATE_ROLE).chain(LOGIN_ROLES.iter().map(|r| r.name)) {
        if provision::role_exists(&mut admin, role).await? {
            findings.push(text_finding(conn, role).await?);
        }
    }

    findings.push(
        match check::role_attributes(&mut admin, MIGRATE_ROLE).await? {
            Some(found) => {
                let mut wrong = forbidden_attributes(&found, true);
                let search_path = format!("search_path={}", provision::ADMIN_SCHEMA);
                if !found.config.contains(&search_path) {
                    wrong.push(format!("`{search_path}` is not set"));
                }
                Finding::none_of(
                    format!("role {MIGRATE_ROLE}"),
                    "creates roles, is no superuser, resolves names in admin",
                    &wrong,
                )
            }
            None => Finding::new(
                format!("role {MIGRATE_ROLE}"),
                Verdict::Fail,
                "the role does not exist: run `db provision`",
            ),
        },
    );
    for role in LOGIN_ROLES {
        if let Some(found) = check::role_attributes(&mut admin, role.name).await? {
            findings.push(login_role_finding(role, &found));
        }
    }
    for role in [AUDIT_WRITER_ROLE, RETENTION_ROLE] {
        if let Some(found) = check::role_attributes(&mut admin, role).await? {
            let mut wrong = forbidden_attributes(&found, false);
            if found.can_login {
                wrong.push("it can sign in".to_owned());
            }
            findings.push(Finding::none_of(
                format!("role {role}"),
                "cannot sign in and holds no cluster attribute",
                &wrong,
            ));
        }
    }

    if provision::role_exists(&mut admin, READONLY_ROLE).await? {
        findings.push(Finding::none_of(
            format!("{READONLY_ROLE} writes nothing"),
            "no write privilege on any table",
            &check::table_privileges(
                &mut admin,
                READONLY_ROLE,
                &AKR_SCHEMAS,
                None,
                &check::WRITE_PRIVILEGES,
            )
            .await?,
        ));
        findings.push(Finding::none_of(
            format!("{READONLY_ROLE} reads no ledger"),
            "no privilege on the idempotency ledger, the job queue or the outbox",
            &check::table_privileges(
                &mut admin,
                READONLY_ROLE,
                &[provision::ADMIN_SCHEMA],
                Some(&LEDGER_TABLES),
                &check::ALL_PRIVILEGES,
            )
            .await?,
        ));
    }
    if provision::role_exists(&mut admin, MONITOR_ROLE).await? {
        let mut held = check::schema_usage(&mut admin, MONITOR_ROLE, &AKR_SCHEMAS).await?;
        held.extend(
            check::table_privileges(
                &mut admin,
                MONITOR_ROLE,
                &AKR_SCHEMAS,
                None,
                &check::ALL_PRIVILEGES,
            )
            .await?,
        );
        findings.push(Finding::none_of(
            format!("{MONITOR_ROLE} reads no product data"),
            "no privilege on any schema or table of the product",
            &held,
        ));
    }

    findings.push(Finding::none_of(
        "PUBLIC holds nothing",
        "no privilege is granted to PUBLIC on a schema or table of the product",
        &check::public_grants(&mut admin, &AKR_SCHEMAS).await?,
    ));
    findings.push(Finding::none_of(
        "table ownership",
        "every table belongs to the migration role",
        &check::tables_not_owned_by(&mut admin, &AKR_SCHEMAS, MIGRATE_ROLE).await?,
    ));

    admin.close().await?;
    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attributes(name: &str) -> check::RoleAttributes {
        check::RoleAttributes {
            name: name.to_owned(),
            can_login: true,
            superuser: false,
            create_db: false,
            create_role: false,
            bypass_rls: false,
            replication: false,
            config: Vec::new(),
            member_of: Vec::new(),
        }
    }

    fn role(name: &str) -> &'static LoginRole {
        LOGIN_ROLES
            .iter()
            .find(|role| role.name == name)
            .unwrap_or_else(|| unreachable!("{name} is a login role"))
    }

    #[test]
    fn a_role_with_its_settings_passes_and_one_without_them_fails() {
        let app = role("akr_app");
        let mut found = attributes("akr_app");
        found.config = app.settings.as_config();
        assert_eq!(login_role_finding(app, &found).verdict, Verdict::Pass);

        found.config.pop();
        let finding = login_role_finding(app, &found);
        assert_eq!(finding.verdict, Verdict::Fail);
        assert!(
            finding
                .detail
                .contains("idle_in_transaction_session_timeout")
        );
    }

    #[test]
    fn a_role_that_cannot_sign_in_yet_is_skipped_not_failed() {
        let mut found = attributes("akr_ingest");
        found.can_login = false;
        assert_eq!(
            login_role_finding(role("akr_ingest"), &found).verdict,
            Verdict::Skip
        );
    }

    #[test]
    fn a_cluster_attribute_fails_any_role_and_only_the_migration_role_creates_roles() {
        let mut found = attributes("akr_readonly");
        found.config = role("akr_readonly").settings.as_config();
        found.bypass_rls = true;
        found.create_role = true;
        let finding = login_role_finding(role("akr_readonly"), &found);
        assert_eq!(finding.verdict, Verdict::Fail);
        assert!(finding.detail.contains("BYPASSRLS") && finding.detail.contains("CREATEROLE"));

        assert_eq!(forbidden_attributes(&found, true), ["BYPASSRLS"]);
    }

    #[test]
    fn the_monitoring_role_must_be_a_member_of_pg_monitor_only() {
        let monitor = role("akr_monitor");
        let mut found = attributes("akr_monitor");
        found.config = monitor.settings.as_config();
        found.member_of = vec!["pg_monitor".to_owned()];
        assert_eq!(login_role_finding(monitor, &found).verdict, Verdict::Pass);

        found.member_of.push("akr_app".to_owned());
        assert_eq!(login_role_finding(monitor, &found).verdict, Verdict::Fail);
    }

    #[test]
    fn one_failed_finding_fails_the_command_and_a_skip_does_not() {
        let pass = Finding::new("a", Verdict::Pass, "fine");
        let skip = Finding::new("b", Verdict::Skip, "not here");
        assert!(verdict(&[pass.clone(), skip.clone()]).is_ok());

        let fail = Finding::none_of("c", "clean", &["admin.settings:SELECT".to_owned()]);
        assert_eq!(fail.line(), "FAIL  c: admin.settings:SELECT");
        assert!(matches!(
            verdict(&[pass, skip, fail]),
            Err(CtlError::ChecksFailed {
                failed: 1,
                total: 3
            })
        ));
    }

    #[test]
    fn a_database_with_another_locale_is_refused_by_code() {
        let locale = DatabaseLocale {
            encoding: "UTF8".to_owned(),
            provider: "c".to_owned(),
            locale: None,
        };
        let refused = wrong_locale("akr", &locale);
        assert_eq!(refused.code(), Some("database_locale"));
        assert!(refused.to_string().contains("nothing was changed"));
    }
}
