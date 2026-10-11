//! The registry of job kinds and timer kinds: every kind the code enqueues, declared once.
//!
//! A binary claims only the kinds it registered a handler for, so a kind another binary owns, or one a newer
//! release added, is left alone.

use std::time::Duration;

/// One kind of background job.
#[derive(Debug, PartialEq, Eq)]
pub struct JobKind {
    pub name: &'static str,
    /// How many jobs of the kind run at once, across every worker of the installation.
    pub concurrency: u32,
    /// How many attempts a job gets before it is dead.
    pub max_attempts: i32,
    /// How long a claim lasts without a heartbeat. `None`: the setting `jobs.lease_s`.
    pub lease: Option<Duration>,
    /// The setting that holds the schedule of a periodic kind (`schedule.<kind>`).
    pub schedule: Option<&'static str>,
}

impl JobKind {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            concurrency: 1,
            max_attempts: 5,
            lease: None,
            schedule: None,
        }
    }

    pub const fn concurrency(mut self, concurrency: u32) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub const fn max_attempts(mut self, max_attempts: i32) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    pub const fn lease(mut self, lease: Duration) -> Self {
        self.lease = Some(lease);
        self
    }

    pub const fn scheduled_by(mut self, setting: &'static str) -> Self {
        self.schedule = Some(setting);
        self
    }
}

/// One kind of timer. A due timer enqueues the job `timer_<name>`.
#[derive(Debug, PartialEq, Eq)]
pub struct TimerKind {
    pub name: &'static str,
    /// The kinds of owner a timer of this kind may belong to.
    pub owner_kinds: &'static [&'static str],
    pub job: JobKind,
}

/// The job kinds of phase 00. Every later phase adds its kinds here.
pub mod kinds {
    use akr_common::settings_registry::keys;

    use super::JobKind;

    /// Deletes finished jobs, dispatched outbox rows, old idempotency rows and terminal timers past their
    /// retention.
    pub static MAINTENANCE_SWEEP: JobKind =
        JobKind::new("maintenance_sweep").scheduled_by(keys::SCHEDULE_MAINTENANCE_SWEEP);

    /// Measures the age of the last backup and of the last restore drill.
    pub static BACKUP_CHECK: JobKind =
        JobKind::new("backup_check").scheduled_by(keys::SCHEDULE_BACKUP_CHECK);

    pub static PHASE_00: &[&JobKind] = &[&MAINTENANCE_SWEEP, &BACKUP_CHECK];
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use akr_common::settings_registry;

    use super::*;

    #[test]
    fn kinds_are_named_once_and_a_periodic_kind_has_its_schedule_setting() {
        let mut names = HashSet::new();
        for kind in kinds::PHASE_00 {
            assert!(names.insert(kind.name), "{} is declared twice", kind.name);
            assert!(kind.concurrency >= 1);
            assert!(kind.max_attempts >= 1);
            if let Some(setting) = kind.schedule {
                assert_eq!(setting, format!("schedule.{}", kind.name));
                assert!(
                    settings_registry::lookup(setting).is_some(),
                    "{setting} is not in the settings registry"
                );
            }
        }
    }
}
