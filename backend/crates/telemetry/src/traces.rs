//! Span export over OTLP/HTTP (TS-27).
//!
//! `OTLP_ENDPOINT` is the collector's base URL (`http://otel-collector:4318`); `/v1/traces` is appended
//! here. Bodies are protobuf over HTTP, sent by the client the workspace already builds.
//!
//! Every span passes [`RedactingExporter`] on its way out: its name, its attributes, its events and its
//! status are redacted before the collector's client sees them.
//!
//! Telemetry never takes the bank down. Spans are batched on a background thread; a collector that is down
//! or slow costs the spans that would have gone to it and nothing else.

use std::borrow::Cow;
use std::time::Duration;

use akr_common::config::Env;
use opentelemetry::trace::{Status, TracerProvider as _};
use opentelemetry::{KeyValue, Value};
use opentelemetry_otlp::{Protocol, SpanExporter as OtlpSpanExporter, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::trace::{SdkTracer, SdkTracerProvider, SpanData, SpanExporter};
use tracing::Subscriber;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::registry::LookupSpan;

use crate::TelemetryError;
use crate::redact::SharedRedactor;

/// How long one export may take before it is abandoned: the batch it carries is already late and the next
/// one is queued behind it.
const EXPORT_TIMEOUT: Duration = Duration::from_secs(5);

const TRACES_PATH: &str = "/v1/traces";

/// Holds the span pipeline open and flushes it when dropped. A binary keeps it for the life of the process.
pub struct TraceExport {
    provider: SdkTracerProvider,
}

impl TraceExport {
    /// Sends what is batched now, without shutting the pipeline down.
    pub fn flush(&self) {
        if let Err(e) = self.provider.force_flush() {
            eprintln!("telemetry: could not flush spans: {e}");
        }
    }
}

impl Drop for TraceExport {
    fn drop(&mut self) {
        if let Err(e) = self.provider.shutdown() {
            eprintln!("telemetry: could not flush spans on shutdown: {e}");
        }
    }
}

/// The URL spans are posted to. The base is accepted with or without a trailing slash, and so is a value
/// that already names the traces path.
pub fn traces_url(endpoint: &str) -> String {
    let base = endpoint.trim().trim_end_matches('/');
    if base.ends_with(TRACES_PATH) {
        base.to_owned()
    } else {
        format!("{base}{TRACES_PATH}")
    }
}

pub type OtlpLayer<S> = OpenTelemetryLayer<S, SdkTracer>;

/// The OTLP layer for a subscriber and the guard that keeps it exporting; `None` when no collector is
/// configured.
pub fn otlp_layer<S>(
    endpoint: Option<&str>,
    service: &'static str,
    env: Env,
    instance: &str,
    redactor: SharedRedactor,
) -> Result<Option<(OtlpLayer<S>, TraceExport)>, TelemetryError>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    let Some(endpoint) = endpoint.map(str::trim).filter(|e| !e.is_empty()) else {
        return Ok(None);
    };

    let exporter = OtlpSpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .with_endpoint(traces_url(endpoint))
        .with_timeout(EXPORT_TIMEOUT)
        .build()
        .map_err(|e| TelemetryError::Exporter {
            endpoint: endpoint.to_owned(),
            reason: e.to_string(),
        })?;

    let resource = Resource::builder()
        .with_service_name(service)
        .with_attributes([
            KeyValue::new("deployment.environment.name", env.as_str()),
            KeyValue::new("service.instance.id", instance.to_owned()),
        ])
        .build();

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(RedactingExporter::new(exporter, redactor))
        .with_resource(resource)
        .build();
    let tracer = provider.tracer(service);

    Ok(Some((
        tracing_opentelemetry::layer().with_tracer(tracer),
        TraceExport { provider },
    )))
}

/// Redacts every span before the exporter it wraps sees it.
#[derive(Debug)]
pub struct RedactingExporter<E> {
    inner: E,
    redactor: SharedRedactor,
}

impl<E> RedactingExporter<E> {
    pub fn new(inner: E, redactor: SharedRedactor) -> Self {
        Self { inner, redactor }
    }

    fn text(&self, text: &mut Cow<'static, str>) {
        if let Cow::Owned(redacted) = self.redactor.text(text)
            && redacted != *text
        {
            *text = Cow::Owned(redacted);
        }
    }

    fn attributes(&self, attributes: &mut [KeyValue]) {
        for attribute in attributes {
            let rendered = attribute.value.as_str();
            let shown = self.redactor.field(attribute.key.as_str(), &rendered);
            if shown != rendered {
                attribute.value = Value::String(shown.into_owned().into());
            }
        }
    }

    fn redact(&self, mut span: SpanData) -> SpanData {
        self.text(&mut span.name);
        self.attributes(&mut span.attributes);
        for event in &mut span.events.events {
            self.text(&mut event.name);
            self.attributes(&mut event.attributes);
        }
        for link in &mut span.links.links {
            self.attributes(&mut link.attributes);
        }
        if let Status::Error { description } = &mut span.status {
            self.text(description);
        }
        span
    }
}

impl<E: SpanExporter> SpanExporter for RedactingExporter<E> {
    fn export(
        &self,
        batch: Vec<SpanData>,
    ) -> impl std::future::Future<Output = OTelSdkResult> + Send {
        let batch = batch.into_iter().map(|span| self.redact(span)).collect();
        self.inner.export(batch)
    }

    fn shutdown_with_timeout(&self, timeout: Duration) -> OTelSdkResult {
        self.inner.shutdown_with_timeout(timeout)
    }

    fn shutdown(&self) -> OTelSdkResult {
        self.inner.shutdown()
    }

    fn force_flush(&self) -> OTelSdkResult {
        self.inner.force_flush()
    }

    fn set_resource(&mut self, resource: &Resource) {
        self.inner.set_resource(resource);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn a_base_endpoint_gets_the_traces_path() {
        assert_eq!(traces_url("http://otel:4318"), "http://otel:4318/v1/traces");
        assert_eq!(
            traces_url("http://otel:4318///"),
            "http://otel:4318/v1/traces"
        );
        assert_eq!(
            traces_url("https://collector.bank/otlp"),
            "https://collector.bank/otlp/v1/traces"
        );
    }

    #[test]
    fn an_endpoint_that_already_names_the_path_is_kept() {
        assert_eq!(
            traces_url(" http://otel:4318/v1/traces/ "),
            "http://otel:4318/v1/traces"
        );
    }

    #[test]
    fn no_endpoint_means_no_layer() {
        for endpoint in [None, Some("   ")] {
            let built = otlp_layer::<tracing_subscriber::Registry>(
                endpoint,
                "akr-test",
                Env::Test,
                "node-1",
                SharedRedactor::new(),
            )
            .unwrap();
            assert!(built.is_none());
        }
    }
}
