//! The redactor every sink shares.
//!
//! A binary starts logging before it can read the classification table, so the redactor starts with the
//! bodies and credential names only and takes the classified column names once, when the table has been
//! read. Reading it is lock-free: a log line never waits on another.

use std::borrow::Cow;
use std::fmt;
use std::sync::{Arc, OnceLock};

use akr_common::redaction::{Redactor, length_marker, redact_text};
use akr_domain::classification::ColumnClassification;

/// What stands in for a metric label that holds an id.
pub const ID_MARKER: &str = "[redacted:id]";

#[derive(Clone)]
pub struct SharedRedactor(Arc<Inner>);

struct Inner {
    baseline: Redactor,
    classified: OnceLock<Redactor>,
}

impl fmt::Debug for SharedRedactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedRedactor")
            .field("classified", &self.0.classified.get().is_some())
            .finish()
    }
}

impl Default for SharedRedactor {
    fn default() -> Self {
        Self::new()
    }
}

impl SharedRedactor {
    pub fn new() -> Self {
        Self(Arc::new(Inner {
            baseline: Redactor::baseline(),
            classified: OnceLock::new(),
        }))
    }

    /// Adds the columns the classification table marks `personal`, `financial`, `secret-str` or `card`.
    /// The table is read once per process: a second call changes nothing and returns `false`.
    pub fn load_classification(&self, rows: &[ColumnClassification]) -> bool {
        let columns = rows
            .iter()
            .filter(|row| row.classification.is_redacted())
            .map(|row| row.column_name.as_str());
        self.0
            .classified
            .set(Redactor::baseline().with_classified(columns))
            .is_ok()
    }

    fn current(&self) -> &Redactor {
        self.0.classified.get().unwrap_or(&self.0.baseline)
    }

    pub fn is_classified(&self, field: &str) -> bool {
        self.current().is_classified(field)
    }

    /// The value of a log field or a trace attribute as a sink may see it.
    pub fn field<'a>(&self, name: &str, value: &'a str) -> Cow<'a, str> {
        self.current().field(name, value)
    }

    /// Free text with no field name: a span name, an event name, a status description.
    pub fn text<'a>(&self, value: &'a str) -> Cow<'a, str> {
        redact_text(value)
    }

    /// A number, given as its text. `Some` holds the marker that replaces it; `None` keeps the number.
    pub fn number(&self, name: &str, rendered: &str) -> Option<String> {
        if self.is_classified(name) {
            return Some(length_marker(rendered));
        }
        match redact_text(rendered) {
            Cow::Owned(redacted) if redacted != rendered => Some(redacted),
            _ => None,
        }
    }

    /// The value of a metric label. `None` keeps it as it is. A label never carries the id of a party, an
    /// account or a case, so a value holding any UUID is replaced whole.
    pub fn label(&self, name: &str, value: &str) -> Option<String> {
        if self.is_classified(name) {
            return Some(length_marker(value));
        }
        if contains_uuid(value) {
            return Some(ID_MARKER.to_owned());
        }
        match redact_text(value) {
            Cow::Owned(redacted) if redacted != value => Some(redacted),
            _ => None,
        }
    }
}

const UUID_LEN: usize = 36;
const UUID_HYPHENS: [usize; 4] = [8, 13, 18, 23];

fn contains_uuid(value: &str) -> bool {
    value.as_bytes().windows(UUID_LEN).any(|window| {
        window.iter().enumerate().all(|(i, byte)| {
            if UUID_HYPHENS.contains(&i) {
                *byte == b'-'
            } else {
                byte.is_ascii_hexdigit()
            }
        })
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use akr_domain::classification::Classification;

    use super::*;

    fn row(column: &str, classification: Classification) -> ColumnClassification {
        ColumnClassification {
            schema_name: "canon".to_owned(),
            table_name: "party".to_owned(),
            column_name: column.to_owned(),
            classification,
            encrypted: false,
            keyed_hash: false,
            note: None,
        }
    }

    #[test]
    fn the_table_adds_the_four_redacted_classes_and_no_other() {
        let redactor = SharedRedactor::new();
        assert!(!redactor.is_classified("full_name"));
        assert!(redactor.is_classified("request_body"));

        assert!(redactor.load_classification(&[
            row("full_name", Classification::Personal),
            row("balance", Classification::Financial),
            row("wrapped", Classification::SecretStr),
            row("pan", Classification::Card),
            row("status", Classification::Internal),
            row("rule_body", Classification::DetectionLogic),
            row("code", Classification::Public),
        ]));
        for name in ["full_name", "balance", "wrapped", "pan", "request_body"] {
            assert!(redactor.is_classified(name), "{name}");
        }
        for name in ["status", "rule_body", "code"] {
            assert!(!redactor.is_classified(name), "{name}");
        }
    }

    #[test]
    fn the_table_is_loaded_once() {
        let redactor = SharedRedactor::new();
        assert!(redactor.load_classification(&[row("full_name", Classification::Personal)]));
        assert!(!redactor.load_classification(&[row("alias", Classification::Personal)]));
        assert!(redactor.is_classified("full_name"));
        assert!(!redactor.is_classified("alias"));
    }

    #[test]
    fn a_clone_sees_the_table_its_origin_loaded() {
        let redactor = SharedRedactor::new();
        let clone = redactor.clone();
        redactor.load_classification(&[row("full_name", Classification::Personal)]);
        assert_eq!(clone.field("full_name", "abc"), "[redacted:3]");
    }

    #[test]
    fn a_number_is_kept_unless_it_reads_as_a_card_or_an_id_number() {
        let redactor = SharedRedactor::new();
        assert_eq!(redactor.number("attempts", "3"), None);
        assert_eq!(redactor.number("elapsed_ms", "1532"), None);
        assert_eq!(
            redactor.number("n", "4111111111111111").unwrap(),
            "[redacted:card]"
        );
        assert_eq!(
            redactor.number("n", "199012345678").unwrap(),
            "[redacted:national-id]"
        );
        assert_eq!(redactor.number("token", "42").unwrap(), "[redacted:2]");
    }

    #[test]
    fn a_label_never_keeps_an_id() {
        let redactor = SharedRedactor::new();
        assert_eq!(redactor.label("kind", "maintenance_sweep"), None);
        assert_eq!(redactor.label("route", "/api/v1/cases/{id}"), None);
        assert_eq!(redactor.label("status", "200"), None);
        assert_eq!(
            redactor
                .label(
                    "route",
                    "/api/v1/cases/0198c1de-7b7e-7cc3-9c0b-0a2f4d5e6f70"
                )
                .unwrap(),
            ID_MARKER
        );
        assert_eq!(
            redactor
                .label("account", "IQ98NBIQ850123456789012")
                .unwrap(),
            "[redacted:iban]"
        );
        assert_eq!(redactor.label("secret", "abc").unwrap(), "[redacted:3]");
    }
}
