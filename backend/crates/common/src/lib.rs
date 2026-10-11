//! What every AKR-TMS binary shares: configuration, the error catalogue, request plumbing (correlation,
//! idempotency, cursors, `If-Match`), the settings and flag services, and redaction.
//!
//! This crate opens no database connection and holds no SQL text: its services reach PostgreSQL through the
//! ports of `akr-domain` and the ports declared here, which `akr-db` implements.

pub mod config;
pub mod correlation;
pub mod crypto;
pub mod cursor;
pub mod data_as_of;
pub mod errors;
pub mod etag;
pub mod flags;
pub mod idempotency;
pub mod ids;
pub mod infra;
pub mod redaction;
pub mod settings;
pub mod settings_registry;
pub mod time;

pub use errors::{AppError, Problem, ValidationError};
