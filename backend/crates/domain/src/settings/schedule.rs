//! Schedules of periodic jobs: the value of a `schedule.<kind>` setting.
//!
//! A schedule has six fields, seconds first (`0 30 2 * * *` is 02:30:00 every day). It is read in the bank's
//! time zone, so "02:30" stays 02:30 on the wall clock of the bank whatever the server's zone is.

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use croner::Cron;
use croner::parser::{CronParser, Seconds, Year};

const FIELDS: usize = 6;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ScheduleError {
    #[error("a schedule has six fields, seconds first; this one has {0}")]
    FieldCount(usize),
    #[error("the schedule does not parse: {0}")]
    Pattern(String),
    #[error("the schedule never comes due")]
    NeverDue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schedule {
    cron: Cron,
}

pub fn parse(expr: &str) -> Result<Schedule, ScheduleError> {
    let fields = expr.split_whitespace().count();
    if fields != FIELDS {
        return Err(ScheduleError::FieldCount(fields));
    }
    let cron = CronParser::builder()
        .seconds(Seconds::Required)
        .year(Year::Disallowed)
        .build()
        .parse(expr)
        .map_err(|e| ScheduleError::Pattern(e.to_string()))?;
    Ok(Schedule { cron })
}

impl Schedule {
    /// The first slot strictly after `after`, on the wall clock of `zone`.
    pub fn next_after(
        &self,
        after: DateTime<Utc>,
        zone: Tz,
    ) -> Result<DateTime<Utc>, ScheduleError> {
        self.cron
            .find_next_occurrence(&after.with_timezone(&zone), false)
            .map(|slot| slot.with_timezone(&Utc))
            .map_err(|_| ScheduleError::NeverDue)
    }

    /// The newest slot at or before `at`: the slot a scheduler tick at `at` is responsible for.
    pub fn slot_at(&self, at: DateTime<Utc>, zone: Tz) -> Result<DateTime<Utc>, ScheduleError> {
        self.cron
            .find_previous_occurrence(&at.with_timezone(&zone), true)
            .map(|slot| slot.with_timezone(&Utc))
            .map_err(|_| ScheduleError::NeverDue)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    const BAGHDAD: Tz = chrono_tz::Asia::Baghdad;

    fn utc(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, mo, d, h, mi, s).unwrap()
    }

    #[test]
    fn a_schedule_has_exactly_six_fields() {
        assert!(parse("0 30 2 * * *").is_ok());
        assert_eq!(parse("30 2 * * *"), Err(ScheduleError::FieldCount(5)));
        assert_eq!(
            parse("0 30 2 * * * 2027"),
            Err(ScheduleError::FieldCount(7))
        );
        assert_eq!(parse("@daily"), Err(ScheduleError::FieldCount(1)));
        assert!(matches!(
            parse("0 61 2 * * *"),
            Err(ScheduleError::Pattern(_))
        ));
    }

    #[test]
    fn the_slot_is_on_the_wall_clock_of_the_bank() {
        let daily = parse("0 30 2 * * *").unwrap();
        // 02:30 in Baghdad (UTC+3) is 23:30 UTC of the day before.
        let next = daily
            .next_after(utc(2026, 10, 11, 12, 0, 0), BAGHDAD)
            .unwrap();
        assert_eq!(next, utc(2026, 10, 11, 23, 30, 0));
    }

    #[test]
    fn next_after_is_strictly_after() {
        let daily = parse("0 30 2 * * *").unwrap();
        let slot = utc(2026, 10, 11, 23, 30, 0);
        assert_eq!(
            daily.next_after(slot, BAGHDAD).unwrap(),
            utc(2026, 10, 12, 23, 30, 0)
        );
    }

    #[test]
    fn slot_at_names_the_slot_a_tick_belongs_to() {
        let daily = parse("0 30 2 * * *").unwrap();
        let slot = utc(2026, 10, 11, 23, 30, 0);
        assert_eq!(daily.slot_at(slot, BAGHDAD).unwrap(), slot);
        assert_eq!(
            daily.slot_at(utc(2026, 10, 12, 9, 15, 7), BAGHDAD).unwrap(),
            slot
        );
        // Two ticks 30 s apart name the same slot, so two worker nodes enqueue one job.
        assert_eq!(
            daily
                .slot_at(utc(2026, 10, 12, 9, 15, 37), BAGHDAD)
                .unwrap(),
            slot
        );
    }
}
