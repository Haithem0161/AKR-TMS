//! The metric names of the harness (TS-28). Labels carry a kind, a topic or a loop name: never an id.

use std::time::{SystemTime, UNIX_EPOCH};

pub const JOBS_QUEUED: &str = "akr_jobs_queued";
pub const JOBS_RUNNING: &str = "akr_jobs_running";
pub const JOBS_DEAD_TOTAL: &str = "akr_jobs_dead_total";
pub const JOB_DURATION_SECONDS: &str = "akr_job_duration_seconds";
pub const OUTBOX_PENDING: &str = "akr_outbox_pending";
pub const OUTBOX_STUCK: &str = "akr_outbox_stuck";
pub const TIMERS_DUE: &str = "akr_timers_due";
pub const LOOP_HEARTBEAT_SECONDS: &str = "akr_loop_heartbeat_seconds";

/// Records that a loop completed a round: the gauge holds the time of the round, in seconds since the epoch,
/// so an alert rule can compare it with the clock.
pub fn heartbeat(loop_name: &'static str) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64());
    metrics::gauge!(LOOP_HEARTBEAT_SECONDS, "loop" => loop_name).set(now);
}
