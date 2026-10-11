//! Who ran the command: every command that changes an installation says so in the audit sink.

use std::sync::Arc;

use akr_domain::audit::{Actor, AuditError, AuditEvent, AuditSink, RequestCtx, kinds};
use uuid::Uuid;

const NO_SESSION: &str = "none";
/// The one resource a command acts on.
const INSTALLATION: &str = "installation";

/// The operating-system user behind the command and the login session it runs in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operator {
    pub os_user: String,
    pub os_session: String,
}

impl Operator {
    pub fn from_env() -> Self {
        Self::from_vars(
            |name| std::env::var(name).ok(),
            rustix::process::getuid().as_raw(),
        )
    }

    /// `SUDO_USER` first: under `sudo` it is the person, where `LOGNAME` and `USER` are the target account.
    /// The session is the one PAM opened (`XDG_SESSION_ID`); a process outside any session has none.
    fn from_vars(var: impl Fn(&str) -> Option<String>, uid: u32) -> Self {
        let named = |name: &str| {
            var(name)
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty())
        };
        Self {
            os_user: named("SUDO_USER")
                .or_else(|| named("LOGNAME"))
                .or_else(|| named("USER"))
                .unwrap_or_else(|| format!("uid:{uid}")),
            os_session: named("XDG_SESSION_ID").unwrap_or_else(|| NO_SESSION.to_owned()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Ok,
    Failed,
}

impl Outcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Outcome::Ok => "ok",
            Outcome::Failed => "failed",
        }
    }
}

/// The audit trail of one invocation.
#[derive(Clone)]
pub struct Trail {
    sink: Arc<dyn AuditSink>,
    operator: Operator,
    correlation_id: Uuid,
}

impl Trail {
    pub fn new(sink: Arc<dyn AuditSink>, operator: Operator) -> Self {
        Self {
            sink,
            operator,
            correlation_id: Uuid::now_v7(),
        }
    }

    pub fn sink(&self) -> Arc<dyn AuditSink> {
        self.sink.clone()
    }

    pub fn operator(&self) -> &Operator {
        &self.operator
    }

    pub fn request(&self) -> RequestCtx {
        RequestCtx::cli(self.correlation_id, env!("CARGO_PKG_VERSION"))
    }

    /// Records that `command` ran. The actor is the system principal; the person is in the details.
    pub async fn command_run(&self, command: &str, outcome: Outcome) -> Result<(), AuditError> {
        let event = AuditEvent::new(&kinds::CTL_COMMAND_RUN, Actor::System, self.request())
            .resource_key(INSTALLATION)
            .detail("command", command)?
            .detail("os_user", self.operator.os_user.as_str())?
            .detail("os_session", self.operator.os_session.as_str())?
            .detail("outcome", outcome.as_str())?;
        self.sink.emit_standalone(event).await?;
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::collections::HashMap;

    use akr_domain::audit::{Canon, RecordingSink};

    use super::*;

    fn operator(vars: &[(&str, &str)]) -> Operator {
        let vars: HashMap<String, String> = vars
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        Operator::from_vars(|name| vars.get(name).cloned(), 1001)
    }

    #[test]
    fn the_person_behind_sudo_is_the_operator() {
        let found = operator(&[
            ("SUDO_USER", "dba1"),
            ("LOGNAME", "root"),
            ("USER", "root"),
            ("XDG_SESSION_ID", "42"),
        ]);
        assert_eq!(found.os_user, "dba1");
        assert_eq!(found.os_session, "42");
    }

    #[test]
    fn the_login_name_then_the_user_then_the_uid() {
        assert_eq!(
            operator(&[("LOGNAME", "ops"), ("USER", "x")]).os_user,
            "ops"
        );
        assert_eq!(operator(&[("USER", "x")]).os_user, "x");
        assert_eq!(operator(&[("USER", "  ")]).os_user, "uid:1001");
        assert_eq!(operator(&[]).os_session, "none");
    }

    #[tokio::test]
    async fn a_command_is_recorded_with_who_ran_it_and_how_it_ended() {
        let sink = Arc::new(RecordingSink::new());
        let trail = Trail::new(
            sink.clone(),
            operator(&[("SUDO_USER", "dba1"), ("XDG_SESSION_ID", "7")]),
        );

        trail
            .command_run("db migrate", Outcome::Failed)
            .await
            .unwrap();

        let events = sink.events();
        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert_eq!(event.kind.name, "ctl.command_run");
        assert_eq!(event.actor, Actor::System);
        assert_eq!(event.resource.key.as_deref(), Some("installation"));
        assert_eq!(event.request.ua.as_deref(), Some("akr-ctl/0.1.0"));
        let text = |key: &str| event.details.get(key).map(Canon::to_json).unwrap();
        assert_eq!(text("command"), "db migrate");
        assert_eq!(text("os_user"), "dba1");
        assert_eq!(text("os_session"), "7");
        assert_eq!(text("outcome"), "failed");
    }
}
