//! The failures a port or a domain rule can report. `akr-common` maps each to its problem+json code.

use serde_json::{Map, Value};

/// One field-level failure, located by JSON pointer (RFC 6901).
#[derive(Debug, Clone, PartialEq)]
pub struct FieldViolation {
    pub pointer: String,
    /// Stable machine code of the rule, e.g. `required`, `not_a_cron`, `not_positive`.
    pub code: &'static str,
    /// The facts the rule's sentence needs that only the server holds.
    pub params: Map<String, Value>,
}

impl FieldViolation {
    pub fn new(pointer: impl Into<String>, code: &'static str) -> Self {
        Self {
            pointer: pointer.into(),
            code,
            params: Map::new(),
        }
    }

    pub fn param(mut self, name: &str, value: impl Into<Value>) -> Self {
        self.params.insert(name.to_owned(), value.into());
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("not found")]
    NotFound,

    /// The row moved on since the caller read it. Carries the version it has now.
    #[error("version conflict (current version {current})")]
    VersionConflict { current: i32 },

    #[error("validation failed")]
    Validation(Vec<FieldViolation>),

    /// A value outside the regulatory limit of its setting.
    #[error("the value is outside the limit of `{key}`")]
    BelowFloor { key: String },

    /// The request is well-formed and the resource's current state refuses it.
    #[error("not allowed in this state: {0}")]
    NotAllowedInState(&'static str),

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}
