//! The wait before a failed job or outbox row is tried again.

use std::time::Duration;

const BASE: Duration = Duration::from_secs(2);
const CAP: Duration = Duration::from_secs(3_600);

/// `2^attempts * 2 s`, capped at one hour. `attempts` counts the attempts made so far.
pub fn backoff(attempts: i32) -> Duration {
    let exponent = u32::try_from(attempts).unwrap_or(0);
    2u32.checked_pow(exponent)
        .and_then(|factor| BASE.checked_mul(factor))
        .map_or(CAP, |wait| wait.min(CAP))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_wait_doubles_with_every_attempt_up_to_one_hour() {
        assert_eq!(backoff(0), Duration::from_secs(2));
        assert_eq!(backoff(1), Duration::from_secs(4));
        assert_eq!(backoff(2), Duration::from_secs(8));
        assert_eq!(backoff(5), Duration::from_secs(64));
        assert_eq!(backoff(10), Duration::from_secs(2_048));
        assert_eq!(backoff(11), CAP);
        assert_eq!(backoff(31), CAP);
        assert_eq!(backoff(1_000), CAP);
        assert_eq!(backoff(-3), Duration::from_secs(2));
    }
}
