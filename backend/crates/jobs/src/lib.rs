//! Durable background work on PostgreSQL: jobs, the transactional outbox and timers.
//!
//! The golden rule (`.claude/rules/jobs.md`): work is enqueued only inside the transaction that caused it,
//! so it exists exactly when its cause committed. [`JobQueue::enqueue`], [`Outbox::append`] and
//! [`TimerService::schedule`] take the caller's transaction and nothing else.
//!
//! Delivery is at least once. A handler is idempotent, and a timer handler re-reads its owner and does
//! nothing when the owner's state no longer calls for it.
//!
//! This crate holds the ports ([`JobStore`], [`OutboxStore`], [`TimerStore`]) and the loops; the SQL is in
//! `akr-db`.

pub mod backoff;
pub mod clock;
pub mod error;
pub mod handler;
pub mod kind;
pub mod metrics;
pub mod outbox;
pub mod payload;
pub mod queue;
pub mod store;
pub mod timers;
pub mod worker;

pub use clock::{BusinessClock, CalendarDaysClock, ClockUnit};
pub use error::{JobError, QueueError, RegistryError};
pub use handler::{JobContext, JobHandler, JobRegistry};
pub use kind::{JobKind, TimerKind, kinds};
pub use outbox::{Outbox, OutboxDispatcher, TopicHandler, TopicRegistry};
pub use queue::{JobQueue, NewJob};
pub use store::{
    Job, JobCounts, JobStatus, JobStore, OutboxCounts, OutboxRow, OutboxStore, Reaped, Timer,
    TimerStore,
};
pub use timers::{NewTimer, TimerRegistry, TimerService, TimerSweeper};
pub use worker::Worker;
