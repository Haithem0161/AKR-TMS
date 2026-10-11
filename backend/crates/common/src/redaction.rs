//! Redaction: what must never reach a log line, a trace attribute or a metric label (TS-27).
//!
//! Two rules. A field whose name the classification table marks `personal`, `financial`, `secret-str` or
//! `card`, or which is a request or response body, is replaced by a length marker. In every other string, an
//! IBAN, a card-like run of 13 to 19 digits and an Iraqi national-ID number are replaced where they stand.
//! `akr-telemetry` applies both before any sink.

use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::LazyLock;

use regex::{Captures, Regex};

/// Field names that are dropped whatever the classification table says: bodies and credentials.
const ALWAYS_CLASSIFIED: &[&str] = &[
    "authorization",
    "body",
    "cookie",
    "password",
    "payload",
    "request_body",
    "response_body",
    "secret",
    "secret_id",
    "set-cookie",
    "token",
];

/// A UUID comes first so that an id whose groups happen to be all digits is left alone.
static PATTERNS: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?P<uuid>\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b)",
        r"|(?P<iban>\b[A-Z]{2}[0-9]{2}(?: ?[A-Z0-9]{4}){2,7}(?: ?[A-Z0-9]{1,3})?\b)",
        r"|(?P<card>\b(?:[0-9][ -]?){12,18}[0-9]\b)",
        r"|(?P<nid>\b[0-9]{12}\b)",
    ))
    .ok()
});

pub const IBAN_MARKER: &str = "[redacted:iban]";
pub const CARD_MARKER: &str = "[redacted:card]";
pub const NATIONAL_ID_MARKER: &str = "[redacted:national-id]";

/// Replaces every IBAN, card-like digit run and national-ID number in `text`.
pub fn redact_text(text: &str) -> Cow<'_, str> {
    let Some(patterns) = PATTERNS.as_ref() else {
        return Cow::Owned(length_marker(text));
    };
    patterns.replace_all(text, |c: &Captures<'_>| {
        if c.name("iban").is_some() {
            IBAN_MARKER.to_owned()
        } else if c.name("card").is_some() {
            CARD_MARKER.to_owned()
        } else if c.name("nid").is_some() {
            NATIONAL_ID_MARKER.to_owned()
        } else {
            c[0].to_owned()
        }
    })
}

/// What stands in for a dropped value: its length and nothing else.
pub fn length_marker(value: &str) -> String {
    format!("[redacted:{}]", value.chars().count())
}

/// The field names whose values are dropped.
#[derive(Debug, Clone)]
pub struct Redactor {
    classified: HashSet<String>,
}

impl Redactor {
    /// A redactor that knows the bodies and credential names only. A binary uses it until it has read the
    /// classification table.
    pub fn baseline() -> Self {
        Self {
            classified: ALWAYS_CLASSIFIED.iter().map(|s| (*s).to_owned()).collect(),
        }
    }

    /// Adds the column names the classification table marks `personal`, `financial`, `secret-str` or `card`.
    pub fn with_classified<I, S>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.classified
            .extend(columns.into_iter().map(|c| c.as_ref().to_ascii_lowercase()));
        self
    }

    pub fn is_classified(&self, field: &str) -> bool {
        let field = field.to_ascii_lowercase();
        let leaf = field.rsplit('.').next().unwrap_or(&field);
        self.classified.contains(field.as_str()) || self.classified.contains(leaf)
    }

    /// The value of `field` as a sink may see it.
    pub fn field<'a>(&self, field: &str, value: &'a str) -> Cow<'a, str> {
        if self.is_classified(field) {
            Cow::Owned(length_marker(value))
        } else {
            redact_text(value)
        }
    }
}

impl Default for Redactor {
    fn default() -> Self {
        Self::baseline()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_iban_is_replaced_with_or_without_spaces() {
        for iban in [
            "IQ98NBIQ850123456789012",
            "IQ98 NBIQ 8501 2345 6789 012",
            "GB29NWBK60161331926819",
        ] {
            let line = format!("credit to {iban} failed");
            assert_eq!(
                redact_text(&line),
                format!("credit to {IBAN_MARKER} failed"),
                "{iban}"
            );
        }
    }

    #[test]
    fn a_card_like_run_is_replaced() {
        for card in [
            "4111111111111111",
            "4111 1111 1111 1111",
            "4111-1111-1111-1111",
            "1234567890123",
            "1234567890123456789",
        ] {
            let line = format!("pan={card};");
            assert_eq!(redact_text(&line), format!("pan={CARD_MARKER};"), "{card}");
        }
    }

    #[test]
    fn a_national_id_number_is_replaced() {
        assert_eq!(
            redact_text("holder 199012345678 not found"),
            format!("holder {NATIONAL_ID_MARKER} not found")
        );
    }

    #[test]
    fn ordinary_text_ids_and_times_are_left_alone() {
        for plain in [
            "job 0198c1de-7b7e-7cc3-9c0b-0a2f4d5e6f70 done in 1532 ms",
            "correlation 00000000-0000-0000-0000-000000000001",
            "all digits 12345678-1234-1234-1234-123456789012",
            "at 2026-10-11T09:05:07.000000Z attempt 3 of 5",
            "schedule 0 30 2 * * *",
            "port 5432, 200 rows",
        ] {
            assert_eq!(redact_text(plain), plain);
        }
    }

    #[test]
    fn several_values_in_one_line_are_all_replaced() {
        let out = redact_text("from IQ98NBIQ850123456789012 card 4111111111111111 id 199012345678");
        assert_eq!(
            out,
            format!("from {IBAN_MARKER} card {CARD_MARKER} id {NATIONAL_ID_MARKER}")
        );
    }

    #[test]
    fn a_classified_field_keeps_only_its_length() {
        let r = Redactor::baseline().with_classified(["full_name", "Narrative"]);
        assert_eq!(r.field("full_name", "a synthetic name"), "[redacted:16]");
        assert_eq!(r.field("party.full_name", "abc"), "[redacted:3]");
        assert_eq!(r.field("NARRATIVE", "x"), "[redacted:1]");
        assert_eq!(r.field("request_body", "{\"a\":1}"), "[redacted:7]");
        assert_eq!(r.field("authorization", "Bearer abc"), "[redacted:10]");
    }

    #[test]
    fn an_unclassified_field_is_still_scanned() {
        let r = Redactor::baseline();
        assert_eq!(r.field("message", "ok"), "ok");
        assert_eq!(
            r.field("message", "pan 4111111111111111"),
            format!("pan {CARD_MARKER}")
        );
    }
}
