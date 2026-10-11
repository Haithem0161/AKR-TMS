//! Job handlers and the registry of the kinds a binary runs.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use akr_domain::audit::{Actor, RequestCtx};
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use tokio::sync::Semaphore;
use uuid::Uuid;

use crate::error::{JobError, RegistryError};
use crate::kind::JobKind;
use crate::store::JobStore;

/// What a handler knows about the job it runs. A handler has no session: it acts as the system principal.
#[derive(Clone)]
pub struct JobContext {
    pub job_id: Uuid,
    pub kind: &'static str,
    /// The number of this attempt, from 1. It names this worker's claim.
    pub attempt: i32,
    pub correlation_id: Uuid,
    store: Arc<dyn JobStore>,
    completed: Arc<AtomicBool>,
}

impl JobContext {
    pub(crate) fn new(
        job_id: Uuid,
        kind: &'static str,
        attempt: i32,
        correlation_id: Uuid,
        store: Arc<dyn JobStore>,
    ) -> Self {
        Self {
            job_id,
            kind,
            attempt,
            correlation_id,
            store,
            completed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn actor(&self) -> Actor {
        Actor::System
    }

    /// The request context of the audit events this job emits.
    pub fn request(&self) -> RequestCtx {
        RequestCtx::system(self.job_id, self.correlation_id, self.kind)
    }

    /// Marks the job done inside the handler's transaction, so its effects and its completion commit
    /// together (DR-412). When the claim is gone (the lease reaper queued the job again, or another worker
    /// holds it) this is `Retryable`: the handler returns it and its transaction rolls back.
    pub async fn complete_in(&self, tx: &mut Transaction<'_, Postgres>) -> Result<(), JobError> {
        if self
            .store
            .complete_in(tx, self.job_id, self.attempt)
            .await?
        {
            self.completed.store(true, Ordering::SeqCst);
            Ok(())
        } else {
            Err(JobError::Retryable(
                "the claim on the job was lost".to_owned(),
            ))
        }
    }

    pub(crate) fn completed_in_handler(&self) -> bool {
        self.completed.load(Ordering::SeqCst)
    }
}

/// The code of one job kind. `run` is idempotent: a job may run more than once.
pub trait JobHandler: Send + Sync + 'static {
    const KIND: &'static str;

    fn run(
        &self,
        ctx: JobContext,
        payload: Value,
    ) -> impl Future<Output = Result<(), JobError>> + Send;
}

type Run = Pin<Box<dyn Future<Output = Result<(), JobError>> + Send>>;

pub(crate) trait ErasedHandler: Send + Sync {
    fn run(self: Arc<Self>, ctx: JobContext, payload: Value) -> Run;
}

impl<H: JobHandler> ErasedHandler for H {
    fn run(self: Arc<Self>, ctx: JobContext, payload: Value) -> Run {
        Box::pin(async move { JobHandler::run(&*self, ctx, payload).await })
    }
}

pub(crate) struct Registered {
    pub kind: &'static JobKind,
    pub handler: Arc<dyn ErasedHandler>,
    /// The local bound on jobs of the kind running in this process.
    pub permits: Arc<Semaphore>,
}

/// The kinds a binary runs, each with its handler.
#[derive(Default)]
pub struct JobRegistry {
    entries: HashMap<&'static str, Registered>,
}

impl JobRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<H: JobHandler>(
        &mut self,
        kind: &'static JobKind,
        handler: H,
    ) -> Result<(), RegistryError> {
        if kind.name != H::KIND {
            return Err(RegistryError::KindMismatch {
                kind: kind.name,
                handler: H::KIND,
            });
        }
        if self.entries.contains_key(kind.name) {
            return Err(RegistryError::Duplicate(kind.name.to_owned()));
        }
        self.entries.insert(
            kind.name,
            Registered {
                kind,
                handler: Arc::new(handler),
                permits: Arc::new(Semaphore::new(kind.concurrency.max(1) as usize)),
            },
        );
        Ok(())
    }

    pub fn kinds(&self) -> Vec<&'static JobKind> {
        let mut kinds: Vec<_> = self.entries.values().map(|e| e.kind).collect();
        kinds.sort_by_key(|k| k.name);
        kinds
    }

    pub(crate) fn entries(&self) -> impl Iterator<Item = &Registered> {
        self.entries.values()
    }

    #[cfg(test)]
    pub(crate) fn get(&self, kind: &str) -> Option<&Registered> {
        self.entries.get(kind)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    static PING: JobKind = JobKind::new("test_ping").concurrency(3);
    static OTHER: JobKind = JobKind::new("test_other");

    struct Ping;

    impl JobHandler for Ping {
        const KIND: &'static str = "test_ping";

        async fn run(&self, _ctx: JobContext, _payload: Value) -> Result<(), JobError> {
            Ok(())
        }
    }

    #[test]
    fn a_kind_is_registered_once_and_only_with_its_own_handler() {
        let mut registry = JobRegistry::new();
        assert_eq!(
            registry.register(&OTHER, Ping),
            Err(RegistryError::KindMismatch {
                kind: "test_other",
                handler: "test_ping"
            })
        );
        registry.register(&PING, Ping).unwrap();
        assert_eq!(
            registry.register(&PING, Ping),
            Err(RegistryError::Duplicate("test_ping".to_owned()))
        );
        assert_eq!(registry.kinds(), vec![&PING]);
        assert_eq!(
            registry
                .get("test_ping")
                .unwrap()
                .permits
                .available_permits(),
            3
        );
        assert!(registry.get("unknown").is_none());
    }
}
