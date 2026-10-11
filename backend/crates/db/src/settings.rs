//! `admin.settings`.

use akr_domain::DomainError;
use akr_domain::repositories::SettingsRepository;
use akr_domain::settings::{Setting, SettingFloor, SettingValueType};
use akr_domain::text::I18nText;
use async_trait::async_trait;
use serde_json::Value;
use sqlx::types::Json;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct PgSettingsRepository {
    pool: PgPool,
}

impl PgSettingsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsRepository for PgSettingsRepository {
    async fn get(&self, key: &str) -> Result<Option<Setting>, DomainError> {
        let row = sqlx::query_as!(
            Setting,
            r#"SELECT key, value, value_type AS "value_type: SettingValueType",
                      floor AS "floor: Json<SettingFloor>", governed, label AS "label: Json<I18nText>",
                      updated_by, updated_at, version
               FROM admin.settings
               WHERE key = $1"#,
            key
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn list(&self) -> Result<Vec<Setting>, DomainError> {
        let rows = sqlx::query_as!(
            Setting,
            r#"SELECT key, value, value_type AS "value_type: SettingValueType",
                      floor AS "floor: Json<SettingFloor>", governed, label AS "label: Json<I18nText>",
                      updated_by, updated_at, version
               FROM admin.settings
               ORDER BY key"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn lock(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        key: &str,
    ) -> Result<Option<Setting>, DomainError> {
        let row = sqlx::query_as!(
            Setting,
            r#"SELECT key, value, value_type AS "value_type: SettingValueType",
                      floor AS "floor: Json<SettingFloor>", governed, label AS "label: Json<I18nText>",
                      updated_by, updated_at, version
               FROM admin.settings
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
        value: &Value,
        expected_version: i32,
        actor: Option<Uuid>,
    ) -> Result<Setting, DomainError> {
        let updated = sqlx::query_as!(
            Setting,
            r#"UPDATE admin.settings
               SET value = $2, updated_by = $4, updated_at = now(), version = version + 1
               WHERE key = $1 AND version = $3
               RETURNING key, value, value_type AS "value_type: SettingValueType",
                         floor AS "floor: Json<SettingFloor>", governed, label AS "label: Json<I18nText>",
                         updated_by, updated_at, version"#,
            key,
            value,
            expected_version,
            actor
        )
        .fetch_optional(&mut **tx)
        .await?;
        if let Some(setting) = updated {
            return Ok(setting);
        }
        let current = sqlx::query_scalar!("SELECT version FROM admin.settings WHERE key = $1", key)
            .fetch_optional(&mut **tx)
            .await?;
        Err(match current {
            Some(current) => DomainError::VersionConflict { current },
            None => DomainError::NotFound,
        })
    }
}
