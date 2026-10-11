//! Repository ports. The PostgreSQL adapters are in `akr-db`.
//!
//! A method that changes a row takes the caller's transaction: the change, its audit event and the jobs it
//! causes commit together or not at all.

use async_trait::async_trait;
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::classification::ColumnClassification;
use crate::errors::DomainError;
use crate::flags::{FeatureFlag, UpdateFlag};
use crate::retention::RetentionAssignment;
use crate::settings::Setting;

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Setting>, DomainError>;

    async fn list(&self) -> Result<Vec<Setting>, DomainError>;

    /// Reads the row under a lock held until the transaction ends, so the value validated is the value
    /// replaced.
    async fn lock(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        key: &str,
    ) -> Result<Option<Setting>, DomainError>;

    /// Writes the value when the row is still at `expected_version`; otherwise
    /// [`DomainError::VersionConflict`] with the version it has now.
    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        key: &str,
        value: &Value,
        expected_version: i32,
        actor: Option<Uuid>,
    ) -> Result<Setting, DomainError>;
}

#[async_trait]
pub trait FlagsRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<FeatureFlag>, DomainError>;

    async fn get(&self, key: &str) -> Result<Option<FeatureFlag>, DomainError>;

    async fn lock(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        key: &str,
    ) -> Result<Option<FeatureFlag>, DomainError>;

    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        key: &str,
        input: &UpdateFlag,
        expected_version: i32,
        actor: Option<Uuid>,
    ) -> Result<FeatureFlag, DomainError>;
}

#[async_trait]
pub trait ClassificationRepository: Send + Sync {
    async fn for_table(
        &self,
        schema: &str,
        table: &str,
    ) -> Result<Vec<ColumnClassification>, DomainError>;

    /// Every row of the registry. The redaction layer loads it once at start.
    async fn list(&self) -> Result<Vec<ColumnClassification>, DomainError>;

    /// `(schema, table, column)` of every column in the AKR schemas that has no classification row.
    async fn uncovered_columns(&self) -> Result<Vec<(String, String, String)>, DomainError>;
}

#[async_trait]
pub trait RetentionAssignmentRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<RetentionAssignment>, DomainError>;

    /// `(schema, table)` of every table in the AKR schemas that has no retention assignment.
    async fn uncovered_tables(&self) -> Result<Vec<(String, String)>, DomainError>;
}
