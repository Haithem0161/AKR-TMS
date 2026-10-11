//! Deadline arithmetic.
//!
//! A clock defined in working days must use the bank's calendar. That calendar arrives in phase 09
//! (`BankCalendarClock`); until then the only clock counts calendar time, and is named so nobody takes it
//! for working days.

use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockUnit {
    Hours,
    Days,
}

pub trait BusinessClock: Send + Sync {
    /// The instant `n` units of working time after `from`.
    fn add_working(&self, from: DateTime<Utc>, n: u32, unit: ClockUnit) -> DateTime<Utc>;
}

/// Counts every hour and every day, working or not.
#[derive(Debug, Clone, Copy, Default)]
pub struct CalendarDaysClock;

impl BusinessClock for CalendarDaysClock {
    fn add_working(&self, from: DateTime<Utc>, n: u32, unit: ClockUnit) -> DateTime<Utc> {
        let span = match unit {
            ClockUnit::Hours => Duration::hours(i64::from(n)),
            ClockUnit::Days => Duration::days(i64::from(n)),
        };
        from + span
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn the_calendar_clock_counts_weekends_and_nights() {
        let thursday_evening: DateTime<Utc> = "2026-10-08T18:00:00Z".parse().unwrap();
        let clock = CalendarDaysClock;
        assert_eq!(
            clock.add_working(thursday_evening, 3, ClockUnit::Days),
            "2026-10-11T18:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(
            clock.add_working(thursday_evening, 24, ClockUnit::Hours),
            "2026-10-09T18:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(
            clock.add_working(thursday_evening, 0, ClockUnit::Days),
            thursday_evening
        );
    }
}
