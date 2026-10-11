use std::path::PathBuf;

use akr_common::AppError;
use akr_domain::DomainError;
use akr_domain::audit::AuditError;
use akr_objectstore::ObjectStoreError;

#[derive(Debug, thiserror::Error)]
pub enum CtlError {
    /// The command was given something it cannot use.
    #[error("{0}")]
    Usage(String),

    #[error("{}: {source}", path.display())]
    File {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("database: {0}")]
    Database(#[from] sqlx::Error),

    #[error("migrations: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("object store: {0}")]
    Store(#[from] ObjectStoreError),

    #[error("{0}")]
    App(#[from] AppError),

    #[error("{0}")]
    Domain(#[from] DomainError),

    #[error("audit: {0}")]
    Audit(#[from] AuditError),

    #[error("registry: {0}")]
    Registry(String),

    /// A precondition does not hold. `code` is stable; scripts and the release screen match on it.
    #[error("refused ({code}): {detail}")]
    Refused { code: &'static str, detail: String },

    #[error("{failed} of {total} checks failed")]
    ChecksFailed { failed: usize, total: usize },
}

impl CtlError {
    pub fn usage(message: impl Into<String>) -> Self {
        CtlError::Usage(message.into())
    }

    pub fn refused(code: &'static str, detail: impl Into<String>) -> Self {
        CtlError::Refused {
            code,
            detail: detail.into(),
        }
    }

    pub fn file(path: impl Into<PathBuf>) -> impl FnOnce(std::io::Error) -> Self {
        let path = path.into();
        move |source| CtlError::File { path, source }
    }

    /// The refusal code, when the error is a refusal.
    pub fn code(&self) -> Option<&'static str> {
        match self {
            CtlError::Refused { code, .. } => Some(code),
            _ => None,
        }
    }
}
