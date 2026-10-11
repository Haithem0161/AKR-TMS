//! The worker harness: the job poll, the lease reaper and the scheduler of one process.
//!
//! One loop drives all three. A claimed job runs on its own task with a heartbeat that keeps its lease; the
//! loop never waits for a handler.

use std::sync::Arc;
use std::time::{Duration, Instant};

use akr_common::correlation::{self, CorrelationId};
use akr_common::redaction::redact_text;
use akr_common::settings::SettingsService;
use akr_domain::audit::{Actor, AuditEvent, AuditSink, RequestCtx, kinds as audit_kinds};
use chrono::SecondsFormat;
use serde_json::json;
use tokio::sync::Notify;
use tokio::time::MissedTickBehavior;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use uuid::Uuid;

use crate::backoff::backoff;
use crate::error::JobError;
use crate::handler::{ErasedHandler, JobContext, JobRegistry};
use crate::kind::JobKind;
use crate::metrics::{self, JOB_DURATION_SECONDS, JOBS_DEAD_TOTAL, JOBS_QUEUED, JOBS_RUNNING};
use crate::queue::correlation_of;
use crate::store::{Job, JobInsert, JobStore};

const POLL_EVERY: Duration = Duration::from_millis(500);
const REAP_EVERY: Duration = Duration::from_secs(60);
const SCHEDULE_EVERY: Duration = Duration::from_secs(30);

/// How long running jobs get to finish after the process is told to stop.
const SHUTDOWN_GRACE: Duration = Duration::from_secs(30);

const ERROR_MAX_CHARS: usize = 500;
const LEASE_EXPIRED: &str = "lease expired";

struct Inner {
    store: Arc<dyn JobStore>,
    settings: SettingsService,
    audit: Arc<dyn AuditSink>,
    registry: JobRegistry,
    worker_id: String,
}

pub struct Worker {
    inner: Arc<Inner>,
    wake: Arc<Notify>,
    tasks: TaskTracker,
}

impl Worker {
    pub fn new(
        store: Arc<dyn JobStore>,
        settings: SettingsService,
        audit: Arc<dyn AuditSink>,
        registry: JobRegistry,
        worker_id: impl Into<String>,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                store,
                settings,
                audit,
                registry,
                worker_id: worker_id.into(),
            }),
            wake: Arc::new(Notify::new()),
            tasks: TaskTracker::new(),
        }
    }

    /// Notified by the `akr_jobs` listener: the poll runs at once instead of at its next tick. A hint
    /// only; the tick is what guarantees a job is seen.
    pub fn wake_handle(&self) -> Arc<Notify> {
        self.wake.clone()
    }

    pub async fn run(self, shutdown: CancellationToken) {
        let mut poll = tokio::time::interval(POLL_EVERY);
        let mut reap = tokio::time::interval(REAP_EVERY);
        let mut schedule = tokio::time::interval(SCHEDULE_EVERY);
        for timer in [&mut poll, &mut reap, &mut schedule] {
            timer.set_missed_tick_behavior(MissedTickBehavior::Delay);
        }
        loop {
            tokio::select! {
                () = shutdown.cancelled() => break,
                _ = poll.tick() => { self.poll_once().await; }
                () = self.wake.notified() => { self.poll_once().await; }
                _ = reap.tick() => { self.reap_once().await; }
                _ = schedule.tick() => { self.schedule_once().await; }
            }
        }
        self.tasks.close();
        if tokio::time::timeout(SHUTDOWN_GRACE, self.tasks.wait())
            .await
            .is_err()
        {
            tracing::warn!(
                "jobs were still running at shutdown; the lease reaper will queue them again"
            );
        }
    }

    /// Waits until every job this worker started has settled.
    pub async fn drain(&self) {
        self.tasks.close();
        self.tasks.wait().await;
        self.tasks.reopen();
    }

    /// One round of the poll: claims due jobs of every registered kind, up to the free permits, and starts
    /// them. Returns how many it started.
    pub async fn poll_once(&self) -> usize {
        let mut started = 0;
        for entry in self.inner.registry.entries() {
            let free = entry.permits.available_permits();
            if free == 0 {
                continue;
            }
            let claimed = match self
                .inner
                .store
                .claim(
                    entry.kind.name,
                    i64::from(entry.kind.concurrency.max(1)),
                    free as i64,
                    &self.inner.worker_id,
                )
                .await
            {
                Ok(jobs) => jobs,
                Err(error) => {
                    tracing::error!(kind = entry.kind.name, %error, "jobs could not be claimed");
                    continue;
                }
            };
            for job in claimed {
                let Ok(permit) = entry.permits.clone().try_acquire_owned() else {
                    // More rows than permits cannot be claimed; a row left running is taken back by
                    // the lease reaper.
                    tracing::error!(kind = entry.kind.name, job_id = %job.id, "a claimed job has no permit");
                    continue;
                };
                let inner = self.inner.clone();
                let kind = entry.kind;
                let handler = entry.handler.clone();
                self.tasks.spawn(async move {
                    inner.run_job(kind, handler, job).await;
                    drop(permit);
                });
                started += 1;
            }
        }
        metrics::heartbeat("job_poll");
        started
    }

    /// One round of the lease reaper. Returns how many jobs it took back.
    pub async fn reap_once(&self) -> usize {
        let mut reaped = 0;
        for entry in self.inner.registry.entries() {
            match self.inner.reap_kind(entry.kind).await {
                Ok(n) => reaped += n,
                Err(error) => {
                    tracing::error!(kind = entry.kind.name, %error, "expired leases could not be reaped")
                }
            }
        }
        metrics::heartbeat("lease_reaper");
        reaped
    }

    /// One round of the scheduler: enqueues the next slot of every periodic kind that has none yet. Returns
    /// how many jobs it enqueued.
    pub async fn schedule_once(&self) -> usize {
        let mut enqueued = 0;
        for entry in self.inner.registry.entries() {
            let Some(setting) = entry.kind.schedule else {
                continue;
            };
            match self.inner.schedule_kind(entry.kind, setting).await {
                Ok(true) => enqueued += 1,
                Ok(false) => {}
                Err(error) => {
                    tracing::error!(kind = entry.kind.name, %error, "the next slot could not be enqueued")
                }
            }
        }
        match self.inner.store.counts().await {
            Ok(counts) => {
                ::metrics::gauge!(JOBS_QUEUED).set(counts.queued as f64);
                ::metrics::gauge!(JOBS_RUNNING).set(counts.running as f64);
            }
            Err(error) => tracing::warn!(%error, "job counts could not be read"),
        }
        metrics::heartbeat("scheduler");
        enqueued
    }
}

impl Inner {
    async fn lease_of(&self, kind: &JobKind) -> Duration {
        match kind.lease {
            Some(lease) => lease,
            None => self.settings.jobs_lease_s().await,
        }
    }

    async fn run_job(&self, kind: &'static JobKind, handler: Arc<dyn ErasedHandler>, job: Job) {
        let correlation_id = correlation_of(&job.payload);
        let ctx = JobContext::new(
            job.id,
            kind.name,
            job.attempts,
            correlation_id,
            self.store.clone(),
        );

        let beat = tokio::spawn({
            let store = self.store.clone();
            let worker_id = self.worker_id.clone();
            let every = (self.lease_of(kind).await / 3).max(Duration::from_secs(1));
            let id = job.id;
            async move {
                let mut timer = tokio::time::interval(every);
                timer.tick().await;
                loop {
                    timer.tick().await;
                    if let Err(error) = store.heartbeat(id, &worker_id).await {
                        tracing::warn!(job_id = %id, %error, "the lease of a running job was not refreshed");
                    }
                }
            }
        });

        let started = Instant::now();
        let run = tokio::spawn(correlation::scope(
            CorrelationId::from(correlation_id),
            handler.run(ctx.clone(), job.payload.clone()),
        ));
        let outcome = match run.await {
            Ok(outcome) => outcome,
            Err(_) => Err(JobError::Retryable("the handler panicked".to_owned())),
        };
        beat.abort();
        ::metrics::histogram!(JOB_DURATION_SECONDS, "kind" => kind.name)
            .record(started.elapsed().as_secs_f64());

        if let Err(error) = self.settle(kind, &job, &ctx, outcome).await {
            tracing::error!(kind = kind.name, job_id = %job.id, %error, "the outcome of a job was not recorded; the lease reaper will take it back");
        }
    }

    async fn settle(
        &self,
        kind: &'static JobKind,
        job: &Job,
        ctx: &JobContext,
        outcome: Result<(), JobError>,
    ) -> Result<(), JobError> {
        let error = match outcome {
            Ok(()) => {
                if !ctx.completed_in_handler() && !self.store.complete(job.id, job.attempts).await?
                {
                    tracing::warn!(kind = kind.name, job_id = %job.id, "a job finished after its claim was lost");
                }
                return Ok(());
            }
            Err(error) => error,
        };
        let text = stored_error(error.message());
        let out_of_attempts = job.attempts >= job.max_attempts;
        if matches!(error, JobError::Permanent(_)) || out_of_attempts {
            tracing::error!(kind = kind.name, job_id = %job.id, attempts = job.attempts, error = %text, "a job is dead");
            let mut tx = self.store.begin().await?;
            if self
                .store
                .bury(&mut tx, job.id, job.attempts, &text)
                .await?
            {
                self.emit_dead(&mut tx, kind.name, job.id, job.attempts, ctx.correlation_id)
                    .await?;
                tx.commit().await?;
                ::metrics::counter!(JOBS_DEAD_TOTAL, "kind" => kind.name).increment(1);
            }
        } else {
            tracing::warn!(kind = kind.name, job_id = %job.id, attempts = job.attempts, error = %text, "a job failed and will run again");
            self.store
                .fail(job.id, job.attempts, &text, backoff(job.attempts))
                .await?;
        }
        Ok(())
    }

    async fn emit_dead(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        kind: &'static str,
        job_id: Uuid,
        attempts: i32,
        correlation_id: Uuid,
    ) -> Result<(), JobError> {
        let event = AuditEvent::new(
            &audit_kinds::JOB_DEAD,
            Actor::System,
            RequestCtx::system(job_id, correlation_id, kind),
        )
        .resource_id(job_id)
        .detail("kind", kind)?
        .detail("attempts", attempts)?;
        self.audit.emit(tx, event).await?;
        Ok(())
    }

    async fn reap_kind(&self, kind: &'static JobKind) -> Result<usize, JobError> {
        let lease = self.lease_of(kind).await;
        let mut tx = self.store.begin().await?;
        let reaped = self.store.reap_expired(&mut tx, kind.name, lease).await?;
        let mut dead = 0;
        for job in reaped.iter().filter(|job| job.dead) {
            self.emit_dead(&mut tx, kind.name, job.id, job.attempts, Uuid::now_v7())
                .await?;
            dead += 1;
        }
        tx.commit().await?;
        if !reaped.is_empty() {
            tracing::warn!(
                kind = kind.name,
                count = reaped.len(),
                dead,
                "{LEASE_EXPIRED}: jobs taken back"
            );
        }
        if dead > 0 {
            ::metrics::counter!(JOBS_DEAD_TOTAL, "kind" => kind.name).increment(dead);
        }
        Ok(reaped.len())
    }

    /// Enqueues the next slot of a periodic kind, due at the slot, unless a job for that slot exists. The
    /// slot is computed from the database clock under a lock per kind, so every worker of the installation
    /// names the same slot and exactly one of them inserts it.
    async fn schedule_kind(&self, kind: &'static JobKind, setting: &str) -> Result<bool, JobError> {
        let Some(schedule) = self.settings.schedule(setting).await else {
            return Ok(false);
        };
        let zone = self.settings.bank_timezone().await;
        let mut tx = self.store.begin().await?;
        let now = self.store.lock_schedule(&mut tx, kind.name).await?;
        let slot = schedule
            .next_after(now, zone)
            .map_err(JobError::permanent)?;
        let slot_text = slot.to_rfc3339_opts(SecondsFormat::Secs, true);
        let key = format!("{}:{slot_text}", kind.name);
        let payload = json!({ "slot": slot_text });
        let inserted = self
            .store
            .enqueue_slot(
                &mut tx,
                JobInsert {
                    id: Uuid::now_v7(),
                    kind: kind.name,
                    payload: &payload,
                    run_at: Some(slot),
                    max_attempts: kind.max_attempts,
                    coalesce_key: Some(&key),
                },
            )
            .await?;
        tx.commit().await?;
        Ok(inserted.is_some())
    }
}

/// The text of an error as a row keeps it: redacted, and cut to a length a screen can show.
pub(crate) fn stored_error(message: &str) -> String {
    redact_text(message).chars().take(ERROR_MAX_CHARS).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stored_error_is_redacted_and_cut() {
        let text = stored_error("transfer to IQ98 NBIQ 8501 2345 6789 012 refused");
        assert!(!text.contains("8501"), "{text}");
        let long = stored_error(&"x".repeat(2_000));
        assert_eq!(long.chars().count(), ERROR_MAX_CHARS);
    }
}
