//! The request pipeline of `/api/v1`, outermost first: body limit, correlation, trace, metrics, the auth
//! slot, the authorisation slot (phase 03), idempotency, then the handler.

pub mod correlation;
pub mod idempotency;
pub mod metrics;
pub mod principal;
