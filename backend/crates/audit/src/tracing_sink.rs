//! The stand-in sink: an event becomes a log line and nothing else.
//!
//! It keeps no record, so it is for development and tests only. A binary wired with it reports
//! `Wiring::tracing_sink`, which `CoreConfig::validate` refuses in production.

use async_trait::async_trait;
use chrono::Utc;
use serde_json::{Map, Value};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use akr_domain::audit::{AuditError, AuditEvent, AuditReceipt, AuditSink};

#[derive(Debug, Clone, Copy, Default)]
pub struct TracingSink;

impl TracingSink {
    pub fn new() -> Self {
        Self
    }

    fn write(event: &AuditEvent) -> AuditReceipt {
        let receipt = AuditReceipt {
            id: Uuid::now_v7(),
            occurred_at: Utc::now(),
            entry_hash: [0; 32],
        };
        let details: Map<String, Value> = event
            .details
            .iter()
            .map(|(key, value)| ((*key).to_owned(), value.to_json()))
            .collect();
        let details = Value::Object(details).to_string();
        tracing::info!(
            target: "akr_audit",
            audit_id = %receipt.id,
            kind = event.kind.name,
            resource_type = event.kind.resource_type,
            resource_id = event.resource.id.map(tracing::field::display),
            resource_key = event.resource.key.as_deref(),
            actor_id = event.actor.id().map(tracing::field::display),
            on_behalf_of = event.actor.on_behalf_of().map(tracing::field::display),
            case_id = event.case_id.map(tracing::field::display),
            job_id = event.request.job_id.map(tracing::field::display),
            correlation_id = %event.request.correlation_id,
            ua = event.request.ua.as_deref(),
            details,
            "audit event"
        );
        receipt
    }
}

#[async_trait]
impl AuditSink for TracingSink {
    async fn emit(
        &self,
        _tx: &mut Transaction<'_, Postgres>,
        event: AuditEvent,
    ) -> Result<AuditReceipt, AuditError> {
        Ok(Self::write(&event))
    }

    async fn emit_standalone(&self, event: AuditEvent) -> Result<AuditReceipt, AuditError> {
        Ok(Self::write(&event))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use akr_domain::audit::{Actor, RequestCtx, kinds};

    use super::*;

    #[tokio::test]
    async fn an_event_gets_a_receipt_with_a_fresh_id() {
        let sink = TracingSink::new();
        let event = || {
            AuditEvent::new(
                &kinds::RESTORE_DRILL_RECORDED,
                Actor::System,
                RequestCtx::cli(Uuid::now_v7(), "0.1.0"),
            )
            .resource_key("ops.last_restore_drill_at")
        };
        let first = sink.emit_standalone(event()).await.unwrap();
        let second = sink.emit_standalone(event()).await.unwrap();
        assert_ne!(first.id, second.id);
        assert_eq!(first.id.get_version_num(), 7);
    }
}
