//! Prometheus metrics (TS-28).
//!
//! The recorder every `metrics::counter!`, `gauge!` and `histogram!` call reaches is wrapped by
//! [`RedactingRecorder`], which cleans label values before a series is registered: a label never carries
//! the id of a party, an account or a case, nor any classified value.
//!
//! The text exposition is rendered from the handle by `GET /metrics` on the management listener.

use std::time::Duration;

use metrics::{
    Counter, Gauge, Histogram, Key, KeyName, Label, Metadata, Recorder, SharedString, Unit,
};
use metrics_exporter_prometheus::{
    Matcher, PrometheusBuilder, PrometheusHandle, PrometheusRecorder,
};

use crate::TelemetryError;
use crate::redact::SharedRedactor;

pub const HTTP_REQUESTS_TOTAL: &str = "akr_http_requests_total";
pub const HTTP_REQUEST_DURATION_SECONDS: &str = "akr_http_request_duration_seconds";
pub const IDEMPOTENCY_REPLAYS_TOTAL: &str = "akr_idempotency_replays_total";
pub const S3GUARD_REJECTED_TOTAL: &str = "akr_s3guard_rejected_total";
pub const BACKUP_AGE_SECONDS: &str = "akr_backup_age_seconds";
pub const RESTORE_DRILL_AGE_DAYS: &str = "akr_restore_drill_age_days";
pub const SWEEP_DELETED_TOTAL: &str = "akr_sweep_deleted_total";

/// Request latency, in seconds.
const LATENCY_BUCKETS: &[f64] = &[
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];

/// A job runs from milliseconds to the better part of an hour.
const JOB_BUCKETS: &[f64] = &[
    0.05, 0.25, 1.0, 5.0, 15.0, 60.0, 300.0, 900.0, 1800.0, 3600.0,
];

const JOB_DURATION_SECONDS: &str = "akr_job_duration_seconds";

/// Histograms are drained into their buckets this often.
const UPKEEP_INTERVAL: Duration = Duration::from_secs(5);

fn builder() -> Result<PrometheusBuilder, TelemetryError> {
    PrometheusBuilder::new()
        .set_buckets(LATENCY_BUCKETS)
        .and_then(|b| {
            b.set_buckets_for_metric(Matcher::Full(JOB_DURATION_SECONDS.to_owned()), JOB_BUCKETS)
        })
        .map_err(|e| TelemetryError::Buckets(e.to_string()))
}

/// Installs the process-wide recorder and returns the handle `GET /metrics` renders. Once per process.
pub fn install(redactor: SharedRedactor) -> Result<PrometheusHandle, TelemetryError> {
    let (recorder, handle) = detached(redactor)?;
    metrics::set_global_recorder(recorder).map_err(|_| TelemetryError::AlreadyInstalled)?;

    let upkeep = handle.clone();
    std::thread::Builder::new()
        .name("akr-metrics-upkeep".to_owned())
        .spawn(move || {
            loop {
                std::thread::sleep(UPKEEP_INTERVAL);
                upkeep.run_upkeep();
            }
        })
        .map_err(|e| TelemetryError::Upkeep(e.to_string()))?;
    Ok(handle)
}

/// A recorder that is not installed, with its handle. Tests build many applications in one process, and
/// only one recorder can be the process-wide one; a test records through
/// `metrics::with_local_recorder`.
pub fn detached(
    redactor: SharedRedactor,
) -> Result<(RedactingRecorder<PrometheusRecorder>, PrometheusHandle), TelemetryError> {
    let recorder = builder()?.build_recorder();
    let handle = recorder.handle();
    Ok((RedactingRecorder::new(recorder, redactor), handle))
}

/// Cleans the label values of a series before the recorder it wraps registers it.
pub struct RedactingRecorder<R> {
    inner: R,
    redactor: SharedRedactor,
}

impl<R> RedactingRecorder<R> {
    pub fn new(inner: R, redactor: SharedRedactor) -> Self {
        Self { inner, redactor }
    }

    /// `None` when every label may stay as it is, which is the case for every series the product records.
    fn cleaned(&self, key: &Key) -> Option<Key> {
        if !key
            .labels()
            .any(|label| self.redactor.label(label.key(), label.value()).is_some())
        {
            return None;
        }
        let labels: Vec<Label> = key
            .labels()
            .map(|label| {
                let value = self
                    .redactor
                    .label(label.key(), label.value())
                    .unwrap_or_else(|| label.value().to_owned());
                Label::new(label.key().to_owned(), value)
            })
            .collect();
        Some(Key::from_parts(key.name().to_owned(), labels))
    }
}

impl<R: Recorder> Recorder for RedactingRecorder<R> {
    fn describe_counter(&self, key: KeyName, unit: Option<Unit>, description: SharedString) {
        self.inner.describe_counter(key, unit, description);
    }

    fn describe_gauge(&self, key: KeyName, unit: Option<Unit>, description: SharedString) {
        self.inner.describe_gauge(key, unit, description);
    }

    fn describe_histogram(&self, key: KeyName, unit: Option<Unit>, description: SharedString) {
        self.inner.describe_histogram(key, unit, description);
    }

    fn register_counter(&self, key: &Key, metadata: &Metadata<'_>) -> Counter {
        match self.cleaned(key) {
            Some(cleaned) => self.inner.register_counter(&cleaned, metadata),
            None => self.inner.register_counter(key, metadata),
        }
    }

    fn register_gauge(&self, key: &Key, metadata: &Metadata<'_>) -> Gauge {
        match self.cleaned(key) {
            Some(cleaned) => self.inner.register_gauge(&cleaned, metadata),
            None => self.inner.register_gauge(key, metadata),
        }
    }

    fn register_histogram(&self, key: &Key, metadata: &Metadata<'_>) -> Histogram {
        match self.cleaned(key) {
            Some(cleaned) => self.inner.register_histogram(&cleaned, metadata),
            None => self.inner.register_histogram(key, metadata),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn a_series_with_plain_labels_is_rendered_as_recorded() {
        let (recorder, handle) = detached(SharedRedactor::new()).unwrap();
        metrics::with_local_recorder(&recorder, || {
            metrics::counter!(HTTP_REQUESTS_TOTAL, "method" => "GET", "route" => "/healthz", "status" => "200")
                .increment(2);
            metrics::gauge!(BACKUP_AGE_SECONDS).set(42.0);
        });
        let text = handle.render();
        assert!(
            text.contains(
                r#"akr_http_requests_total{method="GET",route="/healthz",status="200"} 2"#
            ),
            "{text}"
        );
        assert!(text.contains("akr_backup_age_seconds 42"), "{text}");
    }

    #[test]
    fn a_job_duration_falls_in_the_job_buckets() {
        let (recorder, handle) = detached(SharedRedactor::new()).unwrap();
        metrics::with_local_recorder(&recorder, || {
            metrics::histogram!(JOB_DURATION_SECONDS, "kind" => "maintenance_sweep").record(120.0);
            metrics::histogram!(HTTP_REQUEST_DURATION_SECONDS, "route" => "/readyz").record(0.02);
        });
        let text = handle.render();
        assert!(
            text.contains(
                r#"akr_job_duration_seconds_bucket{kind="maintenance_sweep",le="300"} 1"#
            ),
            "{text}"
        );
        assert!(
            text.contains(
                r#"akr_http_request_duration_seconds_bucket{route="/readyz",le="0.025"} 1"#
            ),
            "{text}"
        );
    }

    #[test]
    fn a_label_that_holds_an_id_is_replaced() {
        let (recorder, handle) = detached(SharedRedactor::new()).unwrap();
        metrics::with_local_recorder(&recorder, || {
            metrics::counter!(
                "akr_test_total",
                "case" => "0198c1de-7b7e-7cc3-9c0b-0a2f4d5e6f70",
                "kind" => "plain"
            )
            .increment(1);
        });
        let text = handle.render();
        assert!(!text.contains("0198c1de"), "{text}");
        assert!(text.contains(r#"kind="plain""#), "{text}");
    }
}
