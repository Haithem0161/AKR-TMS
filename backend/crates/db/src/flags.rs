//! `admin.feature_flags`.

use akr_domain::DomainError;
use akr_domain::flags::{FeatureFlag, FlagScope, UpdateFlag};
use akr_domain::repositories::FlagsRepository;
use akr_domain::text::I18nText;
use async_trait::async_trait;
use sqlx::types::Json;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct PgFlagsRepository {
    pool: PgPool,
}

impl PgFlagsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FlagsRepository for PgFlagsRepository {
    async fn list(&self) -> Result<Vec<FeatureFlag>, DomainError> {
        let rows = sqlx::query_as!(
            FeatureFlag,
            r#"SELECT key, enabled, scope AS "scope: Json<FlagScope>", label AS "label: Json<I18nText>",
                      updated_by, updated_at, version
               FROM admin.feature_flags
               ORDER BY key"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn get(&self, key: &str) -> Result<Option<FeatureFlag>, DomainError> {
        let row = sqlx::query_as!(
            FeatureFlag,
            r#"SELECT key, enabled, scope AS "scope: Json<FlagScope>", label AS "label: Json<I18nText>",
                      updated_by, updated_at, version
               FROM admin.feature_flags
               WHERE key = $1"#,
            key
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn lock(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        key: &str,
    ) -> Result<Option<FeatureFlag>, DomainError> {
        let row = sqlx::query_as!(
            FeatureFlag,
            r#"SELECT key, enabled, scope AS "scope: Json<FlagScope>", label AS "label: Json<I18nText>",
                      updated_by, updated_at, version
               FROM admin.feature_flags
               WHERE key = $1
               FOR UPDATE"#,
            key
        )
        .fetch_optional(&mut **tx)
        .await?;
        Ok(row)
    }

    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        key: &str,
        input: &UpdateFlag,
        expected_version: i32,
        actor: Option<Uuid>,
    ) -> Result<FeatureFlag, DomainError> {
        let updated = sqlx::query_as!(
            FeatureFlag,
            r#"UPDATE admin.feature_flags
               SET enabled = COALESCE($2, enabled), scope = COALESCE($3, scope), updated_by = $5,
                   updated_at = now(), version = version + 1
               WHERE key = $1 AND version = $4
               RETURNING key, enabled, scope AS "scope: Json<FlagScope>", label AS "label: Json<I18nText>",
                         updated_by, updated_at, version"#,
            key,
            input.enabled,
            input.scope.as_ref().map(Json) as _,
            expected_version,
            actor
        )
        .fetch_optional(&mut **tx)
        .await?;
        if let Some(flag) = updated {
            return Ok(flag);
        }
        let current = sqlx::query_scalar!(
            "SELECT version FROM admin.feature_flags WHERE key = $1",
            key
        )
        .fetch_optional(&mut **tx)
        .await?;
        Err(match current {
            Some(current) => DomainError::VersionConflict { current },
            None => DomainError::NotFound,
        })
    }
}
