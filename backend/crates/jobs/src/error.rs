//! The failures of the harness.

/// How a handler failed. The harness decides from it whether the job runs again.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum JobError {
    /// Worth another attempt: a dependency was down, a lock was held, the claim was lost.
    #[error("{0}")]
    Retryable(String),
    /// No attempt can succeed: the payload is wrong, the owner is gone.
    #[error("{0}")]
    Permanent(String),
}

impl JobError {
    pub fn retryable(cause: impl std::fmt::Display) -> Self {
        JobError::Retryable(cause.to_string())
    }

    pub fn permanent(cause: impl std::fmt::Display) -> Self {
        JobError::Permanent(cause.to_string())
    }

    pub fn message(&self) -> &str {
        match self {
            JobError::Retryable(m) | JobError::Permanent(m) => m,
        }
    }
}

impl From<sqlx::Error> for JobError {
    fn from(error: sqlx::Error) -> Self {
        JobError::Retryable(error.to_string())
    }
}

impl From<akr_domain::DomainError> for JobError {
    fn from(error: akr_domain::DomainError) -> Self {
        match error {
            akr_domain::DomainError::Database(e) => JobError::Retryable(e.to_string()),
            other => JobError::Permanent(other.to_string()),
        }
    }
}

impl From<akr_domain::audit::AuditError> for JobError {
    fn from(error: akr_domain::audit::AuditError) -> Self {
        match error {
            akr_domain::audit::AuditError::Database(e) => JobError::Retryable(e.to_string()),
            other => JobError::Permanent(other.to_string()),
        }
    }
}

/// Why work could not be put on a queue.
#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    /// A payload is a JSON object of ids and codes.
    #[error("the payload of `{kind}` is not a JSON object")]
    PayloadNotAnObject { kind: String },
    /// The timer kind does not list this kind of owner.
    #[error("a `{kind}` timer cannot belong to a `{owner_kind}`")]
    OwnerKind {
        kind: &'static str,
        owner_kind: String,
    },
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl From<QueueError> for akr_common::AppError {
    fn from(error: QueueError) -> Self {
        match error {
            QueueError::Database(e) => e.into(),
            other => akr_common::AppError::internal(other),
        }
    }
}

impl From<QueueError> for JobError {
    fn from(error: QueueError) -> Self {
        match error {
            QueueError::Database(e) => JobError::Retryable(e.to_string()),
            other => JobError::Permanent(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RegistryError {
    #[error("`{0}` is registered twice")]
    Duplicate(String),
    #[error("the handler is for `{handler}` and was registered as `{kind}`")]
    KindMismatch {
        kind: &'static str,
        handler: &'static str,
    },
    #[error("the timer kind `{timer}` must enqueue the job `timer_{timer}`, not `{job}`")]
    TimerJobName {
        timer: &'static str,
        job: &'static str,
    },
}
