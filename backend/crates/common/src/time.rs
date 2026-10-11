//! Time. Instants are UTC; a business date is a calendar day in the bank's zone (`bank.timezone`).

use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;

/// The zone the seeded `bank.timezone` names. Used only when the setting cannot be read.
pub const DEFAULT_BANK_ZONE: Tz = chrono_tz::Asia::Baghdad;

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// An IANA zone name as a zone, or `None` when the list does not hold it.
pub fn zone(name: &str) -> Option<Tz> {
    Tz::from_str(name).ok()
}

/// The calendar day of `at` on the wall clock of `zone`.
pub fn business_date(at: DateTime<Utc>, zone: Tz) -> NaiveDate {
    at.with_timezone(&zone).date_naive()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn the_business_date_turns_at_midnight_in_the_bank_zone() {
        // 21:30 UTC is 00:30 the next day in Baghdad (UTC+3).
        let late = Utc.with_ymd_and_hms(2026, 10, 11, 21, 30, 0).unwrap();
        assert_eq!(
            business_date(late, DEFAULT_BANK_ZONE),
            NaiveDate::from_ymd_opt(2026, 10, 12).unwrap()
        );
        let early = Utc.with_ymd_and_hms(2026, 10, 11, 20, 59, 59).unwrap();
        assert_eq!(
            business_date(early, DEFAULT_BANK_ZONE),
            NaiveDate::from_ymd_opt(2026, 10, 11).unwrap()
        );
    }

    #[test]
    fn a_zone_must_be_on_the_iana_list() {
        assert_eq!(zone("Asia/Baghdad"), Some(DEFAULT_BANK_ZONE));
        assert_eq!(zone("Mars/Olympus"), None);
    }
}
