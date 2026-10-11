//! What every handler can reach.

use std::sync::Arc;

use akr_audit::AuditSink;
use akr_authz::Authorizer;
use akr_common::AppError;
use akr_common::cursor::CursorCodec;
use akr_common::flags::FlagsService;
use akr_common::idempotency::IdempotencyService;
use akr_common::settings::SettingsService;
use akr_crypto::Crypto;
use akr_db::{PgFlagsRepository, PgIdempotencyStore, PgJobStore, PgSettingsRepository};
use akr_jobs::JobQueue;
use akr_objectstore::ObjectStore;
use fred::clients::Pool as ValkeyPool;
use sqlx::PgPool;
use zeroize::Zeroizing;

use crate::config::ApiConfig;
use crate::middleware::principal::AuthSlot;
use crate::valkey::ValkeySettingsNotifier;

#[derive(Clone)]
pub struct AppState {
    /// The pool of the role `akr_app`.
    pub pool: PgPool,
    pub valkey: ValkeyPool,
    pub settings: SettingsService,
    pub flags: FlagsService,
    pub jobs: JobQueue,
    pub crypto: Arc<dyn Crypto>,
    pub store: Arc<dyn ObjectStore>,
    pub audit: Arc<dyn AuditSink>,
    pub authz: Arc<dyn Authorizer>,
    pub config: Arc<ApiConfig>,
    pub idempotency: IdempotencyService,
    pub cursors: Arc<CursorCodec>,
    /// Who the caller is. A stub until phase 01.
    pub auth: AuthSlot,
    #[cfg(feature = "fault-injection")]
    pub faults: Arc<Faults>,
}

/// What a binary, or a test, hands over to get an [`AppState`].
pub struct Dependencies {
    pub pool: PgPool,
    pub valkey: ValkeyPool,
    pub crypto: Arc<dyn Crypto>,
    pub store: Arc<dyn ObjectStore>,
    pub audit: Arc<dyn AuditSink>,
    pub authz: Arc<dyn Authorizer>,
    /// `keys/cursor-hmac`.
    pub cursor_key: Zeroizing<Vec<u8>>,
    pub config: ApiConfig,
}

impl AppState {
    pub fn new(deps: Dependencies) -> Result<Self, AppError> {
        let Dependencies {
            pool,
            valkey,
            crypto,
            store,
            audit,
            authz,
            cursor_key,
            config,
        } = deps;

        let settings = SettingsService::new(
            Arc::new(PgSettingsRepository::new(pool.clone())),
            audit.clone(),
            Arc::new(ValkeySettingsNotifier::new(valkey.clone())),
        );
        let flags = FlagsService::new(
            Arc::new(PgFlagsRepository::new(pool.clone())),
            audit.clone(),
        );
        let idempotency = IdempotencyService::new(
            Arc::new(PgIdempotencyStore::new(pool.clone())),
            crypto.clone(),
            settings.clone(),
        );
        let auth = AuthSlot::from_config(&config).map_err(AppError::internal)?;

        Ok(Self {
            jobs: JobQueue::new(Arc::new(PgJobStore::new(pool.clone()))),
            cursors: Arc::new(CursorCodec::new(cursor_key)?),
            pool,
            valkey,
            settings,
            flags,
            crypto,
            store,
            audit,
            authz,
            config: Arc::new(config),
            idempotency,
            auth,
            #[cfg(feature = "fault-injection")]
            faults: Arc::new(Faults::default()),
        })
    }
}

/// Failures a test can switch on.
#[cfg(feature = "fault-injection")]
#[derive(Debug, Default)]
pub struct Faults {
    die_before_commit: std::sync::atomic::AtomicBool,
}

#[cfg(feature = "fault-injection")]
impl Faults {
    /// The next mutation that reaches its commit loses its transaction instead, as if the process had died
    /// there.
    pub fn die_before_next_commit(&self) {
        self.die_before_commit
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub(crate) fn take_die_before_commit(&self) -> bool {
        self.die_before_commit
            .swap(false, std::sync::atomic::Ordering::SeqCst)
    }
}
