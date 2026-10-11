//! The audit port: the vocabulary of auditable facts and the sink they are written to.
//!
//! The port is here, not in `akr-audit`, because services below that crate emit events (`SettingsService`,
//! the job harness) and the dependency points the other way (DBS decision 38). `akr-audit` re-exports this
//! module and holds the sinks that do I/O.
//!
//! Every mutation names its audit event. An event that cannot be recorded fails the transaction of the change
//! it records.

use std::collections::BTreeMap;
use std::net::IpAddr;
use std::sync::{Mutex, PoisonError};

use async_trait::async_trait;
use chrono::{DateTime, SecondsFormat, Utc};
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

/// The system principal: the actor of jobs and sweeps (`admin.users`, phase 01).
pub const SYSTEM_PRINCIPAL_ID: Uuid = Uuid::from_u128(1);

const UA_MAX_CHARS: usize = 256;

#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    /// A number that is not an integer has no canonical form, so the chain hash could not be recomputed.
    #[error("audit detail `{key}` holds a number that is not an integer")]
    FloatInPayload { key: String },
    #[error("audit details must be a JSON object")]
    DetailsNotAnObject,
    #[error("`{key}` is not a detail of {kind}")]
    DetailKeyNotInCatalogue { kind: &'static str, key: String },
    #[error("audit canonicalization failed: {0}")]
    Canonicalization(String),
    #[error("the audit sink needs a transaction")]
    NotInTransaction,
    #[error("no partition of {table} for {month}")]
    PartitionMissing { table: &'static str, month: String },
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

/// A value of an audit payload. It has no float: amounts and decimals travel as decimal strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Canon {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Array(Vec<Canon>),
    Object(BTreeMap<String, Canon>),
}

impl Canon {
    /// Converts a JSON value. `key` names the detail in the error.
    pub fn from_json(key: &str, value: &Value) -> Result<Canon, AuditError> {
        Self::convert(key, value, false)
    }

    /// Converts a JSON value that may hold decimals (a setting value): a number that is not an integer is
    /// kept as its decimal text.
    pub fn from_json_decimal_as_text(key: &str, value: &Value) -> Result<Canon, AuditError> {
        Self::convert(key, value, true)
    }

    fn convert(key: &str, value: &Value, decimal_as_text: bool) -> Result<Canon, AuditError> {
        Ok(match value {
            Value::Null => Canon::Null,
            Value::Bool(b) => Canon::Bool(*b),
            Value::Number(n) => match n.as_i64() {
                Some(i) => Canon::Int(i),
                None if decimal_as_text => Canon::Str(n.to_string()),
                None => {
                    return Err(AuditError::FloatInPayload {
                        key: key.to_owned(),
                    });
                }
            },
            Value::String(s) => Canon::text(s)?,
            Value::Array(items) => Canon::Array(
                items
                    .iter()
                    .map(|item| Self::convert(key, item, decimal_as_text))
                    .collect::<Result<_, _>>()?,
            ),
            Value::Object(map) => Canon::Object(
                map.iter()
                    .map(|(k, v)| Ok((k.clone(), Self::convert(key, v, decimal_as_text)?)))
                    .collect::<Result<_, AuditError>>()?,
            ),
        })
    }

    fn text(s: &str) -> Result<Canon, AuditError> {
        if s.contains('\0') {
            return Err(AuditError::Canonicalization(
                "a string holds U+0000".to_owned(),
            ));
        }
        Ok(Canon::Str(s.to_owned()))
    }

    /// The value as plain JSON, for a log line or a test assertion. The chain hash never uses it.
    pub fn to_json(&self) -> Value {
        match self {
            Canon::Null => Value::Null,
            Canon::Bool(b) => Value::Bool(*b),
            Canon::Int(i) => Value::from(*i),
            Canon::Str(s) => Value::String(s.clone()),
            Canon::Array(items) => Value::Array(items.iter().map(Canon::to_json).collect()),
            Canon::Object(map) => {
                Value::Object(map.iter().map(|(k, v)| (k.clone(), v.to_json())).collect())
            }
        }
    }
}

impl From<bool> for Canon {
    fn from(value: bool) -> Self {
        Canon::Bool(value)
    }
}

impl From<i64> for Canon {
    fn from(value: i64) -> Self {
        Canon::Int(value)
    }
}

impl From<i32> for Canon {
    fn from(value: i32) -> Self {
        Canon::Int(i64::from(value))
    }
}

impl From<u32> for Canon {
    fn from(value: u32) -> Self {
        Canon::Int(i64::from(value))
    }
}

impl From<&str> for Canon {
    fn from(value: &str) -> Self {
        Canon::Str(value.replace('\0', ""))
    }
}

impl From<String> for Canon {
    fn from(value: String) -> Self {
        Canon::from(value.as_str())
    }
}

impl From<Uuid> for Canon {
    fn from(value: Uuid) -> Self {
        Canon::Str(value.as_hyphenated().to_string())
    }
}

impl From<DateTime<Utc>> for Canon {
    fn from(value: DateTime<Utc>) -> Self {
        Canon::Str(value.to_rfc3339_opts(SecondsFormat::Micros, true))
    }
}

impl<T: Into<Canon>> From<Option<T>> for Canon {
    fn from(value: Option<T>) -> Self {
        value.map_or(Canon::Null, Into::into)
    }
}

/// The hash-chained stream an event kind is written to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stream {
    AuditLog,
    AccessEvent,
    CaseEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Info,
    Notice,
    Warning,
    Critical,
}

/// One entry of the event catalogue.
#[derive(Debug, PartialEq, Eq)]
pub struct EventKind {
    pub name: &'static str,
    pub resource_type: &'static str,
    pub stream: Stream,
    pub severity: Severity,
    /// A security event the SIEM export carries (phase 25).
    pub siem: bool,
    /// The only detail names an event of this kind may hold.
    pub detail_keys: &'static [&'static str],
}

/// The event kinds of phase 00. Every later phase adds its own to this module and to `akr_audit::CATALOGUE`.
pub mod kinds {
    use super::{EventKind, Severity, Stream};

    pub static SETTING_UPDATED: EventKind = EventKind {
        name: "setting.updated",
        resource_type: "setting",
        stream: Stream::AuditLog,
        severity: Severity::Notice,
        siem: true,
        detail_keys: &["before", "after", "source"],
    };

    pub static FLAG_UPDATED: EventKind = EventKind {
        name: "flag.updated",
        resource_type: "flag",
        stream: Stream::AuditLog,
        severity: Severity::Notice,
        siem: true,
        detail_keys: &["before", "after"],
    };

    pub static RESTORE_DRILL_RECORDED: EventKind = EventKind {
        name: "restore_drill.recorded",
        resource_type: "setting",
        stream: Stream::AuditLog,
        severity: Severity::Info,
        siem: true,
        detail_keys: &["result", "evidence_sha256", "os_user", "os_session"],
    };

    /// An `akr-ctl` command that changes an installation, with the operating-system user who ran it.
    pub static CTL_COMMAND_RUN: EventKind = EventKind {
        name: "ctl.command_run",
        resource_type: "installation",
        stream: Stream::AuditLog,
        severity: Severity::Notice,
        siem: true,
        detail_keys: &["command", "os_user", "os_session", "outcome"],
    };

    pub static JOB_DEAD: EventKind = EventKind {
        name: "job.dead",
        resource_type: "job",
        stream: Stream::AuditLog,
        severity: Severity::Warning,
        siem: false,
        detail_keys: &["kind", "attempts"],
    };

    pub static PHASE_00: &[&EventKind] = &[
        &SETTING_UPDATED,
        &FLAG_UPDATED,
        &RESTORE_DRILL_RECORDED,
        &CTL_COMMAND_RUN,
        &JOB_DEAD,
    ];
}

/// Who performed an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actor {
    User {
        id: Uuid,
        on_behalf_of: Option<Uuid>,
    },
    /// The platform acting for itself: job handlers, sweeps, the operator CLI.
    System,
    /// Somebody with no session (a refused sign-in).
    Anonymous,
}

impl Actor {
    pub fn user(id: Uuid) -> Self {
        Actor::User {
            id,
            on_behalf_of: None,
        }
    }

    /// The principal recorded as the actor: the user, the system principal, or nobody.
    pub fn id(&self) -> Option<Uuid> {
        match self {
            Actor::User { id, .. } => Some(*id),
            Actor::System => Some(SYSTEM_PRINCIPAL_ID),
            Actor::Anonymous => None,
        }
    }

    pub fn on_behalf_of(&self) -> Option<Uuid> {
        match self {
            Actor::User { on_behalf_of, .. } => *on_behalf_of,
            Actor::System | Actor::Anonymous => None,
        }
    }
}

/// The resource an event is about: by id, by key (a setting, a flag), or both.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResourceRef {
    pub id: Option<Uuid>,
    pub key: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Allow,
    Deny,
}

/// The authorisation decision behind an event (filled from phase 03).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRecord {
    pub policy_ids: Vec<String>,
    pub effect: Effect,
    pub attrs_hash: [u8; 32],
}

/// The request, job or command that caused an event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestCtx {
    pub ip: Option<IpAddr>,
    pub session_id: Option<Uuid>,
    pub token_id: Option<Uuid>,
    pub job_id: Option<Uuid>,
    pub correlation_id: Uuid,
    pub ua: Option<String>,
}

impl RequestCtx {
    pub fn new(correlation_id: Uuid) -> Self {
        Self {
            ip: None,
            session_id: None,
            token_id: None,
            job_id: None,
            correlation_id,
            ua: None,
        }
    }

    /// The context of a job handler.
    pub fn system(job_id: Uuid, correlation_id: Uuid, kind: &str) -> Self {
        let mut ctx = Self::new(correlation_id).with_ua(&format!("job:{kind}"));
        ctx.job_id = Some(job_id);
        ctx
    }

    /// The context of an `akr-ctl` command.
    pub fn cli(correlation_id: Uuid, version: &str) -> Self {
        Self::new(correlation_id).with_ua(&format!("akr-ctl/{version}"))
    }

    pub fn with_ip(mut self, ip: Option<IpAddr>) -> Self {
        self.ip = ip;
        self
    }

    /// Sets the user agent, cut to 256 characters.
    pub fn with_ua(mut self, ua: &str) -> Self {
        self.ua = Some(ua.chars().take(UA_MAX_CHARS).collect());
        self
    }
}

/// One auditable fact. Built with [`AuditEvent::new`] and the builder methods; `detail` is the only way to
/// add a payload value, and it checks the name against the kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub kind: &'static EventKind,
    pub actor: Actor,
    pub resource: ResourceRef,
    pub case_id: Option<Uuid>,
    pub decision: Option<DecisionRecord>,
    pub request: RequestCtx,
    pub details: BTreeMap<&'static str, Canon>,
}

impl AuditEvent {
    pub fn new(kind: &'static EventKind, actor: Actor, request: RequestCtx) -> Self {
        Self {
            kind,
            actor,
            resource: ResourceRef::default(),
            case_id: None,
            decision: None,
            request,
            details: BTreeMap::new(),
        }
    }

    pub fn resource_id(mut self, id: Uuid) -> Self {
        self.resource.id = Some(id);
        self
    }

    pub fn resource_key(mut self, key: impl Into<String>) -> Self {
        self.resource.key = Some(key.into());
        self
    }

    pub fn case(mut self, case_id: Uuid) -> Self {
        self.case_id = Some(case_id);
        self
    }

    pub fn decision(mut self, decision: DecisionRecord) -> Self {
        self.decision = Some(decision);
        self
    }

    /// Adds a payload value. A name the kind does not list is refused.
    pub fn detail(mut self, key: &str, value: impl Into<Canon>) -> Result<Self, AuditError> {
        let known = self
            .kind
            .detail_keys
            .iter()
            .find(|k| **k == key)
            .ok_or_else(|| AuditError::DetailKeyNotInCatalogue {
                kind: self.kind.name,
                key: key.to_owned(),
            })?;
        self.details.insert(known, value.into());
        Ok(self)
    }
}

/// What a sink returns for a written event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditReceipt {
    pub id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub entry_hash: [u8; 32],
}

#[async_trait]
pub trait AuditSink: Send + Sync {
    /// Writes one event inside the caller's transaction, after the domain change it records.
    async fn emit(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        event: AuditEvent,
    ) -> Result<AuditReceipt, AuditError>;

    /// Writes one event in its own transaction: acts with no domain change (a refused sign-in, a CLI check).
    async fn emit_standalone(&self, event: AuditEvent) -> Result<AuditReceipt, AuditError>;
}

/// A sink that keeps events in memory, for tests. It keeps an event whether or not the transaction it rode in
/// commits.
#[derive(Debug, Default)]
pub struct RecordingSink {
    events: Mutex<Vec<AuditEvent>>,
}

impl RecordingSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn events(&self) -> Vec<AuditEvent> {
        self.events
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.events().iter().map(|e| e.kind.name).collect()
    }

    fn record(&self, event: AuditEvent) -> AuditReceipt {
        self.events
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(event);
        AuditReceipt {
            id: Uuid::now_v7(),
            occurred_at: Utc::now(),
            entry_hash: [0; 32],
        }
    }
}

#[async_trait]
impl AuditSink for RecordingSink {
    async fn emit(
        &self,
        _tx: &mut Transaction<'_, Postgres>,
        event: AuditEvent,
    ) -> Result<AuditReceipt, AuditError> {
        Ok(self.record(event))
    }

    async fn emit_standalone(&self, event: AuditEvent) -> Result<AuditReceipt, AuditError> {
        Ok(self.record(event))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::collections::HashSet;

    use chrono::TimeZone;
    use serde_json::json;

    use super::*;

    fn event(kind: &'static EventKind) -> AuditEvent {
        AuditEvent::new(kind, Actor::System, RequestCtx::new(Uuid::now_v7()))
    }

    #[test]
    fn a_detail_outside_the_kind_is_refused() {
        let ok = event(&kinds::SETTING_UPDATED)
            .resource_key("jobs.lease_s")
            .detail("before", 600)
            .unwrap()
            .detail("after", 900)
            .unwrap();
        assert_eq!(ok.details.get("after"), Some(&Canon::Int(900)));
        assert_eq!(ok.resource.key.as_deref(), Some("jobs.lease_s"));

        let err = event(&kinds::SETTING_UPDATED)
            .detail("payload", 1)
            .unwrap_err();
        assert!(matches!(
            err,
            AuditError::DetailKeyNotInCatalogue {
                kind: "setting.updated",
                ..
            }
        ));
    }

    #[test]
    fn a_float_is_refused_and_a_setting_decimal_becomes_text() {
        assert!(matches!(
            Canon::from_json("after", &json!(0.85)),
            Err(AuditError::FloatInPayload { key }) if key == "after"
        ));
        assert!(matches!(
            Canon::from_json("after", &json!({ "nested": [1, 2.5] })),
            Err(AuditError::FloatInPayload { .. })
        ));
        assert_eq!(
            Canon::from_json_decimal_as_text("after", &json!(0.85)).unwrap(),
            Canon::Str("0.85".to_owned())
        );
        assert_eq!(Canon::from_json("after", &json!(7)).unwrap(), Canon::Int(7));
    }

    #[test]
    fn a_string_with_a_nul_is_refused() {
        assert!(matches!(
            Canon::from_json("after", &json!("a\u{0}b")),
            Err(AuditError::Canonicalization(_))
        ));
    }

    #[test]
    fn json_objects_keep_their_content() {
        let value = json!({ "b": [true, null, "x"], "a": 1 });
        assert_eq!(Canon::from_json("after", &value).unwrap().to_json(), value);
    }

    #[test]
    fn timestamps_are_utc_with_six_digits() {
        let at = Utc.with_ymd_and_hms(2026, 10, 11, 9, 5, 7).unwrap();
        assert_eq!(
            Canon::from(at),
            Canon::Str("2026-10-11T09:05:07.000000Z".to_owned())
        );
    }

    #[test]
    fn the_system_actor_is_the_system_principal() {
        assert_eq!(
            Actor::System.id().unwrap().to_string(),
            "00000000-0000-0000-0000-000000000001"
        );
        assert_eq!(Actor::Anonymous.id(), None);
        let user = Uuid::now_v7();
        assert_eq!(Actor::user(user).id(), Some(user));
    }

    #[test]
    fn the_contexts_of_jobs_and_the_cli_name_themselves() {
        let job = Uuid::now_v7();
        let ctx = RequestCtx::system(job, Uuid::now_v7(), "maintenance_sweep");
        assert_eq!(ctx.job_id, Some(job));
        assert_eq!(ctx.ua.as_deref(), Some("job:maintenance_sweep"));
        assert_eq!(
            RequestCtx::cli(Uuid::now_v7(), "0.1.0").ua.as_deref(),
            Some("akr-ctl/0.1.0")
        );
        let long = RequestCtx::new(Uuid::now_v7()).with_ua(&"x".repeat(300));
        assert_eq!(long.ua.unwrap().chars().count(), 256);
    }

    #[test]
    fn kind_names_are_unique_and_dotted() {
        let mut seen = HashSet::new();
        for kind in kinds::PHASE_00 {
            assert!(seen.insert(kind.name), "{} is declared twice", kind.name);
            assert!(kind.name.contains('.'));
            assert_eq!(kind.stream, Stream::AuditLog);
        }
    }

    #[tokio::test]
    async fn the_recording_sink_keeps_what_it_is_given() {
        let sink = RecordingSink::new();
        sink.emit_standalone(event(&kinds::JOB_DEAD).detail("attempts", 5).unwrap())
            .await
            .unwrap();
        assert_eq!(sink.names(), vec!["job.dead"]);
    }
}
