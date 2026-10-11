//! The idempotency ledger (DR-21, DR-412) and the request transaction it runs in.
//!
//! Every mutating request carries an `Idempotency-Key`. The claim on the key, the handler's writes and the
//! stored response share one transaction: they commit together or not at all, so a process that dies leaves
//! no claim behind and a retry runs the handler exactly once. A second request with the same key waits on
//! the uncommitted claim; it then replays the stored response, runs the handler itself (the first rolled
//! back), or gives up when the wait passes the in-flight lease.
//!
//! The hash covers the raw request bytes, never a re-serialised value: key order and number formatting are
//! not stable. Each part is length-prefixed, so `POST /a` with body `b` cannot collide with `POST /ab`.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex as StdMutex, PoisonError};
use std::time::Duration;

use akr_crypto::{Crypto, Family, Sealed};
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::request::Parts;
use sha2::{Digest, Sha256};
use sqlx::{Postgres, Transaction};
use subtle::ConstantTimeEq;
use tokio::sync::{Mutex, OwnedMappedMutexGuard, OwnedMutexGuard};
use uuid::Uuid;

use crate::errors::AppError;
use crate::settings::SettingsService;

pub const IDEMPOTENCY_KEY_HEADER: &str = "idempotency-key";

/// Set to `true` on a response served from the ledger.
pub const REPLAY_HEADER: &str = "x-idempotent-replay";

const KEY_MIN_LEN: usize = 8;
const KEY_MAX_LEN: usize = 255;

/// How many rows one statement of the retention sweep deletes.
const SWEEP_BATCH: i64 = 5_000;
/// The rest between two full batches of the sweep, so it never holds the table's attention for long.
const SWEEP_PAUSE: Duration = Duration::from_millis(200);

/// The caller the auth slot established for this request. The ledger is scoped to it: two callers may use
/// the same key without meeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrincipalId(pub Uuid);

/// 8 to 255 printable ASCII characters (`^[ -~]{8,255}$`).
pub fn is_valid_key(key: &str) -> bool {
    (KEY_MIN_LEN..=KEY_MAX_LEN).contains(&key.len())
        && key.bytes().all(|b| (b' '..=b'~').contains(&b))
}

/// SHA-256 over the method, the full original path and query, the principal and the raw body.
pub fn request_hash(method: &str, target: &str, principal_id: Uuid, body: &[u8]) -> [u8; 32] {
    let mut hash = Sha256::new();
    for part in [method.as_bytes(), target.as_bytes()] {
        hash.update((part.len() as u64).to_be_bytes());
        hash.update(part);
    }
    hash.update(principal_id.as_bytes());
    hash.update((body.len() as u64).to_be_bytes());
    hash.update(body);
    hash.finalize().into()
}

/// A response as the ledger keeps it: what the handler sent, byte for byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredResponse {
    pub status: u16,
    /// `None` for an empty body.
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

/// One row of `admin.idempotency_keys`, as the service reads it.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LedgerRow {
    pub request_hash: Vec<u8>,
    pub response_status: Option<i32>,
    pub response_content_type: Option<String>,
    pub response_body: Option<Vec<u8>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ClaimError {
    /// The wait on another request's uncommitted claim passed the lease. The transaction is aborted.
    #[error("another request holds the key")]
    InFlight,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// The ledger table. The PostgreSQL adapter is `akr_db::PgIdempotencyStore`.
#[async_trait]
pub trait IdempotencyStore: Send + Sync {
    /// Sets the transaction's `lock_timeout` to `lease`, then inserts the claim unless a row with the
    /// same principal and key exists. `Ok(true)`: the claim is this transaction's. `Ok(false)`: a committed
    /// row exists. The insert waits on an uncommitted row of another transaction; a wait longer than `lease`
    /// is [`ClaimError::InFlight`].
    async fn claim(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        principal_id: Uuid,
        key: &str,
        endpoint: &str,
        request_hash: &[u8; 32],
        lease: Duration,
    ) -> Result<bool, ClaimError>;

    async fn find(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        principal_id: Uuid,
        key: &str,
    ) -> Result<Option<LedgerRow>, sqlx::Error>;

    /// Writes the response on the claim row and sets `completed_at`.
    async fn complete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        principal_id: Uuid,
        key: &str,
        status: i32,
        content_type: Option<&str>,
        sealed_body: Option<&[u8]>,
    ) -> Result<(), sqlx::Error>;

    /// Deletes up to `batch` rows created more than `days` days ago; returns how many went.
    async fn delete_older_than(&self, days: i64, batch: i64) -> Result<u64, sqlx::Error>;
}

/// What a claim tells the middleware to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Claim {
    /// The key is this request's: run the handler, then [`IdempotencyService::complete`].
    Run,
    /// The key was used by the same request before: send this, and do nothing else.
    Replay(StoredResponse),
}

#[derive(Clone)]
pub struct IdempotencyService {
    store: Arc<dyn IdempotencyStore>,
    crypto: Arc<dyn Crypto>,
    settings: SettingsService,
}

impl IdempotencyService {
    pub fn new(
        store: Arc<dyn IdempotencyStore>,
        crypto: Arc<dyn Crypto>,
        settings: SettingsService,
    ) -> Self {
        Self {
            store,
            crypto,
            settings,
        }
    }

    /// How long a claim waits for a request that holds the same key (`idempotency.in_flight_lease_s`).
    ///
    /// The caller reads it before it opens the request transaction: a read of the settings may need a
    /// connection, and a request that already holds one must not wait for a second.
    pub async fn in_flight_lease(&self) -> Duration {
        self.settings.idempotency_in_flight_lease_s().await
    }

    /// Claims `key` for this request inside the request transaction, waiting up to `lease` for a request
    /// that holds it.
    ///
    /// On `Err` the caller rolls the transaction back: after [`AppError::IdempotencyInFlight`] the
    /// database has already aborted it.
    pub async fn claim(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        principal_id: Uuid,
        key: &str,
        endpoint: &str,
        request_hash: &[u8; 32],
        lease: Duration,
    ) -> Result<Claim, AppError> {
        match self
            .store
            .claim(tx, principal_id, key, endpoint, request_hash, lease)
            .await
        {
            Ok(true) => return Ok(Claim::Run),
            Ok(false) => {}
            Err(ClaimError::InFlight) => return Err(AppError::IdempotencyInFlight),
            Err(ClaimError::Database(error)) => return Err(error.into()),
        }

        // A committed row always carries its response. A row with none, or one the sweep removed
        // between the two statements, is answered like a request still running: the client retries.
        let Some(row) = self.store.find(tx, principal_id, key).await? else {
            return Err(AppError::IdempotencyInFlight);
        };
        if !bool::from(row.request_hash.ct_eq(request_hash.as_slice())) {
            return Err(AppError::IdempotencyReuse);
        }
        let Some(status) = row.response_status else {
            return Err(AppError::IdempotencyInFlight);
        };
        let status =
            u16::try_from(status).map_err(|_| AppError::internal("a stored status is not HTTP"))?;
        let body = match row.response_body {
            Some(bytes) => {
                let sealed = Sealed::from_bytes(&bytes)?;
                self.crypto
                    .open(Family::Idempotency, &aad(principal_id, key), &sealed)
                    .await?
            }
            None => Vec::new(),
        };
        Ok(Claim::Replay(StoredResponse {
            status,
            content_type: row.response_content_type,
            body,
        }))
    }

    /// Seals the handler's response onto the claim row. The caller commits next.
    pub async fn complete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        principal_id: Uuid,
        key: &str,
        response: &StoredResponse,
    ) -> Result<(), AppError> {
        let sealed = if response.body.is_empty() {
            None
        } else {
            Some(
                self.crypto
                    .seal(Family::Idempotency, &aad(principal_id, key), &response.body)
                    .await?
                    .to_bytes(),
            )
        };
        self.store
            .complete(
                tx,
                principal_id,
                key,
                i32::from(response.status),
                response.content_type.as_deref(),
                sealed.as_deref(),
            )
            .await?;
        Ok(())
    }

    /// Deletes the rows older than `idempotency.ttl_days`; returns how many went. Run by
    /// `maintenance_sweep`.
    pub async fn sweep(&self) -> Result<u64, AppError> {
        let days = self.settings.idempotency_ttl_days().await;
        let mut total = 0;
        loop {
            let deleted = self.store.delete_older_than(days, SWEEP_BATCH).await?;
            total += deleted;
            if deleted < SWEEP_BATCH as u64 {
                return Ok(total);
            }
            tokio::time::sleep(SWEEP_PAUSE).await;
        }
    }
}

/// Binds a sealed body to its row: a body copied to another principal or key does not open.
fn aad(principal_id: Uuid, key: &str) -> Vec<u8> {
    let mut aad = Vec::with_capacity(16 + key.len());
    aad.extend_from_slice(principal_id.as_bytes());
    aad.extend_from_slice(key.as_bytes());
    aad
}

type AfterCommit = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

type Slot = Option<Transaction<'static, Postgres>>;

struct RequestTxInner {
    tx: Arc<Mutex<Slot>>,
    after_commit: StdMutex<Vec<AfterCommit>>,
}

/// The transaction of one mutating request (DR-412). The idempotency middleware opens it and commits it
/// after a 2xx answer, or rolls it back after any other; a handler takes it as an extractor, does its
/// business writes on it and never commits it.
#[derive(Clone)]
pub struct RequestTx {
    inner: Arc<RequestTxInner>,
}

/// The request transaction, borrowed for a run of statements. It owns its borrow, so a handler may keep
/// it across an await.
pub type TxGuard = OwnedMappedMutexGuard<Slot, Transaction<'static, Postgres>>;

impl RequestTx {
    pub fn new(tx: Transaction<'static, Postgres>) -> Self {
        Self {
            inner: Arc::new(RequestTxInner {
                tx: Arc::new(Mutex::new(Some(tx))),
                after_commit: StdMutex::new(Vec::new()),
            }),
        }
    }

    /// Borrows the transaction. It never waits: one connection runs one statement at a time, so a second
    /// borrow while the first is alive is a fault of the caller, and so is a borrow after the request
    /// ended. Both are an error.
    pub fn lock(&self) -> Result<TxGuard, AppError> {
        let slot = self.inner.tx.clone().try_lock_owned().map_err(|_| {
            AppError::internal(
                "the request transaction was borrowed while another borrow was alive",
            )
        })?;
        OwnedMutexGuard::try_map(slot, Option::as_mut).map_err(|_| {
            AppError::internal("the request transaction was used after the request ended")
        })
    }

    /// Registers work that must follow the commit and never run without it (telling other processes that a
    /// setting changed). It runs in the order registered, and is dropped on a rollback.
    pub fn after_commit(&self, work: impl Future<Output = ()> + Send + 'static) {
        self.inner
            .after_commit
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(Box::pin(work));
    }

    /// Takes the transaction out, for the middleware to finish. `None` when a handler still holds it (a
    /// borrow that outlived the handler) or it was taken before.
    pub fn take(&self) -> Option<Transaction<'static, Postgres>> {
        self.inner
            .tx
            .try_lock()
            .ok()
            .and_then(|mut slot| slot.take())
    }

    /// Takes the work registered with [`RequestTx::after_commit`].
    pub fn take_after_commit(&self) -> Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>> {
        std::mem::take(
            &mut *self
                .inner
                .after_commit
                .lock()
                .unwrap_or_else(PoisonError::into_inner),
        )
    }
}

impl<S: Send + Sync> FromRequestParts<S> for RequestTx {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<RequestTx>().cloned().ok_or_else(|| {
            AppError::internal("a handler asked for the request transaction on a route without one")
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::collections::HashMap;

    use akr_crypto::InMemoryCrypto;
    use akr_domain::DomainError;
    use akr_domain::audit::RecordingSink;
    use akr_domain::repositories::SettingsRepository;
    use akr_domain::settings::Setting;
    use serde_json::Value;

    use super::*;
    use crate::settings::NoNotifier;

    const METHOD: &str = "POST";
    const TARGET: &str = "/api/v1/_echo?x=1";

    #[test]
    fn a_key_is_eight_to_255_printable_ascii_characters() {
        assert!(is_valid_key("abcdefgh"));
        assert!(is_valid_key("with space ~!"));
        assert!(is_valid_key(&"k".repeat(255)));
        assert!(!is_valid_key("short"));
        assert!(!is_valid_key(&"k".repeat(256)));
        assert!(!is_valid_key("tab\tinside1"));
        assert!(!is_valid_key("مفتاح-عربي-طويل"));
        assert!(!is_valid_key("line\nbreak1"));
    }

    #[test]
    fn the_hash_changes_with_every_part_of_the_request() {
        let principal = Uuid::now_v7();
        let base = request_hash(METHOD, TARGET, principal, b"{\"a\":1}");
        assert_eq!(base, request_hash(METHOD, TARGET, principal, b"{\"a\":1}"));
        assert_ne!(base, request_hash("PUT", TARGET, principal, b"{\"a\":1}"));
        assert_ne!(
            base,
            request_hash(METHOD, "/api/v1/_echo?x=2", principal, b"{\"a\":1}")
        );
        assert_ne!(
            base,
            request_hash(METHOD, TARGET, Uuid::now_v7(), b"{\"a\":1}")
        );
        assert_ne!(base, request_hash(METHOD, TARGET, principal, b"{\"a\": 1}"));
    }

    #[test]
    fn length_prefixes_keep_neighbouring_parts_apart() {
        let principal = Uuid::now_v7();
        assert_ne!(
            request_hash("POST", "/a", principal, b"b"),
            request_hash("POST", "/ab", principal, b"")
        );
        assert_ne!(
            request_hash("POS", "T/a", principal, b""),
            request_hash("POST", "/a", principal, b"")
        );
    }

    /// The ledger as a map. A claim, a replay and the request transaction need a database: those tests
    /// are in `akr-db`.
    #[derive(Default)]
    struct MemoryStore {
        rows: StdMutex<HashMap<(Uuid, String), LedgerRow>>,
    }

    #[async_trait]
    impl IdempotencyStore for MemoryStore {
        async fn claim(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            _principal_id: Uuid,
            _key: &str,
            _endpoint: &str,
            _request_hash: &[u8; 32],
            _lease: Duration,
        ) -> Result<bool, ClaimError> {
            unreachable!("needs a transaction")
        }

        async fn find(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            _principal_id: Uuid,
            _key: &str,
        ) -> Result<Option<LedgerRow>, sqlx::Error> {
            unreachable!("needs a transaction")
        }

        async fn complete(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            _principal_id: Uuid,
            _key: &str,
            _status: i32,
            _content_type: Option<&str>,
            _sealed_body: Option<&[u8]>,
        ) -> Result<(), sqlx::Error> {
            unreachable!("needs a transaction")
        }

        async fn delete_older_than(&self, _days: i64, batch: i64) -> Result<u64, sqlx::Error> {
            let mut rows = self.rows.lock().unwrap();
            let doomed: Vec<_> = rows.keys().take(batch as usize).cloned().collect();
            for key in &doomed {
                rows.remove(key);
            }
            Ok(doomed.len() as u64)
        }
    }

    struct NoSettings;

    #[async_trait]
    impl SettingsRepository for NoSettings {
        async fn get(&self, _key: &str) -> Result<Option<Setting>, DomainError> {
            Ok(None)
        }

        async fn list(&self) -> Result<Vec<Setting>, DomainError> {
            Ok(Vec::new())
        }

        async fn lock(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            _key: &str,
        ) -> Result<Option<Setting>, DomainError> {
            Ok(None)
        }

        async fn update(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            _key: &str,
            _value: &Value,
            _expected_version: i32,
            _actor: Option<Uuid>,
        ) -> Result<Setting, DomainError> {
            Err(DomainError::NotFound)
        }
    }

    fn service(store: Arc<MemoryStore>) -> IdempotencyService {
        let settings = SettingsService::new(
            Arc::new(NoSettings),
            Arc::new(RecordingSink::new()),
            Arc::new(NoNotifier),
        );
        IdempotencyService::new(
            store,
            Arc::new(InMemoryCrypto::from_seed([7; 32])),
            settings,
        )
    }

    #[tokio::test]
    async fn the_sweep_deletes_in_batches_until_none_is_left() {
        let store = Arc::new(MemoryStore::default());
        {
            let mut rows = store.rows.lock().unwrap();
            for n in 0..(SWEEP_BATCH as usize * 2 + 17) {
                rows.insert(
                    (Uuid::now_v7(), format!("key-{n:08}")),
                    LedgerRow {
                        request_hash: vec![0; 32],
                        response_status: Some(204),
                        response_content_type: None,
                        response_body: None,
                    },
                );
            }
        }
        let deleted = service(store.clone()).sweep().await.unwrap();
        assert_eq!(deleted, SWEEP_BATCH as u64 * 2 + 17);
        assert!(store.rows.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn a_sealed_body_opens_only_for_its_own_principal_and_key() {
        let crypto = InMemoryCrypto::from_seed([7; 32]);
        let principal = Uuid::now_v7();
        let sealed = crypto
            .seal(
                Family::Idempotency,
                &aad(principal, "key-00000001"),
                b"{\"id\":1}",
            )
            .await
            .unwrap();
        let stored = Sealed::from_bytes(&sealed.to_bytes()).unwrap();
        assert_eq!(
            crypto
                .open(
                    Family::Idempotency,
                    &aad(principal, "key-00000001"),
                    &stored
                )
                .await
                .unwrap(),
            b"{\"id\":1}"
        );
        assert!(
            crypto
                .open(
                    Family::Idempotency,
                    &aad(principal, "key-00000002"),
                    &stored
                )
                .await
                .is_err()
        );
        assert!(
            crypto
                .open(
                    Family::Idempotency,
                    &aad(Uuid::now_v7(), "key-00000001"),
                    &stored
                )
                .await
                .is_err()
        );
    }
}
