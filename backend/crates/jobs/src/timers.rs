//! Timers: a job that must run at a future instant for one owner (a freeze that falls due, a task that times
//! out).
//!
//! **Rule for every timer handler:** it re-reads its owner and does nothing when the owner's state no longer
//! calls for it. Cancelling a timer is an optimisation; the re-check is the guarantee.

use std::collections::BTreeMap;
use std::sync::Arc;

use akr_common::settings::SettingsService;
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use sqlx::{Postgres, Transaction};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::error::{QueueError, RegistryError};
use crate::kind::TimerKind;
use crate::metrics::{self, TIMERS_DUE};
use crate::queue::{JobQueue, NewJob, with_correlation};
use crate::store::{TimerInsert, TimerStore};

/// How many due timers one statement of the sweep claims.
const SWEEP_BATCH: i64 = 200;

/// A timer to schedule.
#[derive(Debug, Clone)]
pub struct NewTimer {
    pub kind: &'static TimerKind,
    pub owner_kind: String,
    pub owner_id: Uuid,
    pub fire_at: DateTime<Utc>,
    /// The calendar the deadline was computed on (phase 09).
    pub calendar_id: Option<Uuid>,
    /// A JSON object of ids and codes, handed to the job the timer enqueues.
    pub payload: Value,
}

impl NewTimer {
    pub fn new(
        kind: &'static TimerKind,
        owner_kind: &str,
        owner_id: Uuid,
        fire_at: DateTime<Utc>,
    ) -> Self {
        Self {
            kind,
            owner_kind: owner_kind.to_owned(),
            owner_id,
            fire_at,
            calendar_id: None,
            payload: json!({}),
        }
    }

    pub fn calendar(mut self, calendar_id: Uuid) -> Self {
        self.calendar_id = Some(calendar_id);
        self
    }

    pub fn payload(mut self, payload: Value) -> Self {
        self.payload = payload;
        self
    }
}

#[derive(Clone)]
pub struct TimerService {
    store: Arc<dyn TimerStore>,
}

impl TimerService {
    pub fn new(store: Arc<dyn TimerStore>) -> Self {
        Self { store }
    }

    /// Schedules a timer in the caller's transaction.
    pub async fn schedule(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        timer: NewTimer,
    ) -> Result<Uuid, QueueError> {
        if !timer.kind.owner_kinds.contains(&timer.owner_kind.as_str()) {
            return Err(QueueError::OwnerKind {
                kind: timer.kind.name,
                owner_kind: timer.owner_kind,
            });
        }
        let payload = with_correlation(timer.kind.name, timer.payload)?;
        let id = Uuid::now_v7();
        self.store
            .schedule(
                tx,
                TimerInsert {
                    id,
                    owner_kind: &timer.owner_kind,
                    owner_id: timer.owner_id,
                    kind: timer.kind.name,
                    fire_at: timer.fire_at,
                    calendar_id: timer.calendar_id,
                    payload: &payload,
                },
            )
            .await?;
        Ok(id)
    }

    /// Cancels a pending timer. A timer that already fired or was cancelled is left alone and the call
    /// succeeds with 0 (DR-410).
    pub async fn cancel(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        self.store.cancel(tx, id).await
    }

    /// Cancels the pending timers of one kind of one owner; returns how many.
    pub async fn cancel_for_owner(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        owner_kind: &str,
        owner_id: Uuid,
        kind: &TimerKind,
    ) -> Result<u64, sqlx::Error> {
        self.store
            .cancel_for_owner(tx, owner_kind, owner_id, kind.name)
            .await
    }
}

/// The timer kinds a binary fires. A due timer of a kind that is not registered is left alone.
#[derive(Default)]
pub struct TimerRegistry {
    kinds: BTreeMap<&'static str, &'static TimerKind>,
}

impl TimerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, kind: &'static TimerKind) -> Result<(), RegistryError> {
        if kind.job.name.strip_prefix("timer_") != Some(kind.name) {
            return Err(RegistryError::TimerJobName {
                timer: kind.name,
                job: kind.job.name,
            });
        }
        if self.kinds.insert(kind.name, kind).is_some() {
            return Err(RegistryError::Duplicate(kind.name.to_owned()));
        }
        Ok(())
    }

    fn names(&self) -> Vec<String> {
        self.kinds.keys().map(|name| (*name).to_owned()).collect()
    }
}

pub struct TimerSweeper {
    store: Arc<dyn TimerStore>,
    queue: JobQueue,
    settings: SettingsService,
    registry: TimerRegistry,
}

impl TimerSweeper {
    pub fn new(
        store: Arc<dyn TimerStore>,
        queue: JobQueue,
        settings: SettingsService,
        registry: TimerRegistry,
    ) -> Self {
        Self {
            store,
            queue,
            settings,
            registry,
        }
    }

    pub async fn run(self, shutdown: CancellationToken) {
        loop {
            let pause = self.settings.timers_sweep_interval_s().await;
            tokio::select! {
                () = shutdown.cancelled() => break,
                () = tokio::time::sleep(pause) => {
                    if let Err(error) = self.sweep_once().await {
                        tracing::error!(%error, "the timer sweep failed");
                    }
                }
            }
        }
    }

    /// One sweep: marks the due timers fired and enqueues `timer_<kind>` for each, in one transaction per
    /// batch. Returns how many fired.
    pub async fn sweep_once(&self) -> Result<usize, QueueError> {
        let kinds = self.registry.names();
        let mut fired = 0;
        while !kinds.is_empty() {
            let mut tx = self.store.begin().await?;
            let due = self.store.claim_due(&mut tx, &kinds, SWEEP_BATCH).await?;
            for timer in &due {
                let Some(kind) = self.registry.kinds.get(timer.kind.as_str()) else {
                    continue;
                };
                let mut payload = match &timer.payload {
                    Value::Object(fields) => fields.clone(),
                    _ => serde_json::Map::new(),
                };
                payload.insert("timer_id".to_owned(), json!(timer.id));
                payload.insert("owner_kind".to_owned(), json!(timer.owner_kind));
                payload.insert("owner_id".to_owned(), json!(timer.owner_id));
                self.queue
                    .enqueue(
                        &mut tx,
                        NewJob::new(&kind.job).payload(Value::Object(payload)),
                    )
                    .await?;
            }
            tx.commit().await?;
            fired += due.len();
            if (due.len() as i64) < SWEEP_BATCH {
                break;
            }
        }
        ::metrics::gauge!(TIMERS_DUE).set(self.store.due_count().await? as f64);
        metrics::heartbeat("timer_sweep");
        Ok(fired)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::kind::JobKind;

    static GOOD: TimerKind = TimerKind {
        name: "test_due",
        owner_kinds: &["test_owner"],
        job: JobKind::new("timer_test_due"),
    };
    static MISNAMED: TimerKind = TimerKind {
        name: "test_other",
        owner_kinds: &["test_owner"],
        job: JobKind::new("test_other_job"),
    };

    #[test]
    fn a_timer_kind_names_its_job_after_itself_and_is_registered_once() {
        let mut registry = TimerRegistry::new();
        registry.register(&GOOD).unwrap();
        assert_eq!(
            registry.register(&GOOD),
            Err(RegistryError::Duplicate("test_due".to_owned()))
        );
        assert_eq!(
            registry.register(&MISNAMED),
            Err(RegistryError::TimerJobName {
                timer: "test_other",
                job: "test_other_job"
            })
        );
        assert_eq!(registry.names(), vec!["test_due".to_owned()]);
    }
}
