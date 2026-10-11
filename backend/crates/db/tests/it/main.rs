//! Tests of the PostgreSQL adapters and of the services that sit on them. Every test gets its own database
//! with the migrations applied (`DATABASE_URL` names a role that may create databases).

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod data_keys;
mod flags;
mod idempotency;
mod jobs;
mod outbox;
mod provision;
mod schema;
mod settings;
mod support;
mod timers;
