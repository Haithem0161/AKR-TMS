//! Correlation ids.
//!
//! Every request has one: the client's `X-Correlation-Id` when it is a UUID, otherwise a new UUIDv7. It goes
//! into the tracing span, the response header, every problem body, every job and outbox payload the request
//! causes and every audit entry, so an error a user saw can be followed to the logs and the audit trail.

use std::fmt;
use std::future::Future;

use uuid::Uuid;

/// The header that carries the id, inbound and outbound.
pub const CORRELATION_HEADER: &str = "x-correlation-id";

tokio::task_local! {
    static CURRENT: CorrelationId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CorrelationId(Uuid);

impl CorrelationId {
    pub fn generate() -> Self {
        CorrelationId(Uuid::now_v7())
    }

    /// The client's value when it is a UUID; otherwise a new id. A malformed header does not fail the
    /// request, it is just not trusted: only a UUID can be echoed into logs and the audit trail safely.
    pub fn from_header_or_generate(raw: Option<&str>) -> Self {
        raw.and_then(|v| Uuid::try_parse(v.trim()).ok())
            .map_or_else(Self::generate, CorrelationId)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for CorrelationId {
    fn from(value: Uuid) -> Self {
        CorrelationId(value)
    }
}

impl fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_hyphenated())
    }
}

/// Runs `fut` with `id` as the correlation id of its task. The correlation middleware wraps a request in
/// it, and the job harness wraps a handler in it.
pub async fn scope<F: Future>(id: CorrelationId, fut: F) -> F::Output {
    CURRENT.scope(id, fut).await
}

/// The correlation id of the running request or job, when there is one.
pub fn current() -> Option<CorrelationId> {
    CURRENT.try_with(|id| *id).ok()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn generated_ids_are_unique_and_sort_by_time() {
        let a = CorrelationId::generate();
        let b = CorrelationId::generate();
        assert_ne!(a, b);
        assert!(a.as_uuid() < b.as_uuid());
    }

    #[test]
    fn a_client_uuid_is_kept() {
        let sent = "0198c1de-7b7e-7cc3-9c0b-0a2f4d5e6f70";
        assert_eq!(
            CorrelationId::from_header_or_generate(Some(sent)).to_string(),
            sent
        );
        assert_eq!(
            CorrelationId::from_header_or_generate(Some(&format!("  {sent} "))).to_string(),
            sent
        );
    }

    #[test]
    fn anything_else_is_replaced() {
        for bad in ["", "req-1", "a\nb", "\"quoted\"", &"x".repeat(300)] {
            let id = CorrelationId::from_header_or_generate(Some(bad));
            assert_ne!(id.to_string(), bad);
            assert_eq!(id.as_uuid().get_version_num(), 7);
        }
        assert_eq!(
            CorrelationId::from_header_or_generate(None)
                .as_uuid()
                .get_version_num(),
            7
        );
    }

    #[test]
    fn the_header_name_is_lowercase() {
        assert_eq!(CORRELATION_HEADER, CORRELATION_HEADER.to_lowercase());
    }

    #[tokio::test]
    async fn the_id_is_visible_inside_its_scope_only() {
        assert_eq!(current(), None);
        let id = CorrelationId::generate();
        let seen = scope(id, async { current() }).await;
        assert_eq!(seen, Some(id));
        assert_eq!(current(), None);
    }
}
