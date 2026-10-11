//! `akr-ctl ops`: what an operator records about the installation.

use std::path::Path;
use std::sync::Arc;

use akr_common::settings::SettingsService;
use akr_common::settings_registry::keys::OPS_LAST_RESTORE_DRILL_AT;
use akr_db::PgSettingsRepository;
use akr_domain::audit::{Actor, AuditEvent, kinds};
use akr_domain::repositories::SettingsRepository;
use akr_domain::settings::ChangeSource;
use chrono::{DateTime, SecondsFormat, Utc};
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::error::CtlError;
use crate::operator::Trail;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum DrillResult {
    Passed,
    Failed,
}

impl DrillResult {
    pub fn as_str(self) -> &'static str {
        match self {
            DrillResult::Passed => "passed",
            DrillResult::Failed => "failed",
        }
    }
}

/// Records a restore drill. A drill that passed moves `ops.last_restore_drill_at` to `now`; a drill that
/// failed is recorded and moves nothing, so the alert on an old drill keeps counting from the last restore
/// that worked. The setting and the audit event commit together.
pub async fn record_restore_drill(
    pool: &PgPool,
    trail: &Trail,
    result: DrillResult,
    evidence: &Path,
    now: DateTime<Utc>,
) -> Result<Vec<String>, CtlError> {
    let bytes = std::fs::read(evidence).map_err(CtlError::file(evidence))?;
    if bytes.is_empty() {
        return Err(CtlError::usage(format!(
            "{} is empty: the evidence of a drill is its report",
            evidence.display()
        )));
    }
    let evidence_sha256 = hex::encode(Sha256::digest(&bytes));

    let repo = Arc::new(PgSettingsRepository::new(pool.clone()));
    let settings = SettingsService::for_cli(repo.clone(), trail.sink());
    let current = repo.get(OPS_LAST_RESTORE_DRILL_AT).await?.ok_or_else(|| {
        CtlError::usage(format!(
            "the setting `{OPS_LAST_RESTORE_DRILL_AT}` is missing: run `db migrate`"
        ))
    })?;
    let request = trail.request();
    let at = now.to_rfc3339_opts(SecondsFormat::Secs, true);

    let mut tx = pool.begin().await?;
    if result == DrillResult::Passed {
        settings
            .apply(
                &mut tx,
                OPS_LAST_RESTORE_DRILL_AT,
                json!(at),
                current.version,
                &Actor::System,
                &request,
                ChangeSource::Direct,
            )
            .await?;
    }
    let event = AuditEvent::new(&kinds::RESTORE_DRILL_RECORDED, Actor::System, request)
        .resource_key(OPS_LAST_RESTORE_DRILL_AT)
        .detail("result", result.as_str())?
        .detail("evidence_sha256", evidence_sha256.as_str())?
        .detail("os_user", trail.operator().os_user.as_str())?
        .detail("os_session", trail.operator().os_session.as_str())?;
    trail.sink().emit(&mut tx, event).await?;
    tx.commit().await?;

    Ok(vec![
        format!("restore drill recorded: {}", result.as_str()),
        format!("evidence sha256: {evidence_sha256}"),
        match result {
            DrillResult::Passed => format!("{OPS_LAST_RESTORE_DRILL_AT} = {at}"),
            DrillResult::Failed => {
                format!(
                    "{OPS_LAST_RESTORE_DRILL_AT} is unchanged: only a drill that passed moves it"
                )
            }
        },
    ])
}
