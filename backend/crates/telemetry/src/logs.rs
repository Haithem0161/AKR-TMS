//! JSON logs, one object per line (TS-27).
//!
//! The layer writes the line itself, so no field reaches the output without passing the redactor: a
//! classified field keeps only its length, and every other value is scanned for an IBAN, a card-like number
//! and a national-ID number.
//!
//! A line holds `timestamp` (UTC), `level`, `service`, `instance`, `target`, `message`, `correlation_id`
//! when a span in scope carries one, `fields` (the event's own fields) and `spans` (root first, each with its
//! name and fields).

use std::fmt;
use std::io::Write;

use chrono::{SecondsFormat, Utc};
use serde_json::{Map, Number, Value};
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::registry::LookupSpan;

use crate::redact::SharedRedactor;

const MESSAGE: &str = "message";
const CORRELATION_ID: &str = "correlation_id";

pub struct JsonLayer<W> {
    service: &'static str,
    instance: String,
    redactor: SharedRedactor,
    writer: W,
}

impl JsonLayer<fn() -> std::io::Stdout> {
    pub fn stdout(service: &'static str, instance: &str, redactor: SharedRedactor) -> Self {
        Self::new(service, instance, redactor, std::io::stdout)
    }
}

impl<W> JsonLayer<W> {
    pub fn new(service: &'static str, instance: &str, redactor: SharedRedactor, writer: W) -> Self {
        Self {
            service,
            instance: instance.to_owned(),
            redactor,
            writer,
        }
    }
}

/// The redacted fields of a span, kept with the span for the lines written inside it.
struct SpanFields(Map<String, Value>);

struct Redacting<'a> {
    redactor: &'a SharedRedactor,
    out: &'a mut Map<String, Value>,
}

impl Redacting<'_> {
    fn text(&mut self, field: &Field, value: &str) {
        let shown = self.redactor.field(field.name(), value).into_owned();
        self.out
            .insert(field.name().to_owned(), Value::String(shown));
    }

    fn number(&mut self, field: &Field, rendered: String, number: Option<Number>) {
        let shown = match (self.redactor.number(field.name(), &rendered), number) {
            (Some(marker), _) => Value::String(marker),
            (None, Some(number)) => Value::Number(number),
            (None, None) => Value::String(rendered),
        };
        self.out.insert(field.name().to_owned(), shown);
    }
}

impl Visit for Redacting<'_> {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.text(field, value);
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.text(field, &format!("{value:?}"));
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.text(field, &value.to_string());
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        if self.redactor.is_classified(field.name()) {
            self.text(field, if value { "true" } else { "false" });
        } else {
            self.out.insert(field.name().to_owned(), Value::Bool(value));
        }
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.number(field, value.to_string(), Some(value.into()));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.number(field, value.to_string(), Some(value.into()));
    }

    fn record_i128(&mut self, field: &Field, value: i128) {
        self.number(field, value.to_string(), None);
    }

    fn record_u128(&mut self, field: &Field, value: u128) {
        self.number(field, value.to_string(), None);
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.number(field, value.to_string(), Number::from_f64(value));
    }
}

impl<S, W> Layer<S> for JsonLayer<W>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    W: for<'writer> MakeWriter<'writer> + 'static,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let Some(span) = ctx.span(id) else {
            return;
        };
        let mut fields = Map::new();
        attrs.record(&mut Redacting {
            redactor: &self.redactor,
            out: &mut fields,
        });
        span.extensions_mut().insert(SpanFields(fields));
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let Some(span) = ctx.span(id) else {
            return;
        };
        let mut extensions = span.extensions_mut();
        if let Some(SpanFields(fields)) = extensions.get_mut::<SpanFields>() {
            values.record(&mut Redacting {
                redactor: &self.redactor,
                out: fields,
            });
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut fields = Map::new();
        event.record(&mut Redacting {
            redactor: &self.redactor,
            out: &mut fields,
        });
        let message = fields.remove(MESSAGE);

        let mut correlation_id = None;
        let mut spans = Vec::new();
        if let Some(scope) = ctx.event_scope(event) {
            for span in scope.from_root() {
                let mut entry = Map::new();
                entry.insert("name".to_owned(), Value::String(span.name().to_owned()));
                if let Some(SpanFields(span_fields)) = span.extensions().get::<SpanFields>() {
                    for (name, value) in span_fields {
                        if name == CORRELATION_ID {
                            correlation_id = Some(value.clone());
                        }
                        entry.insert(name.clone(), value.clone());
                    }
                }
                spans.push(Value::Object(entry));
            }
        }

        let metadata = event.metadata();
        let mut line = Map::new();
        line.insert(
            "timestamp".to_owned(),
            Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true)),
        );
        line.insert(
            "level".to_owned(),
            Value::String(metadata.level().as_str().to_owned()),
        );
        line.insert("service".to_owned(), Value::String(self.service.to_owned()));
        line.insert("instance".to_owned(), Value::String(self.instance.clone()));
        line.insert(
            "target".to_owned(),
            Value::String(metadata.target().to_owned()),
        );
        if let Some(message) = message {
            line.insert(MESSAGE.to_owned(), message);
        }
        if let Some(correlation_id) = correlation_id {
            line.insert(CORRELATION_ID.to_owned(), correlation_id);
        }
        if !fields.is_empty() {
            line.insert("fields".to_owned(), Value::Object(fields));
        }
        if !spans.is_empty() {
            line.insert("spans".to_owned(), Value::Array(spans));
        }

        let Ok(mut bytes) = serde_json::to_vec(&Value::Object(line)) else {
            return;
        };
        bytes.push(b'\n');
        // A log line that cannot be written has nowhere to be reported.
        let _ = self.writer.make_writer().write_all(&bytes);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::sync::{Arc, Mutex};

    use tracing_subscriber::layer::SubscriberExt;

    use super::*;

    #[derive(Clone, Default)]
    struct Capture(Arc<Mutex<Vec<u8>>>);

    impl Write for Capture {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> MakeWriter<'a> for Capture {
        type Writer = Capture;

        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    fn lines(run: impl FnOnce()) -> Vec<Value> {
        let capture = Capture::default();
        let layer = JsonLayer::new("akr-test", "node-1", SharedRedactor::new(), capture.clone());
        tracing::subscriber::with_default(tracing_subscriber::registry().with(layer), run);
        let bytes = capture.0.lock().unwrap().clone();
        String::from_utf8(bytes)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    }

    #[test]
    fn a_line_names_its_service_its_level_and_its_message() {
        let out = lines(|| tracing::warn!(attempts = 3, ready = false, "the store is slow"));
        assert_eq!(out.len(), 1);
        let line = &out[0];
        assert_eq!(line["level"], "WARN");
        assert_eq!(line["service"], "akr-test");
        assert_eq!(line["instance"], "node-1");
        assert_eq!(line["message"], "the store is slow");
        assert_eq!(line["fields"]["attempts"], 3);
        assert_eq!(line["fields"]["ready"], false);
        assert!(line["timestamp"].as_str().unwrap().ends_with('Z'));
        assert!(line.get("spans").is_none());
    }

    #[test]
    fn a_line_inside_a_span_carries_the_correlation_id_and_the_span_fields() {
        let out = lines(|| {
            let outer = tracing::info_span!(
                "request",
                correlation_id = "0198c1de-7b7e-7cc3-9c0b-0a2f4d5e6f70",
                route = tracing::field::Empty,
            );
            let _outer = outer.enter();
            outer.record("route", "/api/v1/_echo");
            let inner = tracing::info_span!("handler", attempt = 2);
            let _inner = inner.enter();
            tracing::info!("done");
        });
        let line = &out[0];
        assert_eq!(
            line["correlation_id"],
            "0198c1de-7b7e-7cc3-9c0b-0a2f4d5e6f70"
        );
        assert_eq!(line["spans"][0]["name"], "request");
        assert_eq!(line["spans"][0]["route"], "/api/v1/_echo");
        assert_eq!(line["spans"][1]["name"], "handler");
        assert_eq!(line["spans"][1]["attempt"], 2);
    }

    #[test]
    fn a_body_field_keeps_only_its_length_and_a_number_that_reads_as_a_card_is_replaced() {
        let out = lines(|| {
            tracing::info!(
                request_body = "{\"a\":1}",
                n = 4111111111111111_u64,
                "pan 4111 1111 1111 1111 seen"
            );
        });
        let line = &out[0];
        assert_eq!(line["fields"]["request_body"], "[redacted:7]");
        assert_eq!(line["fields"]["n"], "[redacted:card]");
        assert_eq!(line["message"], "pan [redacted:card] seen");
    }
}
