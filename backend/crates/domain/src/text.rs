//! Small shared value objects.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A label in the two interface languages. Both are required: a screen never falls back to a key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct I18nText {
    pub ar: String,
    pub en: String,
}

/// The code of a role (`analyst`, `mlro`, ...). Phase 01 narrows the set to the codes of MODULES.md
/// section 0; the wire form stays this string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(try_from = "String", into = "String")]
#[schema(value_type = String, example = "analyst")]
pub struct RoleCode(String);

impl RoleCode {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for RoleCode {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut bytes = value.bytes();
        let starts_lower = bytes.next().is_some_and(|b| b.is_ascii_lowercase());
        let rest_ok = bytes.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if starts_lower && rest_ok && value.len() <= 64 {
            Ok(RoleCode(value))
        } else {
            Err(format!("`{value}` is not a role code"))
        }
    }
}

impl From<RoleCode> for String {
    fn from(value: RoleCode) -> Self {
        value.0
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn a_role_code_is_lowercase_snake() {
        assert!(RoleCode::try_from("mlro".to_owned()).is_ok());
        assert!(RoleCode::try_from("data_steward".to_owned()).is_ok());
        for bad in ["", "MLRO", "1st", "a-b", "a b"] {
            assert!(RoleCode::try_from(bad.to_owned()).is_err(), "{bad}");
        }
    }

    #[test]
    fn a_role_code_travels_as_a_plain_string() {
        let code: RoleCode = serde_json::from_str("\"analyst\"").unwrap();
        assert_eq!(serde_json::to_string(&code).unwrap(), "\"analyst\"");
        assert!(serde_json::from_str::<RoleCode>("\"Not A Code\"").is_err());
    }
}
