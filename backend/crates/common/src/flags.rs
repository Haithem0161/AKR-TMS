//! `FlagsService`: feature flags, read through a short cache.
//!
//! A flag gates a feature, never a security boundary: the environment and Cedar do that. A flag that cannot
//! be read is off.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use akr_domain::audit::{Actor, AuditEvent, AuditSink, Canon, RequestCtx, kinds};
use akr_domain::flags::{FeatureFlag, FlagContext, FlagScope, UpdateFlag};
use akr_domain::repositories::FlagsRepository;
use serde_json::json;
use sqlx::{Postgres, Transaction};
use tokio::sync::RwLock;
use tokio::time::Instant;

use crate::errors::AppError;

const FRESH_FOR: Duration = Duration::from_secs(30);
const RETRY_AFTER: Duration = Duration::from_secs(5);

#[derive(Default)]
struct Cache {
    flags: HashMap<String, (bool, FlagScope)>,
    good_until: Option<Instant>,
}

impl Cache {
    fn is_fresh(&self) -> bool {
        self.good_until.is_some_and(|until| Instant::now() < until)
    }

    fn answer(&self, key: &str, ctx: &FlagContext) -> bool {
        self.flags
            .get(key)
            .is_some_and(|(enabled, scope)| *enabled && scope.covers(ctx))
    }
}

struct Inner {
    repo: Arc<dyn FlagsRepository>,
    audit: Arc<dyn AuditSink>,
    cache: RwLock<Cache>,
}

#[derive(Clone)]
pub struct FlagsService {
    inner: Arc<Inner>,
}

impl FlagsService {
    pub fn new(repo: Arc<dyn FlagsRepository>, audit: Arc<dyn AuditSink>) -> Self {
        Self {
            inner: Arc::new(Inner {
                repo,
                audit,
                cache: RwLock::new(Cache::default()),
            }),
        }
    }

    /// True when the flag is on and its scope is empty or names the caller's branch or one of the caller's
    /// roles. A key with no row is off.
    pub async fn is_enabled(&self, key: &str, ctx: &FlagContext) -> bool {
        {
            let cache = self.inner.cache.read().await;
            if cache.is_fresh() {
                return cache.answer(key, ctx);
            }
        }
        let mut cache = self.inner.cache.write().await;
        if !cache.is_fresh() {
            match self.inner.repo.list().await {
                Ok(rows) => {
                    cache.flags = rows
                        .into_iter()
                        .map(|f| (f.key, (f.enabled, f.scope.0)))
                        .collect();
                    cache.good_until = Some(Instant::now() + FRESH_FOR);
                }
                Err(error) => {
                    tracing::warn!(%error, "feature flags cannot be read; serving the last values");
                    cache.good_until = Some(Instant::now() + RETRY_AFTER);
                }
            }
        }
        cache.answer(key, ctx)
    }

    /// Drops the cached flags. The caller runs it after a change has committed.
    pub async fn invalidate(&self) {
        self.inner.cache.write().await.good_until = None;
    }

    /// Changes one flag in the caller's transaction and records `flag.updated`.
    pub async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        key: &str,
        input: UpdateFlag,
        expected_version: i32,
        actor: &Actor,
        request: &RequestCtx,
    ) -> Result<FeatureFlag, AppError> {
        let before = self
            .inner
            .repo
            .lock(tx, key)
            .await?
            .ok_or(AppError::NotFound)?;
        let after = self
            .inner
            .repo
            .update(tx, key, &input, expected_version, actor.id())
            .await?;

        let event = AuditEvent::new(&kinds::FLAG_UPDATED, *actor, request.clone())
            .resource_key(key)
            .detail("before", state("before", &before)?)?
            .detail("after", state("after", &after)?)?;
        self.inner.audit.emit(tx, event).await?;

        Ok(after)
    }
}

/// What a flag change records about one side of it: whether the flag is on, and for whom.
fn state(name: &str, flag: &FeatureFlag) -> Result<Canon, AppError> {
    let roles: Vec<&str> = flag.scope.roles.iter().map(|r| r.as_str()).collect();
    let value = json!({
        "enabled": flag.enabled,
        "scope": { "branches": flag.scope.branches, "roles": roles },
    });
    Ok(Canon::from_json(name, &value)?)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use akr_domain::DomainError;
    use akr_domain::audit::RecordingSink;
    use akr_domain::text::{I18nText, RoleCode};
    use async_trait::async_trait;
    use chrono::Utc;
    use sqlx::types::Json;
    use uuid::Uuid;

    use super::*;

    #[derive(Default)]
    struct FakeRepo {
        rows: Mutex<Vec<FeatureFlag>>,
        reads: AtomicUsize,
        failing: AtomicBool,
    }

    impl FakeRepo {
        fn set(&self, key: &str, enabled: bool, scope: FlagScope) {
            let mut rows = self.rows.lock().unwrap();
            rows.retain(|f| f.key != key);
            rows.push(flag(key, enabled, scope));
        }
    }

    fn flag(key: &str, enabled: bool, scope: FlagScope) -> FeatureFlag {
        FeatureFlag {
            key: key.to_owned(),
            enabled,
            scope: Json(scope),
            label: Json(I18nText {
                ar: "x".to_owned(),
                en: "x".to_owned(),
            }),
            updated_by: None,
            updated_at: Utc::now(),
            version: 1,
        }
    }

    #[async_trait]
    impl FlagsRepository for FakeRepo {
        async fn list(&self) -> Result<Vec<FeatureFlag>, DomainError> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            if self.failing.load(Ordering::SeqCst) {
                return Err(DomainError::Database(sqlx::Error::PoolTimedOut));
            }
            Ok(self.rows.lock().unwrap().clone())
        }

        async fn get(&self, key: &str) -> Result<Option<FeatureFlag>, DomainError> {
            Ok(self
                .rows
                .lock()
                .unwrap()
                .iter()
                .find(|f| f.key == key)
                .cloned())
        }

        async fn lock(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            _key: &str,
        ) -> Result<Option<FeatureFlag>, DomainError> {
            Err(DomainError::NotFound)
        }

        async fn update(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            _key: &str,
            _input: &UpdateFlag,
            _expected_version: i32,
            _actor: Option<Uuid>,
        ) -> Result<FeatureFlag, DomainError> {
            Err(DomainError::NotFound)
        }
    }

    fn service(repo: Arc<FakeRepo>) -> FlagsService {
        FlagsService::new(repo, Arc::new(RecordingSink::new()))
    }

    fn role(code: &str) -> RoleCode {
        RoleCode::try_from(code.to_owned()).unwrap()
    }

    #[tokio::test]
    async fn a_flag_is_on_only_when_enabled_and_in_scope() {
        let branch = Uuid::now_v7();
        let repo = Arc::new(FakeRepo::default());
        repo.set("everyone", true, FlagScope::default());
        repo.set("switched_off", false, FlagScope::default());
        repo.set(
            "pilot",
            true,
            FlagScope {
                branches: vec![branch],
                roles: vec![role("mlro")],
            },
        );
        let flags = service(repo);
        let anyone = FlagContext::default();
        let in_branch = FlagContext {
            branch_id: Some(branch),
            roles: vec![role("analyst")],
        };
        let with_role = FlagContext {
            branch_id: None,
            roles: vec![role("mlro")],
        };

        assert!(flags.is_enabled("everyone", &anyone).await);
        assert!(!flags.is_enabled("switched_off", &anyone).await);
        assert!(!flags.is_enabled("pilot", &anyone).await);
        assert!(flags.is_enabled("pilot", &in_branch).await);
        assert!(flags.is_enabled("pilot", &with_role).await);
        assert!(!flags.is_enabled("no_such_flag", &anyone).await);
    }

    #[tokio::test(start_paused = true)]
    async fn flags_are_read_once_per_cache_age_and_again_after_an_invalidation() {
        let repo = Arc::new(FakeRepo::default());
        repo.set("feature", false, FlagScope::default());
        let flags = service(repo.clone());
        let anyone = FlagContext::default();
        assert!(!flags.is_enabled("feature", &anyone).await);
        assert!(!flags.is_enabled("feature", &anyone).await);
        assert_eq!(repo.reads.load(Ordering::SeqCst), 1);

        repo.set("feature", true, FlagScope::default());
        assert!(
            !flags.is_enabled("feature", &anyone).await,
            "still the cached answer"
        );
        flags.invalidate().await;
        assert!(flags.is_enabled("feature", &anyone).await);

        repo.set("feature", false, FlagScope::default());
        tokio::time::advance(FRESH_FOR + Duration::from_secs(1)).await;
        assert!(!flags.is_enabled("feature", &anyone).await);
        assert_eq!(repo.reads.load(Ordering::SeqCst), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn a_flag_that_was_never_read_is_off_and_a_failed_read_keeps_the_last_answer() {
        let repo = Arc::new(FakeRepo::default());
        repo.set("feature", true, FlagScope::default());
        repo.failing.store(true, Ordering::SeqCst);
        let flags = service(repo.clone());
        let anyone = FlagContext::default();
        assert!(!flags.is_enabled("feature", &anyone).await);

        repo.failing.store(false, Ordering::SeqCst);
        tokio::time::advance(RETRY_AFTER + Duration::from_secs(1)).await;
        assert!(flags.is_enabled("feature", &anyone).await);

        repo.failing.store(true, Ordering::SeqCst);
        tokio::time::advance(FRESH_FOR + Duration::from_secs(1)).await;
        assert!(flags.is_enabled("feature", &anyone).await);
    }

    #[test]
    fn a_change_records_whether_the_flag_is_on_and_for_whom() {
        let branch = Uuid::now_v7();
        let scoped = flag(
            "pilot",
            true,
            FlagScope {
                branches: vec![branch],
                roles: vec![role("mlro")],
            },
        );
        assert_eq!(
            state("after", &scoped).unwrap().to_json(),
            json!({"enabled": true, "scope": {"branches": [branch.to_string()], "roles": ["mlro"]}})
        );
    }
}
