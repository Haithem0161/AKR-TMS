//! `JobQueue`: the only way to create a job.

use std::sync::Arc;

use akr_common::correlation;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::error::QueueError;
use crate::kind::JobKind;
use crate::store::{JobInsert, JobStore};

/// The payload field that carries the correlation id of the request or job that caused the work.
pub const CORRELATION_FIELD: &str = "correlation_id";

/// A job to enqueue.
#[derive(Debug, Clone)]
pub struct NewJob {
    pub kind: &'static JobKind,
    /// A JSON object of ids and codes ([`crate::payload::check`]).
    pub payload: Value,
    /// When the job becomes due. `None`: now.
    pub run_at: Option<DateTime<Utc>>,
    pub max_attempts: i32,
    /// With a key, a second enqueue of the same kind and key while the first is still queued does nothing.
    pub coalesce_key: Option<String>,
}

impl NewJob {
    pub fn new(kind: &'static JobKind) -> Self {
        Self {
            kind,
            payload: Value::Object(serde_json::Map::new()),
            run_at: None,
            max_attempts: kind.max_attempts,
            coalesce_key: None,
        }
    }

    pub fn payload(mut self, payload: Value) -> Self {
        self.payload = payload;
        self
    }

    pub fn run_at(mut self, run_at: DateTime<Utc>) -> Self {
        self.run_at = Some(run_at);
        self
    }

    pub fn max_attempts(mut self, max_attempts: i32) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    pub fn coalesce_key(mut self, key: impl Into<String>) -> Self {
        self.coalesce_key = Some(key.into());
        self
    }
}

#[derive(Clone)]
pub struct JobQueue {
    store: Arc<dyn JobStore>,
}

impl JobQueue {
    pub fn new(store: Arc<dyn JobStore>) -> Self {
        Self { store }
    }

    /// Enqueues a job in the caller's transaction: the job exists exactly when the transaction commits.
    /// `None` when it was coalesced into a job already queued.
    pub async fn enqueue(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        job: NewJob,
    ) -> Result<Option<Uuid>, QueueError> {
        let payload = with_correlation(job.kind.name, job.payload)?;
        let id = self
            .store
            .enqueue(
                tx,
                JobInsert {
                    id: Uuid::now_v7(),
                    kind: job.kind.name,
                    payload: &payload,
                    run_at: job.run_at,
                    max_attempts: job.max_attempts,
                    coalesce_key: job.coalesce_key.as_deref(),
                },
            )
            .await?;
        Ok(id)
    }
}

/// Adds the correlation id of the current request or job to a payload that has none.
pub(crate) fn with_correlation(kind: &str, payload: Value) -> Result<Value, QueueError> {
    let Value::Object(mut fields) = payload else {
        return Err(QueueError::PayloadNotAnObject {
            kind: kind.to_owned(),
        });
    };
    if !fields.contains_key(CORRELATION_FIELD)
        && let Some(id) = correlation::current()
    {
        fields.insert(
            CORRELATION_FIELD.to_owned(),
            Value::String(id.as_uuid().to_string()),
        );
    }
    Ok(Value::Object(fields))
}

/// The correlation id a payload carries, or a new one.
pub(crate) fn correlation_of(payload: &Value) -> Uuid {
    payload
        .get(CORRELATION_FIELD)
        .and_then(Value::as_str)
        .and_then(|id| Uuid::parse_str(id).ok())
        .unwrap_or_else(Uuid::now_v7)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use akr_common::correlation::CorrelationId;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn a_payload_inherits_the_correlation_id_of_its_cause() {
        let cause = CorrelationId::generate();
        let payload = correlation::scope(cause, async {
            with_correlation("k", json!({"case_id": "x"})).unwrap()
        })
        .await;
        assert_eq!(correlation_of(&payload), cause.as_uuid());

        let own = Uuid::now_v7();
        let kept = correlation::scope(cause, async {
            with_correlation("k", json!({"correlation_id": own.to_string()})).unwrap()
        })
        .await;
        assert_eq!(correlation_of(&kept), own);
    }

    #[test]
    fn outside_a_request_a_payload_gets_no_id_and_a_reader_makes_one() {
        let payload = with_correlation("k", json!({})).unwrap();
        assert_eq!(payload, json!({}));
        assert_ne!(correlation_of(&payload), correlation_of(&payload));
    }

    #[test]
    fn a_payload_that_is_not_an_object_is_refused() {
        assert!(matches!(
            with_correlation("k", json!([1])),
            Err(QueueError::PayloadNotAnObject { .. })
        ));
    }
}
