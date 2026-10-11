//! The auth slot.
//!
//! Until phase 01 brings sessions and tokens, a caller outside production names itself with the header
//! `X-Dev-Principal`, so the principal-scoped idempotency ledger can be exercised. With the stub off
//! nobody can be a principal, and every route behind the slot answers 401.

use akr_common::AppError;
use akr_common::config::Env;
use akr_common::idempotency::PrincipalId;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

use crate::config::ApiConfig;

pub const DEV_PRINCIPAL_HEADER: &str = "x-dev-principal";

#[derive(Debug, thiserror::Error)]
#[error("the X-Dev-Principal stub cannot run in production")]
pub struct StubRefused;

/// Takes the caller's word for who it is.
#[derive(Debug, Clone)]
pub struct DevPrincipalStub(());

impl DevPrincipalStub {
    pub fn new(env: Env) -> Result<Self, StubRefused> {
        if env.is_production() {
            return Err(StubRefused);
        }
        Ok(Self(()))
    }
}

#[derive(Debug, Clone)]
pub enum AuthSlot {
    Dev(DevPrincipalStub),
    /// No way to authenticate exists yet.
    Closed,
}

impl AuthSlot {
    pub fn from_config(config: &ApiConfig) -> Result<Self, StubRefused> {
        if config.core.dev_principal {
            Ok(AuthSlot::Dev(DevPrincipalStub::new(config.env())?))
        } else {
            Ok(AuthSlot::Closed)
        }
    }
}

pub async fn layer(State(slot): State<AuthSlot>, mut req: Request, next: Next) -> Response {
    let AuthSlot::Dev(_) = slot else {
        return AppError::Unauthenticated.into_response();
    };
    let Some(sent) = req.headers().get(DEV_PRINCIPAL_HEADER) else {
        return AppError::Unauthenticated.into_response();
    };
    let Some(id) = sent
        .to_str()
        .ok()
        .and_then(|value| Uuid::try_parse(value.trim()).ok())
    else {
        return AppError::BadRequest("dev_principal_malformed").into_response();
    };
    req.extensions_mut().insert(PrincipalId(id));
    next.run(req).await
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn the_stub_refuses_production() {
        assert!(DevPrincipalStub::new(Env::Production).is_err());
        for env in [Env::Development, Env::Test, Env::Staging] {
            assert!(DevPrincipalStub::new(env).is_ok());
        }
    }
}
