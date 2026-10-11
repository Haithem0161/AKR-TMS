//! Runtime settings: what the bank may want different next month (`.claude/rules/admin-managed.md`).
//!
//! The rules of a value live here, without a database: its type, its nullability and its regulatory limit.

use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::types::Json;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::errors::{DomainError, FieldViolation};
use crate::text::I18nText;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Setting {
    pub key: String,
    #[schema(value_type = Object)]
    pub value: Value,
    pub value_type: SettingValueType,
    #[schema(value_type = Option<SettingFloor>)]
    pub floor: Option<Json<SettingFloor>>,
    /// A governed key changes only through a change request (phase 04).
    pub governed: bool,
    #[schema(value_type = I18nText)]
    pub label: Json<I18nText>,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
}

/// The `value_type` column of `admin.settings`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SettingValueType {
    Integer,
    Decimal,
    Boolean,
    String,
    #[sqlx(rename = "duration_s")]
    #[serde(rename = "duration_s")]
    DurationS,
    Cron,
    Timestamp,
    Json,
}

impl SettingValueType {
    pub fn as_str(self) -> &'static str {
        match self {
            SettingValueType::Integer => "integer",
            SettingValueType::Decimal => "decimal",
            SettingValueType::Boolean => "boolean",
            SettingValueType::String => "string",
            SettingValueType::DurationS => "duration_s",
            SettingValueType::Cron => "cron",
            SettingValueType::Timestamp => "timestamp",
            SettingValueType::Json => "json",
        }
    }
}

/// The regulatory limit of a setting. The bank may move the value only inside it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SettingFloor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub min: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub max: Option<Value>,
    /// The rule the limit comes from, quoted on the screen that refuses a value.
    pub basis: String,
}

impl SettingFloor {
    /// True when `value` is a number inside the limit. A value that is not a number has no order and is
    /// outside every limit.
    pub fn admits(&self, value: &Value) -> bool {
        let Some(v) = value.as_f64() else {
            return false;
        };
        let above_min = self
            .min
            .as_ref()
            .and_then(Value::as_f64)
            .is_none_or(|min| v >= min);
        let below_max = self
            .max
            .as_ref()
            .and_then(Value::as_f64)
            .is_none_or(|max| v <= max);
        above_min && below_max
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateSetting {
    #[schema(value_type = Object)]
    pub value: Value,
}

/// How a setting change reached the service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeSource {
    /// Straight from a caller. Refused on a governed key.
    Direct,
    /// The activation of an approved change request (phase 04).
    ChangeRequest(Uuid),
}

/// Checks `value` against the type of its setting. `nullable` is the registry's word on whether the key
/// may hold `null`.
pub fn validate_value(
    value_type: SettingValueType,
    value: &Value,
    nullable: bool,
) -> Result<(), DomainError> {
    if value.is_null() {
        return if nullable {
            Ok(())
        } else {
            Err(violation("required"))
        };
    }
    let ok = match value_type {
        SettingValueType::Integer => value.as_i64().is_some(),
        SettingValueType::Decimal => value.is_number(),
        SettingValueType::Boolean => value.is_boolean(),
        SettingValueType::String => value.is_string(),
        SettingValueType::DurationS => value.as_i64().is_some_and(|n| n > 0),
        SettingValueType::Cron => value.as_str().is_some_and(is_cron),
        SettingValueType::Timestamp => value
            .as_str()
            .is_some_and(|s| DateTime::parse_from_rfc3339(s).is_ok()),
        SettingValueType::Json => true,
    };
    if ok {
        Ok(())
    } else {
        Err(violation(match value_type {
            SettingValueType::Integer => "not_an_integer",
            SettingValueType::Decimal => "not_a_number",
            SettingValueType::Boolean => "not_a_boolean",
            SettingValueType::String => "not_a_string",
            SettingValueType::DurationS => "not_a_positive_duration",
            SettingValueType::Cron => "not_a_cron",
            SettingValueType::Timestamp => "not_a_timestamp",
            SettingValueType::Json => "invalid",
        }))
    }
}

/// The value of `bank.timezone` must be a zone of the IANA list.
pub fn validate_timezone(value: &Value) -> Result<(), DomainError> {
    match value.as_str().map(chrono_tz::Tz::from_str) {
        Some(Ok(_)) => Ok(()),
        _ => Err(violation("not_a_timezone")),
    }
}

/// Checks `value` against the limit of its setting, when it has one.
pub fn check_floor(
    key: &str,
    floor: Option<&SettingFloor>,
    value: &Value,
) -> Result<(), DomainError> {
    match floor {
        Some(floor) if !floor.admits(value) => Err(DomainError::BelowFloor {
            key: key.to_owned(),
        }),
        _ => Ok(()),
    }
}

/// A schedule: six fields, seconds first (`0 30 2 * * *`).
pub fn is_cron(expr: &str) -> bool {
    crate::settings::schedule::parse(expr).is_ok()
}

fn violation(code: &'static str) -> DomainError {
    DomainError::Validation(vec![FieldViolation::new("/value", code)])
}

pub mod schedule;

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::*;

    fn floor(min: Option<Value>, max: Option<Value>) -> SettingFloor {
        SettingFloor {
            min,
            max,
            basis: "test".to_owned(),
        }
    }

    #[test]
    fn each_type_accepts_its_own_values_and_refuses_the_others() {
        use SettingValueType::*;
        let cases = [
            (Integer, json!(7), true),
            (Integer, json!(7.5), false),
            (Integer, json!("7"), false),
            (Decimal, json!(0.25), true),
            (Decimal, json!(3), true),
            (Decimal, json!("0.25"), false),
            (Boolean, json!(true), true),
            (Boolean, json!(1), false),
            (String, json!("ar"), true),
            (String, json!(1), false),
            (DurationS, json!(120), true),
            (DurationS, json!(0), false),
            (DurationS, json!(-5), false),
            (Cron, json!("0 30 2 * * *"), true),
            (Cron, json!("not a schedule"), false),
            (Cron, json!("30 2 * * *"), false),
            (Timestamp, json!("2026-10-11T08:00:00Z"), true),
            (Timestamp, json!("2026-10-11"), false),
            (Json, json!({"a": [1, 2]}), true),
        ];
        for (ty, value, expected) in cases {
            assert_eq!(
                validate_value(ty, &value, false).is_ok(),
                expected,
                "{} with {value}",
                ty.as_str()
            );
        }
    }

    #[test]
    fn null_is_accepted_only_where_the_registry_allows_it() {
        assert!(validate_value(SettingValueType::Timestamp, &Value::Null, true).is_ok());
        assert!(validate_value(SettingValueType::Timestamp, &Value::Null, false).is_err());
    }

    #[test]
    fn a_floor_admits_values_inside_it_and_refuses_values_outside() {
        let at_least_eight = floor(Some(json!(8)), None);
        assert!(at_least_eight.admits(&json!(8)));
        assert!(at_least_eight.admits(&json!(30)));
        assert!(!at_least_eight.admits(&json!(7)));

        let at_most = floor(None, Some(json!(1.25)));
        assert!(at_most.admits(&json!(1.25)));
        assert!(!at_most.admits(&json!(1.26)));

        let both = floor(Some(json!(1)), Some(json!(5)));
        assert!(both.admits(&json!(3)));
        assert!(!both.admits(&json!(0)));
        assert!(!both.admits(&json!(6)));
        assert!(!both.admits(&json!("3")));
    }

    #[test]
    fn check_floor_names_the_key_it_refused() {
        let limit = floor(Some(json!(8)), None);
        match check_floor("password.min_length", Some(&limit), &json!(6)) {
            Err(DomainError::BelowFloor { key }) => assert_eq!(key, "password.min_length"),
            other => panic!("expected BelowFloor, got {other:?}"),
        }
        assert!(check_floor("idempotency.ttl_days", None, &json!(1)).is_ok());
    }

    #[test]
    fn a_floor_with_keys_this_build_does_not_know_still_reads() {
        let parsed: SettingFloor =
            serde_json::from_value(json!({"min": 2555, "applies_to": ["R5"], "basis": "D2.1"}))
                .unwrap();
        assert_eq!(parsed.min, Some(json!(2555)));
    }

    #[test]
    fn the_timezone_must_be_on_the_iana_list() {
        assert!(validate_timezone(&json!("Asia/Baghdad")).is_ok());
        assert!(validate_timezone(&json!("Baghdad")).is_err());
        assert!(validate_timezone(&json!(3)).is_err());
    }
}
