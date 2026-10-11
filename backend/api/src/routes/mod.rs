//! Route modules. Every handler is registered through `routes!`, so a route cannot exist outside the
//! OpenAPI document.

pub mod echo;
pub mod infra;

/// The tag of the liveness and readiness routes.
pub const INFRA_TAG: &str = "infra";
