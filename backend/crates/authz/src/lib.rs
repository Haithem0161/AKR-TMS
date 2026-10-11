//! The authorisation slot (gate 2).
//!
//! Phase 03 fills it with the Cedar engine. Until then the slot holds [`AllowAllAuthorizer`], which a
//! production binary cannot construct, so the stand-in cannot reach a deployed bank. The trait is the seam
//! `AppState` holds; phase 03 replaces the request and decision types here with its full ones.

use akr_common::config::Env;
use akr_domain::audit::{Effect, RequestCtx};
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AuthzError {
    #[error("the allow-all authorizer is refused in production")]
    AllowAllInProduction,
    #[error("the authorizer is unavailable: {0}")]
    Unavailable(String),
}

/// What an action is done to. Phase 03 adds the kinds it protects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceKind {
    System,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResourceRef {
    System,
}

#[derive(Debug, Clone)]
pub struct AuthzRequest<'a> {
    pub principal_id: Uuid,
    pub request: &'a RequestCtx,
    /// The action name, as in `"setting.update"`.
    pub action: &'a str,
    pub resource: ResourceRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub effect: Effect,
    /// The policies that determined the decision.
    pub policy_ids: Vec<String>,
}

#[async_trait]
pub trait Authorizer: Send + Sync {
    async fn authorize(&self, req: AuthzRequest<'_>) -> Result<Decision, AuthzError>;

    /// Drops the cached attributes of one resource; services call it after commit.
    async fn invalidate_resource(&self, kind: ResourceKind, id: Uuid);
}

/// Allows everything. Development and tests only.
#[derive(Debug)]
pub struct AllowAllAuthorizer(());

impl AllowAllAuthorizer {
    pub fn new(env: Env) -> Result<Self, AuthzError> {
        if env.is_production() {
            return Err(AuthzError::AllowAllInProduction);
        }
        Ok(Self(()))
    }
}

#[async_trait]
impl Authorizer for AllowAllAuthorizer {
    async fn authorize(&self, _req: AuthzRequest<'_>) -> Result<Decision, AuthzError> {
        Ok(Decision {
            effect: Effect::Allow,
            policy_ids: Vec::new(),
        })
    }

    async fn invalidate_resource(&self, _kind: ResourceKind, _id: Uuid) {}
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn production_cannot_construct_the_stand_in() {
        assert_eq!(
            AllowAllAuthorizer::new(Env::Production).unwrap_err(),
            AuthzError::AllowAllInProduction
        );
        for env in [Env::Development, Env::Test, Env::Staging] {
            assert!(AllowAllAuthorizer::new(env).is_ok(), "{env:?}");
        }
    }

    #[tokio::test]
    async fn every_request_is_allowed_and_names_no_policy() {
        let authz = AllowAllAuthorizer::new(Env::Test).unwrap();
        let request = RequestCtx::new(Uuid::now_v7());
        let decision = authz
            .authorize(AuthzRequest {
                principal_id: Uuid::now_v7(),
                request: &request,
                action: "setting.update",
                resource: ResourceRef::System,
            })
            .await
            .unwrap();
        assert_eq!(decision.effect, Effect::Allow);
        assert!(decision.policy_ids.is_empty());
        authz
            .invalidate_resource(ResourceKind::System, Uuid::now_v7())
            .await;
    }
}
