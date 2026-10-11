//! The registry of setting keys: every key the code reads, declared once (DR-34).
//!
//! A row of `admin.settings` holds the value and the bank-facing facts (label, regulatory floor). The
//! registry holds what the code must know without a database: the type, whether `null` is allowed, the
//! bounds of a sane value and the seeded default. A test asserts that the registry and the rows the
//! migrations seed agree, so a key cannot exist in one and not in the other.
//!
//! Secrets never live in settings: only paths and thresholds do.

use akr_domain::DomainError;
use akr_domain::errors::FieldViolation;
use akr_domain::settings::{SettingValueType, validate_timezone, validate_value};
use serde_json::Value;

/// The keys of phase 00. Every later phase adds its keys here and to [`REGISTRY`].
pub mod keys {
    pub const LOCALE_DEFAULT: &str = "locale.default";
    pub const BANK_TIMEZONE: &str = "bank.timezone";
    pub const IDEMPOTENCY_TTL_DAYS: &str = "idempotency.ttl_days";
    pub const IDEMPOTENCY_IN_FLIGHT_LEASE_S: &str = "idempotency.in_flight_lease_s";
    pub const JOBS_LEASE_S: &str = "jobs.lease_s";
    pub const JOBS_DONE_RETENTION_DAYS: &str = "jobs.done_retention_days";
    pub const OUTBOX_RETENTION_DAYS: &str = "outbox.retention_days";
    pub const OUTBOX_MAX_ATTEMPTS: &str = "outbox.max_attempts";
    pub const TIMERS_SWEEP_INTERVAL_S: &str = "timers.sweep_interval_s";
    pub const TIMERS_TERMINAL_RETENTION_DAYS: &str = "timers.terminal_retention_days";
    pub const SCHEDULE_MAINTENANCE_SWEEP: &str = "schedule.maintenance_sweep";
    pub const SCHEDULE_BACKUP_CHECK: &str = "schedule.backup_check";
    pub const OPS_BACKUP_MARKER_PATH: &str = "ops.backup_marker_path";
    pub const OPS_BACKUP_MAX_AGE_H: &str = "ops.backup_max_age_h";
    pub const OPS_LAST_RESTORE_DRILL_AT: &str = "ops.last_restore_drill_at";
    pub const OPS_RESTORE_DRILL_MAX_AGE_DAYS: &str = "ops.restore_drill_max_age_days";
}

/// What a value must satisfy beyond its type. Bounds are sanity limits of the software; the regulatory limit
/// of a key is the `floor` of its row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    None,
    Range { min: i64, max: i64 },
    OneOf(&'static [&'static str]),
    Timezone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingDef {
    pub key: &'static str,
    pub value_type: SettingValueType,
    /// `null` is a valid value (a date that has not happened yet).
    pub nullable: bool,
    /// The key changes only through a change request (phase 04).
    pub governed: bool,
    /// The seeded value, as JSON text. A getter falls back to it when the row cannot be read.
    pub default: &'static str,
    pub rule: Rule,
    /// `akr-ctl` may write this governed key directly (the drill dates).
    pub cli_writable: bool,
}

const fn def(
    key: &'static str,
    value_type: SettingValueType,
    governed: bool,
    default: &'static str,
    rule: Rule,
) -> SettingDef {
    SettingDef {
        key,
        value_type,
        nullable: false,
        governed,
        default,
        rule,
        cli_writable: false,
    }
}

const fn range(min: i64, max: i64) -> Rule {
    Rule::Range { min, max }
}

pub const REGISTRY: &[SettingDef] = &[
    def(
        keys::LOCALE_DEFAULT,
        SettingValueType::String,
        false,
        "\"ar\"",
        Rule::OneOf(&["ar", "en"]),
    ),
    def(
        keys::BANK_TIMEZONE,
        SettingValueType::String,
        true,
        "\"Asia/Baghdad\"",
        Rule::Timezone,
    ),
    def(
        keys::IDEMPOTENCY_TTL_DAYS,
        SettingValueType::Integer,
        false,
        "7",
        range(1, 90),
    ),
    def(
        keys::IDEMPOTENCY_IN_FLIGHT_LEASE_S,
        SettingValueType::DurationS,
        false,
        "120",
        range(1, 3_600),
    ),
    def(
        keys::JOBS_LEASE_S,
        SettingValueType::DurationS,
        false,
        "600",
        range(30, 86_400),
    ),
    def(
        keys::JOBS_DONE_RETENTION_DAYS,
        SettingValueType::Integer,
        false,
        "30",
        range(1, 3_650),
    ),
    def(
        keys::OUTBOX_RETENTION_DAYS,
        SettingValueType::Integer,
        false,
        "30",
        range(1, 3_650),
    ),
    def(
        keys::OUTBOX_MAX_ATTEMPTS,
        SettingValueType::Integer,
        false,
        "10",
        range(1, 100),
    ),
    def(
        keys::TIMERS_SWEEP_INTERVAL_S,
        SettingValueType::DurationS,
        false,
        "30",
        range(1, 3_600),
    ),
    def(
        keys::TIMERS_TERMINAL_RETENTION_DAYS,
        SettingValueType::Integer,
        false,
        "90",
        range(1, 3_650),
    ),
    def(
        keys::SCHEDULE_MAINTENANCE_SWEEP,
        SettingValueType::Cron,
        false,
        "\"0 30 2 * * *\"",
        Rule::None,
    ),
    def(
        keys::SCHEDULE_BACKUP_CHECK,
        SettingValueType::Cron,
        false,
        "\"0 0 6 * * *\"",
        Rule::None,
    ),
    def(
        keys::OPS_BACKUP_MARKER_PATH,
        SettingValueType::String,
        true,
        "\"/var/lib/akr/backup/heartbeat\"",
        Rule::None,
    ),
    def(
        keys::OPS_BACKUP_MAX_AGE_H,
        SettingValueType::Integer,
        true,
        "26",
        range(1, 720),
    ),
    SettingDef {
        nullable: true,
        cli_writable: true,
        ..def(
            keys::OPS_LAST_RESTORE_DRILL_AT,
            SettingValueType::Timestamp,
            true,
            "null",
            Rule::None,
        )
    },
    def(
        keys::OPS_RESTORE_DRILL_MAX_AGE_DAYS,
        SettingValueType::Integer,
        true,
        "31",
        range(1, 366),
    ),
];

/// Key endings that would suggest a secret. A key that ends in one of them must be listed in
/// [`NON_SECRET_NAMES`], with the reason it holds no secret.
pub const SECRET_LIKE_ENDINGS: &[&str] = &["password", "secret", "token", "key"];

/// Keys that end in a secret-like word and hold no secret. Empty in phase 00.
pub const NON_SECRET_NAMES: &[&str] = &[];

pub fn lookup(key: &str) -> Option<&'static SettingDef> {
    REGISTRY.iter().find(|d| d.key == key)
}

impl SettingDef {
    /// The seeded value.
    pub fn default_value(&self) -> Value {
        serde_json::from_str(self.default).unwrap_or(Value::Null)
    }

    /// Checks a value against the type and the rule of the key. Failures point at `/value`.
    pub fn validate(&self, value: &Value) -> Result<(), DomainError> {
        validate_value(self.value_type, value, self.nullable)?;
        if value.is_null() {
            return Ok(());
        }
        match self.rule {
            Rule::None => Ok(()),
            Rule::Timezone => validate_timezone(value),
            Rule::Range { min, max } => match value.as_i64() {
                Some(n) if (min..=max).contains(&n) => Ok(()),
                _ => Err(DomainError::Validation(vec![
                    FieldViolation::new("/value", "out_of_range")
                        .param("min", min)
                        .param("max", max),
                ])),
            },
            Rule::OneOf(allowed) => match value.as_str() {
                Some(s) if allowed.contains(&s) => Ok(()),
                _ => Err(DomainError::Validation(vec![
                    FieldViolation::new("/value", "not_one_of").param("allowed", allowed.to_vec()),
                ])),
            },
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use std::collections::{BTreeMap, HashSet};
    use std::path::Path;

    use regex::Regex;
    use serde_json::json;

    use super::*;

    /// One seeded row of `admin.settings`, read from the migration files.
    #[derive(Debug, PartialEq)]
    struct Seeded {
        value: Value,
        value_type: String,
        governed: bool,
    }

    /// Every `(key, value, value_type, floor, governed, label)` tuple of an `INSERT INTO admin.settings` in
    /// `backend/migrations/`.
    fn seeded_rows() -> BTreeMap<String, Seeded> {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
        let insert = Regex::new(r"(?s)INSERT INTO admin\.settings\s*\(key, value, value_type, floor, governed, label\)\s*VALUES(.*?);").unwrap();
        let tuple = Regex::new(
            r"\(\s*'([a-z0-9_.]+)',\s*'((?:[^']|'')*)',\s*'([a-z_]+)',\s*(?:NULL|'(?:[^']|'')*'),\s*(TRUE|FALSE),",
        )
        .unwrap();
        let mut rows = BTreeMap::new();
        let mut files: Vec<_> = std::fs::read_dir(&dir)
            .expect("the migrations directory")
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|x| x == "sql"))
            .collect();
        files.sort();
        for file in files {
            let sql = std::fs::read_to_string(&file).unwrap();
            for block in insert.captures_iter(&sql) {
                for t in tuple.captures_iter(&block[1]) {
                    let value =
                        serde_json::from_str(&t[2].replace("''", "'")).unwrap_or_else(|e| {
                            panic!(
                                "{}: the value of {} is not JSON: {e}",
                                file.display(),
                                &t[1]
                            )
                        });
                    let previous = rows.insert(
                        t[1].to_owned(),
                        Seeded {
                            value,
                            value_type: t[3].to_owned(),
                            governed: &t[4] == "TRUE",
                        },
                    );
                    assert!(previous.is_none(), "{} is seeded twice", &t[1]);
                }
            }
        }
        rows
    }

    #[test]
    fn the_registry_and_the_seeded_rows_agree() {
        let seeded = seeded_rows();
        assert_eq!(seeded.len(), 16, "phase 00 seeds sixteen keys");

        let declared: HashSet<&str> = REGISTRY.iter().map(|d| d.key).collect();
        let in_migrations: HashSet<&str> = seeded.keys().map(String::as_str).collect();
        assert_eq!(declared, in_migrations);

        for def in REGISTRY {
            let row = &seeded[def.key];
            assert_eq!(row.value_type, def.value_type.as_str(), "{}", def.key);
            assert_eq!(row.governed, def.governed, "{}", def.key);
            assert_eq!(row.value, def.default_value(), "{}", def.key);
        }
    }

    #[test]
    fn keys_are_declared_once() {
        let mut seen = HashSet::new();
        for def in REGISTRY {
            assert!(seen.insert(def.key), "{} is declared twice", def.key);
            assert_eq!(lookup(def.key), Some(def));
        }
        assert_eq!(lookup("no.such_key"), None);
    }

    #[test]
    fn every_default_satisfies_its_own_definition() {
        for def in REGISTRY {
            def.validate(&def.default_value())
                .unwrap_or_else(|e| panic!("{}: {e:?}", def.key));
        }
    }

    #[test]
    fn no_key_looks_like_a_secret() {
        for def in REGISTRY {
            let last = def.key.rsplit(['.', '_']).next().unwrap();
            if SECRET_LIKE_ENDINGS.contains(&last) {
                assert!(
                    NON_SECRET_NAMES.contains(&def.key),
                    "{} ends in `{last}`: a secret belongs in OpenBao, or the key in NON_SECRET_NAMES",
                    def.key
                );
            }
        }
        for name in NON_SECRET_NAMES {
            assert!(
                lookup(name).is_some(),
                "{name} is allow-listed and not declared"
            );
        }
    }

    #[test]
    fn only_the_drill_date_may_be_null_and_written_by_the_cli() {
        let nullable: Vec<_> = REGISTRY
            .iter()
            .filter(|d| d.nullable)
            .map(|d| d.key)
            .collect();
        assert_eq!(nullable, [keys::OPS_LAST_RESTORE_DRILL_AT]);
        let cli: Vec<_> = REGISTRY
            .iter()
            .filter(|d| d.cli_writable)
            .map(|d| d.key)
            .collect();
        assert_eq!(cli, [keys::OPS_LAST_RESTORE_DRILL_AT]);
        for def in REGISTRY.iter().filter(|d| d.cli_writable) {
            assert!(
                def.governed,
                "{}: the exemption is for governed keys",
                def.key
            );
        }
    }

    #[test]
    fn a_value_outside_the_bounds_names_them() {
        let ttl = lookup(keys::IDEMPOTENCY_TTL_DAYS).unwrap();
        assert!(ttl.validate(&json!(7)).is_ok());
        let Err(DomainError::Validation(v)) = ttl.validate(&json!(0)) else {
            panic!("0 days must be refused");
        };
        assert_eq!(v[0].code, "out_of_range");
        assert_eq!(v[0].params["max"], 90);
        assert!(ttl.validate(&json!("7")).is_err());
        assert!(ttl.validate(&Value::Null).is_err());
    }

    #[test]
    fn the_locale_the_zone_and_the_schedules_are_checked() {
        let locale = lookup(keys::LOCALE_DEFAULT).unwrap();
        assert!(locale.validate(&json!("en")).is_ok());
        assert!(locale.validate(&json!("fr")).is_err());

        let zone = lookup(keys::BANK_TIMEZONE).unwrap();
        assert!(zone.validate(&json!("Asia/Baghdad")).is_ok());
        assert!(zone.validate(&json!("Baghdad")).is_err());

        let sweep = lookup(keys::SCHEDULE_MAINTENANCE_SWEEP).unwrap();
        assert!(sweep.validate(&json!("0 0 3 * * *")).is_ok());
        assert!(sweep.validate(&json!("0 3 * * *")).is_err());

        let drill = lookup(keys::OPS_LAST_RESTORE_DRILL_AT).unwrap();
        assert!(drill.validate(&Value::Null).is_ok());
        assert!(drill.validate(&json!("2026-10-11T09:00:00Z")).is_ok());
        assert!(drill.validate(&json!("yesterday")).is_err());
    }
}
