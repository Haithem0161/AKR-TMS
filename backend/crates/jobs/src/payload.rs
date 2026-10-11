//! The rule for what a job or outbox payload may hold: ids and codes only (tipping-off review, phase 00
//! section 3).
//!
//! A payload is read by operators in the cockpit and travels through logs of failures. A name, a reason or
//! any free text in it would tell a reader about a case they have no access to. Every registered kind and
//! topic has a test that runs a sample payload through [`check`].

use chrono::{DateTime, NaiveDate};
use serde_json::Value;
use uuid::Uuid;

const NAME_ENDINGS: &[&str] = &[
    "_id", "_ids", "_code", "_codes", "_kind", "_at", "_on", "_no", "_version", "_count", "_hash",
];
const NAMES: &[&str] = &[
    "id", "ids", "kind", "code", "codes", "reason", "slot", "attempt", "by", "version", "count",
];
const CODE_MAX_LEN: usize = 64;

/// Checks that `payload` is an object whose every value, at any depth, is named like an id or a code and
/// holds one: a UUID, a number, a boolean, a date or timestamp, or a short machine code. A field that only
/// groups other fields may have any name.
pub fn check(payload: &Value) -> Result<(), String> {
    match payload {
        Value::Object(_) => walk("", payload),
        _ => Err("a payload is a JSON object".to_owned()),
    }
}

fn walk(path: &str, value: &Value) -> Result<(), String> {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) => Ok(()),
        Value::String(text) => {
            if is_id_or_code(text) {
                Ok(())
            } else {
                Err(format!(
                    "{path} holds text that is not an id, a date or a code"
                ))
            }
        }
        Value::Array(items) => items.iter().try_for_each(|item| walk(path, item)),
        Value::Object(fields) => fields.iter().try_for_each(|(name, field)| {
            let here = format!("{path}/{name}");
            if !field.is_object() && !is_allowed_name(name) {
                return Err(format!("{here} is not named like an id or a code"));
            }
            walk(&here, field)
        }),
    }
}

fn is_allowed_name(name: &str) -> bool {
    NAMES.contains(&name) || NAME_ENDINGS.iter().any(|ending| name.ends_with(ending))
}

fn is_id_or_code(text: &str) -> bool {
    Uuid::parse_str(text).is_ok()
        || DateTime::parse_from_rfc3339(text).is_ok()
        || text.parse::<NaiveDate>().is_ok()
        || is_code(text)
}

/// A machine code: lower-case ASCII letters, digits and `_ . : -`, starting with a letter or a digit.
fn is_code(text: &str) -> bool {
    let mut bytes = text.bytes();
    let starts_well = bytes
        .next()
        .is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit());
    starts_well
        && text.len() <= CODE_MAX_LEN
        && bytes.all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'.' | b':' | b'-')
        })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn ids_codes_dates_and_numbers_pass() {
        let payload = json!({
            "correlation_id": "0190c000-0000-7000-8000-000000000001",
            "timer_id": "0190c000-0000-7000-8000-000000000002",
            "owner_kind": "freeze",
            "reason": "setting",
            "slot": "2026-10-11T02:30:00Z",
            "due_on": "2026-10-11",
            "attempt": 3,
            "party_ids": ["0190c000-0000-7000-8000-000000000003"],
            "source": {"batch_id": "0190c000-0000-7000-8000-000000000004", "row_count": 1200},
            "list_version": null,
        });
        assert_eq!(check(&payload), Ok(()));
        assert_eq!(check(&json!({})), Ok(()));
    }

    #[test]
    fn free_text_is_refused_whatever_the_field_is_called() {
        assert!(check(&json!({"reason": "Suspected structuring by the customer"})).is_err());
        assert!(check(&json!({"owner_kind": "حالة اشتباه"})).is_err());
        assert!(check(&json!({"party_ids": ["Some Person"]})).is_err());
    }

    #[test]
    fn a_field_that_is_not_named_like_an_id_or_a_code_is_refused() {
        assert!(check(&json!({"customer_name": "x"})).is_err());
        assert!(check(&json!({"note": "ok"})).is_err());
        assert!(check(&json!({"source": {"comment": "ok"}})).is_err());
    }

    #[test]
    fn a_payload_must_be_an_object() {
        assert!(check(&json!(["a"])).is_err());
        assert!(check(&json!("a")).is_err());
    }
}
