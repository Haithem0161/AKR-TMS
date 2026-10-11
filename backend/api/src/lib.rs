//! `akr-api`: the HTTP surface of AKR-TMS.
//!
//! A library and a binary, so the integration tests drive the same router the binary serves.

pub mod app;
pub mod config;
pub mod middleware;
pub mod readiness;
pub mod routes;
pub mod state;
pub mod valkey;

pub use app::{ApiDoc, build_app, build_app_with, openapi};
pub use config::ApiConfig;
pub use state::{AppState, Dependencies};
