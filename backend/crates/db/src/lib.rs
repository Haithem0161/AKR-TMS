//! The PostgreSQL adapters of the ports declared in `akr-domain`, `akr-common`, `akr-crypto` and `akr-jobs`.
//!
//! This is the one crate that opens a connection or holds SQL text. Every statement on a table of the
//! migrations is checked against the schema at compile time; `.sqlx/` carries the checked form for builds
//! without a database. The two exceptions are [`provision`] (utility statements, which PostgreSQL does not
//! prepare) and [`check`] (reads of PostgreSQL's own catalogue).

pub mod check;
pub mod classification;
pub mod data_keys;
pub mod flags;
pub mod idempotency;
pub mod jobs;
pub mod outbox;
pub mod pool;
pub mod provision;
pub mod retention;
pub mod schema;
pub mod settings;
pub mod timers;

pub use classification::PgClassificationRepository;
pub use data_keys::PgDataKeyStore;
pub use flags::PgFlagsRepository;
pub use idempotency::PgIdempotencyStore;
pub use jobs::{JOBS_CHANNEL, PgJobStore, listen_for_jobs};
pub use outbox::PgOutboxStore;
pub use pool::{
    MIGRATOR, MigrationState, applied_versions, connect, database_check, migration_state,
    migrations_check, migrations_current, ping,
};
pub use retention::PgRetentionAssignmentRepository;
pub use schema::AKR_SCHEMAS;
pub use settings::PgSettingsRepository;
pub use timers::PgTimerStore;
