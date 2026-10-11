//! Column classification: the registry that drives masking, field encryption and log redaction.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// The class of a column (`admin.data_classification`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "data_classification", rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Classification {
    Public,
    Internal,
    Personal,
    Financial,
    DetectionLogic,
    SecretStr,
    Card,
}

impl Classification {
    pub const ALL: [Classification; 7] = [
        Classification::Public,
        Classification::Internal,
        Classification::Personal,
        Classification::Financial,
        Classification::DetectionLogic,
        Classification::SecretStr,
        Classification::Card,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Classification::Public => "public",
            Classification::Internal => "internal",
            Classification::Personal => "personal",
            Classification::Financial => "financial",
            Classification::DetectionLogic => "detection-logic",
            Classification::SecretStr => "secret-str",
            Classification::Card => "card",
        }
    }

    /// The classes whose values never reach a log line, a trace or a metric label.
    pub fn is_redacted(self) -> bool {
        matches!(
            self,
            Classification::Personal
                | Classification::Financial
                | Classification::SecretStr
                | Classification::Card
        )
    }

    /// The classes a column may be stored encrypted under (the CHECK of `admin.column_classification`).
    pub fn may_be_encrypted(self) -> bool {
        self.is_redacted()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ColumnClassification {
    pub schema_name: String,
    pub table_name: String,
    pub column_name: String,
    pub classification: Classification,
    pub encrypted: bool,
    pub keyed_hash: bool,
    pub note: Option<String>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn the_wire_names_are_the_enum_labels_of_migration_0004() {
        let names: Vec<&str> = Classification::ALL.iter().map(|c| c.as_str()).collect();
        assert_eq!(
            names,
            [
                "public",
                "internal",
                "personal",
                "financial",
                "detection-logic",
                "secret-str",
                "card"
            ]
        );
        for class in Classification::ALL {
            let json = serde_json::to_string(&class).unwrap();
            assert_eq!(json, format!("\"{}\"", class.as_str()));
        }
    }

    #[test]
    fn detection_logic_is_not_redacted_and_may_not_be_encrypted() {
        assert!(!Classification::DetectionLogic.is_redacted());
        assert!(!Classification::Internal.may_be_encrypted());
        assert!(Classification::Card.is_redacted());
    }
}
