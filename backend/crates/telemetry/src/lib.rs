//! Logs, traces and metrics for every AKR-TMS binary (TS-27, TS-28).
//!
//! Three sinks: JSON lines on stdout, spans to the OTLP collector when one is configured, and the Prometheus
//! exposition. Redaction stands in front of each of them, so a classified value, an IBAN, a card-like number
//! or a national-ID number cannot leave the process through telemetry whatever a call site passes.

pub mod logs;
pub mod metrics;
pub mod redact;
pub mod traces;

use akr_common::config::CoreConfig;
use akr_domain::classification::ColumnClassification;
use metrics_exporter_prometheus::PrometheusHandle;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub use logs::JsonLayer;
pub use metrics_exporter_prometheus::PrometheusHandle as MetricsHandle;
pub use redact::SharedRedactor;
pub use traces::TraceExport;

/// What is logged when `RUST_LOG` is not set.
const DEFAULT_FILTER: &str = "info";

#[derive(Debug, thiserror::Error)]
pub enum TelemetryError {
    #[error("OTLP_ENDPOINT {endpoint}: the span exporter cannot be built: {reason}")]
    Exporter { endpoint: String, reason: String },
    #[error("the metric buckets are not valid: {0}")]
    Buckets(String),
    #[error("the metrics upkeep thread cannot start: {0}")]
    Upkeep(String),
    #[error("telemetry is already installed in this process")]
    AlreadyInstalled,
}

/// The telemetry of a running binary. Keep it for the life of the process: dropping it sends the last
/// batch of spans.
pub struct Telemetry {
    metrics: PrometheusHandle,
    redactor: SharedRedactor,
    traces: Option<TraceExport>,
}

impl Telemetry {
    /// Installs the process-wide subscriber and metrics recorder. Once per process.
    ///
    /// An `OTLP_ENDPOINT` the exporter cannot be built for is an error: a binary told to send traces
    /// refuses to run without them.
    pub fn init(core: &CoreConfig) -> Result<Self, TelemetryError> {
        let service = core.binary.name();
        let redactor = SharedRedactor::new();
        let filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_FILTER));

        let (otel, traces) = match traces::otlp_layer(
            core.otlp_endpoint.as_ref().map(|url| url.as_str()),
            service,
            core.env,
            &core.instance_id,
            redactor.clone(),
        )? {
            Some((layer, export)) => (Some(layer), Some(export)),
            None => (None, None),
        };

        tracing_subscriber::registry()
            .with(filter)
            .with(otel)
            .with(JsonLayer::stdout(
                service,
                &core.instance_id,
                redactor.clone(),
            ))
            .try_init()
            .map_err(|_| TelemetryError::AlreadyInstalled)?;

        let metrics = metrics::install(redactor.clone())?;
        if traces.is_some() {
            tracing::info!("exporting spans over OTLP/HTTP");
        }
        Ok(Self {
            metrics,
            redactor,
            traces,
        })
    }

    /// The handle `GET /metrics` renders.
    pub fn metrics(&self) -> PrometheusHandle {
        self.metrics.clone()
    }

    /// Gives the redactor the classification table. A binary calls it once, as soon as it has a database.
    pub fn load_classification(&self, rows: &[ColumnClassification]) {
        if self.redactor.load_classification(rows) {
            tracing::info!(
                columns = rows.len(),
                "classification table loaded for redaction"
            );
        }
    }

    /// Sends the spans batched so far.
    pub fn flush(&self) {
        if let Some(traces) = &self.traces {
            traces.flush();
        }
    }
}
