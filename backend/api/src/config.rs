//! What the API knows about itself after start-up: the shared core, plus its own limits.

use akr_common::config::{CoreConfig, Env};

/// The largest request body the API reads, and the largest answer the idempotency ledger keeps. An upload
/// route of a later phase sets its own limit from a setting.
pub const MAX_BODY_BYTES: usize = 2 * 1024 * 1024;

/// Connections of the `akr_app` pool.
pub const DB_POOL_SIZE: u32 = 20;

/// Connections of the Valkey pool.
pub const VALKEY_POOL_SIZE: usize = 4;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub core: CoreConfig,
    pub max_body_bytes: usize,
}

impl ApiConfig {
    pub fn new(core: CoreConfig) -> Self {
        Self {
            core,
            max_body_bytes: MAX_BODY_BYTES,
        }
    }

    pub fn env(&self) -> Env {
        self.core.env
    }

    /// The development routes (`/api/v1/_echo`, Swagger UI) exist everywhere but in production.
    pub fn dev_routes(&self) -> bool {
        !self.core.env.is_production()
    }
}
