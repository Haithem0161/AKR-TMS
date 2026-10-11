//! The transactional outbox: facts that other parts of the system react to, written in the transaction
//! that made them true and delivered at least once.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use akr_common::correlation::{self, CorrelationId};
use akr_common::settings::SettingsService;
use async_trait::async_trait;
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::backoff::backoff;
use crate::error::{JobError, QueueError};
use crate::metrics::{self, OUTBOX_PENDING, OUTBOX_STUCK};
use crate::queue::{correlation_of, with_correlation};
use crate::store::{OutboxRow, OutboxStore};
use crate::worker::stored_error;

const DISPATCH_EVERY: Duration = Duration::from_secs(1);

/// How many rows one round delivers before it yields to the next tick.
const ROUND_LIMIT: usize = 200;

#[derive(Clone)]
pub struct Outbox {
    store: Arc<dyn OutboxStore>,
}

impl Outbox {
    pub fn new(store: Arc<dyn OutboxStore>) -> Self {
        Self { store }
    }

    /// Appends a message in the caller's transaction. The payload is a JSON object of ids and codes.
    pub async fn append(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        topic: &str,
        payload: Value,
    ) -> Result<Uuid, QueueError> {
        let payload = with_correlation(topic, payload)?;
        let id = Uuid::now_v7();
        self.store.append(tx, id, topic, &payload).await?;
        Ok(id)
    }
}

/// A consumer of one topic. `tx` is the transaction that holds the row: what the handler writes commits
/// with the row's dispatch, or not at all. A handler is idempotent.
#[async_trait]
pub trait TopicHandler: Send + Sync {
    async fn handle(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        row: &OutboxRow,
    ) -> Result<(), JobError>;
}

/// The handlers of every topic a binary consumes: a list per topic, run in the order registered.
#[derive(Default)]
pub struct TopicRegistry {
    handlers: BTreeMap<String, Vec<Arc<dyn TopicHandler>>>,
}

impl TopicRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends `handler` to the list of `topic`.
    pub fn register(&mut self, topic: &str, handler: Arc<dyn TopicHandler>) {
        self.handlers
            .entry(topic.to_owned())
            .or_default()
            .push(handler);
    }

    pub fn topics(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }
}

pub struct OutboxDispatcher {
    store: Arc<dyn OutboxStore>,
    settings: SettingsService,
    registry: TopicRegistry,
}

impl OutboxDispatcher {
    pub fn new(
        store: Arc<dyn OutboxStore>,
        settings: SettingsService,
        registry: TopicRegistry,
    ) -> Self {
        Self {
            store,
            settings,
            registry,
        }
    }

    pub async fn run(self, shutdown: CancellationToken) {
        let mut tick = tokio::time::interval(DISPATCH_EVERY);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            tokio::select! {
                () = shutdown.cancelled() => break,
                _ = tick.tick() => {
                    if let Err(error) = self.dispatch_once().await {
                        tracing::error!(%error, "the outbox round failed");
                    }
                }
            }
        }
    }

    /// One round: delivers due rows one at a time, each in its own transaction, until none is due. Returns
    /// how many were dispatched.
    ///
    /// A row leaves the queue only by success or at `outbox.max_attempts`; a row at the limit stays, counted
    /// by `akr_outbox_stuck`.
    pub async fn dispatch_once(&self) -> Result<usize, sqlx::Error> {
        let max_attempts =
            i32::try_from(self.settings.outbox_max_attempts().await).unwrap_or(i32::MAX);
        let topics = self.registry.topics();
        let mut dispatched = 0;
        if !topics.is_empty() {
            for _ in 0..ROUND_LIMIT {
                let mut tx = self.store.begin().await?;
                let Some(row) = self.store.claim(&mut tx, &topics, max_attempts).await? else {
                    break;
                };
                match self.deliver(&mut tx, &row).await {
                    Ok(()) => {
                        self.store.mark_dispatched(&mut tx, row.id).await?;
                        tx.commit().await?;
                        dispatched += 1;
                    }
                    Err(error) => {
                        tx.rollback().await?;
                        let text = stored_error(error.message());
                        tracing::warn!(topic = %row.topic, outbox_id = %row.id, attempts = row.attempts + 1, error = %text, "an outbox row was not delivered");
                        self.store
                            .defer(row.id, &text, backoff(row.attempts + 1))
                            .await?;
                    }
                }
            }
        }
        let counts = self.store.counts(max_attempts).await?;
        ::metrics::gauge!(OUTBOX_PENDING).set(counts.pending as f64);
        ::metrics::gauge!(OUTBOX_STUCK).set(counts.stuck as f64);
        metrics::heartbeat("outbox_dispatch");
        Ok(dispatched)
    }

    async fn deliver(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        row: &OutboxRow,
    ) -> Result<(), JobError> {
        let Some(handlers) = self.registry.handlers.get(&row.topic) else {
            return Ok(());
        };
        correlation::scope(CorrelationId::from(correlation_of(&row.payload)), async {
            for handler in handlers {
                handler.handle(tx, row).await?;
            }
            Ok(())
        })
        .await
    }
}
