//! `SettingsService`: typed reads of `admin.settings` through a short cache, and the one door a value
//! changes through.
//!
//! A read never fails a request: a value that cannot be read, or that no longer fits its type, falls back to
//! the seeded default of the registry with a warning. A change is validated against the registry, checked
//! against the regulatory limit of its row, written with a version check and audited, all in the caller's
//! transaction.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, PoisonError, RwLock as StdRwLock};
use std::time::Duration;

use akr_domain::audit::{Actor, AuditEvent, AuditSink, Canon, RequestCtx, kinds};
use akr_domain::repositories::SettingsRepository;
use akr_domain::settings::schedule::{self, Schedule};
use akr_domain::settings::{ChangeSource, Setting, check_floor};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use tokio::sync::RwLock;
use tokio::time::Instant;

use crate::errors::AppError;
use crate::settings_registry::{self, SettingDef, keys};
use crate::time::{self, DEFAULT_BANK_ZONE};

/// The Valkey channel a change is announced on.
pub const SETTINGS_CHANGED_CHANNEL: &str = "settings.changed";

/// How long a read of the table is served from memory.
const FRESH_FOR: Duration = Duration::from_secs(30);

/// How long the last good values are served after a failed read, before the next try.
const RETRY_AFTER: Duration = Duration::from_secs(5);

/// Tells the other processes that a setting changed, so they drop their cache before it ages out. Best
/// effort: the age limit of the cache is the bound.
#[async_trait]
pub trait SettingsNotifier: Send + Sync {
    async fn settings_changed(&self, key: &str);
}

/// A notifier for a process with nobody to tell (the CLI, tests).
#[derive(Debug, Default, Clone, Copy)]
pub struct NoNotifier;

#[async_trait]
impl SettingsNotifier for NoNotifier {
    async fn settings_changed(&self, _key: &str) {}
}

/// Runs in the transaction of a change, after the write, for every source of change (7.25). A failure
/// rolls the change back.
#[async_trait]
pub trait SettingChangeHook: Send + Sync {
    async fn changed(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        before: &Setting,
        after: &Setting,
    ) -> Result<(), AppError>;
}

#[derive(Default)]
struct Cache {
    values: HashMap<String, Value>,
    good_until: Option<Instant>,
}

impl Cache {
    fn is_fresh(&self) -> bool {
        self.good_until.is_some_and(|until| Instant::now() < until)
    }
}

struct Inner {
    repo: Arc<dyn SettingsRepository>,
    audit: Arc<dyn AuditSink>,
    notifier: Arc<dyn SettingsNotifier>,
    hooks: StdRwLock<HashMap<&'static str, Vec<Arc<dyn SettingChangeHook>>>>,
    cache: RwLock<Cache>,
    /// The instance belongs to `akr-ctl`, which may write the governed keys the registry marks
    /// `cli_writable`.
    cli: bool,
}

#[derive(Clone)]
pub struct SettingsService {
    inner: Arc<Inner>,
}

impl SettingsService {
    pub fn new(
        repo: Arc<dyn SettingsRepository>,
        audit: Arc<dyn AuditSink>,
        notifier: Arc<dyn SettingsNotifier>,
    ) -> Self {
        Self::build(repo, audit, notifier, false)
    }

    /// The service of `akr-ctl`: a direct change is allowed on the governed keys the registry marks
    /// `cli_writable` (the drill dates), and on no other governed key.
    pub fn for_cli(repo: Arc<dyn SettingsRepository>, audit: Arc<dyn AuditSink>) -> Self {
        Self::build(repo, audit, Arc::new(NoNotifier), true)
    }

    fn build(
        repo: Arc<dyn SettingsRepository>,
        audit: Arc<dyn AuditSink>,
        notifier: Arc<dyn SettingsNotifier>,
        cli: bool,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                repo,
                audit,
                notifier,
                hooks: StdRwLock::new(HashMap::new()),
                cache: RwLock::new(Cache::default()),
                cli,
            }),
        }
    }

    /// Adds a hook to the list of `key`. Called while a binary wires itself, before it serves.
    pub fn register_hook(&self, key: &'static str, hook: Arc<dyn SettingChangeHook>) {
        self.inner
            .hooks
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .entry(key)
            .or_default()
            .push(hook);
    }

    fn hooks_for(&self, key: &str) -> Vec<Arc<dyn SettingChangeHook>> {
        self.inner
            .hooks
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .get(key)
            .cloned()
            .unwrap_or_default()
    }

    /// Drops the cached values, so the next read goes to the table.
    pub async fn invalidate(&self) {
        self.inner.cache.write().await.good_until = None;
    }

    /// Drops the cache and tells the other processes. The caller runs it after the change has committed.
    pub async fn notify_changed(&self, key: &str) {
        self.invalidate().await;
        self.inner.notifier.settings_changed(key).await;
    }

    /// The value of `key` as the table holds it, or `None` when no row can be read.
    pub async fn get_raw(&self, key: &str) -> Option<Value> {
        {
            let cache = self.inner.cache.read().await;
            if cache.is_fresh() {
                return cache.values.get(key).cloned();
            }
        }
        let mut cache = self.inner.cache.write().await;
        if !cache.is_fresh() {
            match self.inner.repo.list().await {
                Ok(rows) => {
                    cache.values = rows.into_iter().map(|s| (s.key, s.value)).collect();
                    cache.good_until = Some(Instant::now() + FRESH_FOR);
                }
                Err(error) => {
                    tracing::warn!(%error, "settings cannot be read; serving the last values");
                    cache.good_until = Some(Instant::now() + RETRY_AFTER);
                }
            }
        }
        cache.values.get(key).cloned()
    }

    /// Reads `key` as a `T`. A missing row, or a value `read` refuses, gives the seeded default.
    async fn typed<T>(
        &self,
        key: &'static str,
        read: impl Fn(&Value) -> Option<T>,
        last_resort: T,
    ) -> T {
        if let Some(value) = self.get_raw(key).await {
            if let Some(typed) = read(&value) {
                return typed;
            }
            tracing::warn!(key, "the setting does not fit its type; using the default");
        } else {
            tracing::warn!(key, "the setting has no row; using the default");
        }
        settings_registry::lookup(key)
            .and_then(|def| read(&def.default_value()))
            .unwrap_or(last_resort)
    }

    pub async fn integer(&self, key: &'static str) -> i64 {
        self.typed(key, Value::as_i64, 0).await
    }

    pub async fn duration(&self, key: &'static str) -> Duration {
        self.typed(
            key,
            |v| v.as_u64().filter(|s| *s > 0).map(Duration::from_secs),
            Duration::from_secs(1),
        )
        .await
    }

    pub async fn string(&self, key: &'static str) -> String {
        self.typed(key, |v| v.as_str().map(str::to_owned), String::new())
            .await
    }

    /// A timestamp that may not have happened yet.
    pub async fn timestamp(&self, key: &'static str) -> Option<DateTime<Utc>> {
        self.typed(
            key,
            |v| match v {
                Value::Null => Some(None),
                Value::String(s) => DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|t| Some(t.with_timezone(&Utc))),
                _ => None,
            },
            None,
        )
        .await
    }

    /// The schedule under `key` (`schedule.<kind>`). `None` when the key holds no schedule that parses and
    /// its default does not either; the scheduler then leaves the kind alone.
    pub async fn schedule(&self, key: &str) -> Option<Schedule> {
        let read = |v: &Value| v.as_str().and_then(|expr| schedule::parse(expr).ok());
        if let Some(parsed) = self.get_raw(key).await.as_ref().and_then(read) {
            return Some(parsed);
        }
        tracing::warn!(
            key,
            "no schedule can be read under the key; using the default"
        );
        settings_registry::lookup(key).and_then(|def| read(&def.default_value()))
    }

    pub async fn locale_default(&self) -> String {
        self.string(keys::LOCALE_DEFAULT).await
    }

    pub async fn bank_timezone(&self) -> Tz {
        self.typed(
            keys::BANK_TIMEZONE,
            |v| v.as_str().and_then(time::zone),
            DEFAULT_BANK_ZONE,
        )
        .await
    }

    pub async fn idempotency_ttl_days(&self) -> i64 {
        self.integer(keys::IDEMPOTENCY_TTL_DAYS).await
    }

    pub async fn idempotency_in_flight_lease_s(&self) -> Duration {
        self.duration(keys::IDEMPOTENCY_IN_FLIGHT_LEASE_S).await
    }

    pub async fn jobs_lease_s(&self) -> Duration {
        self.duration(keys::JOBS_LEASE_S).await
    }

    pub async fn jobs_done_retention_days(&self) -> i64 {
        self.integer(keys::JOBS_DONE_RETENTION_DAYS).await
    }

    pub async fn outbox_retention_days(&self) -> i64 {
        self.integer(keys::OUTBOX_RETENTION_DAYS).await
    }

    pub async fn outbox_max_attempts(&self) -> i64 {
        self.integer(keys::OUTBOX_MAX_ATTEMPTS).await
    }

    pub async fn timers_sweep_interval_s(&self) -> Duration {
        self.duration(keys::TIMERS_SWEEP_INTERVAL_S).await
    }

    pub async fn timers_terminal_retention_days(&self) -> i64 {
        self.integer(keys::TIMERS_TERMINAL_RETENTION_DAYS).await
    }

    pub async fn schedule_maintenance_sweep(&self) -> Option<Schedule> {
        self.schedule(keys::SCHEDULE_MAINTENANCE_SWEEP).await
    }

    pub async fn schedule_backup_check(&self) -> Option<Schedule> {
        self.schedule(keys::SCHEDULE_BACKUP_CHECK).await
    }

    pub async fn ops_backup_marker_path(&self) -> String {
        self.string(keys::OPS_BACKUP_MARKER_PATH).await
    }

    pub async fn ops_backup_max_age_h(&self) -> i64 {
        self.integer(keys::OPS_BACKUP_MAX_AGE_H).await
    }

    pub async fn ops_last_restore_drill_at(&self) -> Option<DateTime<Utc>> {
        self.timestamp(keys::OPS_LAST_RESTORE_DRILL_AT).await
    }

    pub async fn ops_restore_drill_max_age_days(&self) -> i64 {
        self.integer(keys::OPS_RESTORE_DRILL_MAX_AGE_DAYS).await
    }

    /// Changes one setting in the caller's transaction and records `setting.updated`.
    ///
    /// The caller commits, then calls [`SettingsService::notify_changed`].
    #[allow(clippy::too_many_arguments)]
    pub async fn apply(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        key: &str,
        value: Value,
        expected_version: i32,
        actor: &Actor,
        request: &RequestCtx,
        source: ChangeSource,
    ) -> Result<Setting, AppError> {
        let def = settings_registry::lookup(key).ok_or(AppError::NotFound)?;
        let before = self
            .inner
            .repo
            .lock(tx, key)
            .await?
            .ok_or(AppError::NotFound)?;

        def.validate(&value)?;
        if !value.is_null() {
            check_floor(key, before.floor.as_ref().map(|f| &f.0), &value)?;
        }
        if !self.may_change(def, &before, source) {
            return Err(AppError::not_allowed("governed"));
        }

        let after = self
            .inner
            .repo
            .update(tx, key, &value, expected_version, actor.id())
            .await?;

        for hook in self.hooks_for(key) {
            hook.changed(tx, &before, &after).await?;
        }

        let event = AuditEvent::new(&kinds::SETTING_UPDATED, *actor, request.clone())
            .resource_key(key)
            .detail(
                "before",
                Canon::from_json_decimal_as_text("before", &before.value)?,
            )?
            .detail(
                "after",
                Canon::from_json_decimal_as_text("after", &after.value)?,
            )?
            .detail("source", source_detail(source))?;
        self.inner.audit.emit(tx, event).await?;

        Ok(after)
    }

    /// A governed key changes only through a change request. The one exception is `akr-ctl` on the keys
    /// the registry marks for it.
    fn may_change(&self, def: &SettingDef, row: &Setting, source: ChangeSource) -> bool {
        let governed = def.governed || row.governed;
        match source {
            ChangeSource::ChangeRequest(_) => true,
            ChangeSource::Direct => !governed || (self.inner.cli && def.cli_writable),
        }
    }
}

fn source_detail(source: ChangeSource) -> Canon {
    let mut detail = BTreeMap::new();
    match source {
        ChangeSource::Direct => {
            detail.insert("type".to_owned(), Canon::from("direct"));
        }
        ChangeSource::ChangeRequest(id) => {
            detail.insert("type".to_owned(), Canon::from("change_request"));
            detail.insert("id".to_owned(), Canon::from(id));
        }
    }
    Canon::Object(detail)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use akr_domain::DomainError;
    use akr_domain::audit::RecordingSink;
    use akr_domain::settings::SettingValueType;
    use akr_domain::text::I18nText;
    use serde_json::json;
    use sqlx::types::Json;
    use uuid::Uuid;

    use super::*;

    #[derive(Default)]
    struct FakeRepo {
        rows: Mutex<Vec<Setting>>,
        reads: AtomicUsize,
        failing: AtomicBool,
    }

    impl FakeRepo {
        fn seeded() -> Arc<Self> {
            let repo = Self::default();
            for def in settings_registry::REGISTRY {
                repo.set(def.key, def.default_value());
            }
            Arc::new(repo)
        }

        fn set(&self, key: &str, value: Value) {
            let mut rows = self.rows.lock().unwrap();
            rows.retain(|s| s.key != key);
            rows.push(row(key, value));
        }

        fn reads(&self) -> usize {
            self.reads.load(Ordering::SeqCst)
        }
    }

    fn row(key: &str, value: Value) -> Setting {
        let def = settings_registry::lookup(key);
        Setting {
            key: key.to_owned(),
            value,
            value_type: def.map_or(SettingValueType::Json, |d| d.value_type),
            floor: None,
            governed: def.is_some_and(|d| d.governed),
            label: Json(serde_json::from_value::<I18nText>(json!({"ar": "x", "en": "x"})).unwrap()),
            updated_by: None,
            updated_at: Utc::now(),
            version: 1,
        }
    }

    #[async_trait]
    impl SettingsRepository for FakeRepo {
        async fn get(&self, key: &str) -> Result<Option<Setting>, DomainError> {
            Ok(self
                .rows
                .lock()
                .unwrap()
                .iter()
                .find(|s| s.key == key)
                .cloned())
        }

        async fn list(&self) -> Result<Vec<Setting>, DomainError> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            if self.failing.load(Ordering::SeqCst) {
                return Err(DomainError::Database(sqlx::Error::PoolTimedOut));
            }
            Ok(self.rows.lock().unwrap().clone())
        }

        async fn lock(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            _key: &str,
        ) -> Result<Option<Setting>, DomainError> {
            Err(DomainError::NotFound)
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

    fn service(repo: Arc<FakeRepo>) -> SettingsService {
        SettingsService::new(repo, Arc::new(RecordingSink::new()), Arc::new(NoNotifier))
    }

    #[tokio::test]
    async fn every_getter_reads_the_seeded_value() {
        let settings = service(FakeRepo::seeded());
        assert_eq!(settings.locale_default().await, "ar");
        assert_eq!(settings.bank_timezone().await, chrono_tz::Asia::Baghdad);
        assert_eq!(settings.idempotency_ttl_days().await, 7);
        assert_eq!(
            settings.idempotency_in_flight_lease_s().await,
            Duration::from_secs(120)
        );
        assert_eq!(settings.jobs_lease_s().await, Duration::from_secs(600));
        assert_eq!(settings.jobs_done_retention_days().await, 30);
        assert_eq!(settings.outbox_retention_days().await, 30);
        assert_eq!(settings.outbox_max_attempts().await, 10);
        assert_eq!(
            settings.timers_sweep_interval_s().await,
            Duration::from_secs(30)
        );
        assert_eq!(settings.timers_terminal_retention_days().await, 90);
        assert_eq!(
            settings.schedule_maintenance_sweep().await,
            schedule::parse("0 30 2 * * *").ok()
        );
        assert_eq!(
            settings.schedule_backup_check().await,
            schedule::parse("0 0 6 * * *").ok()
        );
        assert_eq!(
            settings.ops_backup_marker_path().await,
            "/var/lib/akr/backup/heartbeat"
        );
        assert_eq!(settings.ops_backup_max_age_h().await, 26);
        assert_eq!(settings.ops_last_restore_drill_at().await, None);
        assert_eq!(settings.ops_restore_drill_max_age_days().await, 31);
    }

    #[tokio::test]
    async fn a_changed_row_is_what_a_getter_returns() {
        let repo = FakeRepo::seeded();
        repo.set(keys::IDEMPOTENCY_TTL_DAYS, json!(14));
        repo.set(keys::BANK_TIMEZONE, json!("Asia/Riyadh"));
        repo.set(
            keys::OPS_LAST_RESTORE_DRILL_AT,
            json!("2026-10-11T08:00:00+03:00"),
        );
        let settings = service(repo);
        assert_eq!(settings.idempotency_ttl_days().await, 14);
        assert_eq!(settings.bank_timezone().await, chrono_tz::Asia::Riyadh);
        assert_eq!(
            settings.ops_last_restore_drill_at().await,
            Some("2026-10-11T05:00:00Z".parse().unwrap())
        );
    }

    #[tokio::test]
    async fn a_missing_or_misshapen_row_falls_back_to_the_seeded_default() {
        let repo = Arc::new(FakeRepo::default());
        repo.set(keys::JOBS_LEASE_S, json!("ten minutes"));
        repo.set(keys::SCHEDULE_BACKUP_CHECK, json!("whenever"));
        repo.set(keys::BANK_TIMEZONE, json!("Baghdad"));
        let settings = service(repo);
        assert_eq!(settings.jobs_lease_s().await, Duration::from_secs(600));
        assert_eq!(
            settings.schedule_backup_check().await,
            schedule::parse("0 0 6 * * *").ok()
        );
        assert_eq!(settings.bank_timezone().await, chrono_tz::Asia::Baghdad);
        assert_eq!(settings.idempotency_ttl_days().await, 7);
        assert_eq!(settings.schedule("schedule.no_such_kind").await, None);
    }

    #[tokio::test(start_paused = true)]
    async fn the_table_is_read_once_per_cache_age_and_again_after_an_invalidation() {
        let repo = FakeRepo::seeded();
        let settings = service(repo.clone());
        settings.idempotency_ttl_days().await;
        settings.jobs_lease_s().await;
        assert_eq!(repo.reads(), 1);

        repo.set(keys::IDEMPOTENCY_TTL_DAYS, json!(9));
        assert_eq!(
            settings.idempotency_ttl_days().await,
            7,
            "still the cached value"
        );

        tokio::time::advance(FRESH_FOR + Duration::from_secs(1)).await;
        assert_eq!(settings.idempotency_ttl_days().await, 9);
        assert_eq!(repo.reads(), 2);

        repo.set(keys::IDEMPOTENCY_TTL_DAYS, json!(11));
        settings.notify_changed(keys::IDEMPOTENCY_TTL_DAYS).await;
        assert_eq!(settings.idempotency_ttl_days().await, 11);
        assert_eq!(repo.reads(), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn a_failed_read_keeps_the_last_values_and_is_tried_again_soon() {
        let repo = FakeRepo::seeded();
        repo.set(keys::IDEMPOTENCY_TTL_DAYS, json!(14));
        let settings = service(repo.clone());
        assert_eq!(settings.idempotency_ttl_days().await, 14);

        repo.failing.store(true, Ordering::SeqCst);
        tokio::time::advance(FRESH_FOR + Duration::from_secs(1)).await;
        assert_eq!(
            settings.idempotency_ttl_days().await,
            14,
            "the last good value"
        );
        assert_eq!(repo.reads(), 2);
        assert_eq!(settings.idempotency_ttl_days().await, 14);
        assert_eq!(repo.reads(), 2, "no read storm while the table is down");

        repo.failing.store(false, Ordering::SeqCst);
        repo.set(keys::IDEMPOTENCY_TTL_DAYS, json!(21));
        tokio::time::advance(RETRY_AFTER + Duration::from_secs(1)).await;
        assert_eq!(settings.idempotency_ttl_days().await, 21);
        assert_eq!(repo.reads(), 3);
    }

    #[tokio::test]
    async fn only_the_cli_may_change_a_governed_key_directly_and_only_the_marked_ones() {
        let repo = FakeRepo::seeded();
        let api = service(repo.clone());
        let cli = SettingsService::for_cli(repo, Arc::new(RecordingSink::new()));
        let request = ChangeSource::ChangeRequest(Uuid::now_v7());

        let open = settings_registry::lookup(keys::IDEMPOTENCY_TTL_DAYS).unwrap();
        let governed = settings_registry::lookup(keys::OPS_BACKUP_MAX_AGE_H).unwrap();
        let drill = settings_registry::lookup(keys::OPS_LAST_RESTORE_DRILL_AT).unwrap();
        let row_of = |def: &SettingDef| row(def.key, def.default_value());

        assert!(api.may_change(open, &row_of(open), ChangeSource::Direct));
        assert!(!api.may_change(governed, &row_of(governed), ChangeSource::Direct));
        assert!(api.may_change(governed, &row_of(governed), request));
        assert!(!api.may_change(drill, &row_of(drill), ChangeSource::Direct));

        assert!(cli.may_change(drill, &row_of(drill), ChangeSource::Direct));
        assert!(!cli.may_change(governed, &row_of(governed), ChangeSource::Direct));

        let mut marked_in_the_table = row_of(open);
        marked_in_the_table.governed = true;
        assert!(!api.may_change(open, &marked_in_the_table, ChangeSource::Direct));
    }

    #[test]
    fn the_source_of_a_change_is_recorded_by_type() {
        assert_eq!(
            source_detail(ChangeSource::Direct).to_json(),
            json!({"type": "direct"})
        );
        let id = Uuid::now_v7();
        assert_eq!(
            source_detail(ChangeSource::ChangeRequest(id)).to_json(),
            json!({"type": "change_request", "id": id.to_string()})
        );
    }
}
