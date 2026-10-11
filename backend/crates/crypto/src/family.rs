//! Data key families: which key a value is sealed under.

use std::fmt;
use std::str::FromStr;

use crate::error::CryptoError;

/// A family has one active data key per month. A later phase that needs a new family adds a variant and a
/// wire code here; a code is never reused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Family {
    Personal,
    Financial,
    SecretStr,
    Card,
    Idempotency,
    Evidence,
}

impl Family {
    pub const ALL: [Family; 6] = [
        Family::Personal,
        Family::Financial,
        Family::SecretStr,
        Family::Card,
        Family::Idempotency,
        Family::Evidence,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Family::Personal => "personal",
            Family::Financial => "financial",
            Family::SecretStr => "secret-str",
            Family::Card => "card",
            Family::Idempotency => "idempotency",
            Family::Evidence => "evidence",
        }
    }

    /// The byte that names the family inside a sealed value.
    pub fn code(self) -> u8 {
        match self {
            Family::Personal => 1,
            Family::Financial => 2,
            Family::SecretStr => 3,
            Family::Card => 4,
            Family::Idempotency => 5,
            Family::Evidence => 6,
        }
    }

    pub fn from_code(code: u8) -> Option<Family> {
        Family::ALL.into_iter().find(|f| f.code() == code)
    }
}

impl fmt::Display for Family {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Family {
    type Err = CryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Family::ALL
            .into_iter()
            .find(|f| f.as_str() == s)
            .ok_or_else(|| CryptoError::UnknownKey(format!("family {s}")))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn names_and_codes_are_unique_and_round_trip() {
        let names: HashSet<_> = Family::ALL.iter().map(|f| f.as_str()).collect();
        let codes: HashSet<_> = Family::ALL.iter().map(|f| f.code()).collect();
        assert_eq!(names.len(), Family::ALL.len());
        assert_eq!(codes.len(), Family::ALL.len());
        for family in Family::ALL {
            assert_eq!(Family::from_code(family.code()), Some(family));
            assert_eq!(family.as_str().parse::<Family>().unwrap(), family);
        }
        assert_eq!(Family::from_code(0), None);
        assert!("raw".parse::<Family>().is_err());
    }
}
