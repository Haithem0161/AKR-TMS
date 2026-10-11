//! Feature flags. A flag gates a feature, never a security boundary: the environment and Cedar do that.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::text::{I18nText, RoleCode};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FeatureFlag {
    pub key: String,
    pub enabled: bool,
    #[schema(value_type = FlagScope)]
    pub scope: Json<FlagScope>,
    #[schema(value_type = I18nText)]
    pub label: Json<I18nText>,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
}

/// Who a flag is on for. Empty lists mean everyone.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct FlagScope {
    #[serde(default)]
    pub branches: Vec<Uuid>,
    #[serde(default)]
    pub roles: Vec<RoleCode>,
}

impl FlagScope {
    pub fn is_everyone(&self) -> bool {
        self.branches.is_empty() && self.roles.is_empty()
    }

    /// True when the scope is empty, or names the branch or one of the roles.
    pub fn covers(&self, ctx: &FlagContext) -> bool {
        self.is_everyone()
            || ctx.branch_id.is_some_and(|b| self.branches.contains(&b))
            || ctx.roles.iter().any(|r| self.roles.contains(r))
    }
}

/// The caller a flag is evaluated for.
#[derive(Debug, Clone, Default)]
pub struct FlagContext {
    pub branch_id: Option<Uuid>,
    pub roles: Vec<RoleCode>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateFlag {
    pub enabled: Option<bool>,
    pub scope: Option<FlagScope>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn role(code: &str) -> RoleCode {
        RoleCode::try_from(code.to_owned()).unwrap()
    }

    #[test]
    fn an_empty_scope_covers_everyone() {
        assert!(FlagScope::default().covers(&FlagContext::default()));
    }

    #[test]
    fn a_scope_covers_its_branch_or_one_of_its_roles_and_nobody_else() {
        let branch = Uuid::now_v7();
        let scope = FlagScope {
            branches: vec![branch],
            roles: vec![role("mlro")],
        };
        let in_branch = FlagContext {
            branch_id: Some(branch),
            roles: vec![],
        };
        let with_role = FlagContext {
            branch_id: Some(Uuid::now_v7()),
            roles: vec![role("analyst"), role("mlro")],
        };
        let outsider = FlagContext {
            branch_id: Some(Uuid::now_v7()),
            roles: vec![role("analyst")],
        };
        assert!(scope.covers(&in_branch));
        assert!(scope.covers(&with_role));
        assert!(!scope.covers(&outsider));
        assert!(!scope.covers(&FlagContext::default()));
    }

    #[test]
    fn the_seeded_empty_object_reads_as_everyone() {
        let scope: FlagScope = serde_json::from_str("{}").unwrap();
        assert!(scope.is_everyone());
    }
}
