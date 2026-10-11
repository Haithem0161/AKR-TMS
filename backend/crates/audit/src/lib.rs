//! The audit sink of a running binary.
//!
//! The port (`AuditSink`, `AuditEvent`, the event kinds) is defined in `akr_domain::audit` and re-exported
//! here, so `audit::AuditSink` and `audit::kinds::SETTING_UPDATED` are the paths every phase writes. This
//! crate holds the sinks: [`TracingSink`] now, the PostgreSQL writer from phase 02.

mod tracing_sink;

pub use akr_domain::audit::*;
pub use tracing_sink::TracingSink;
