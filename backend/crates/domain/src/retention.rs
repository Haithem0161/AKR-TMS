//! Retention assignment: which class a table belongs to and which clock ages its rows.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// The retention classes R1 to R13 of DATA-MODEL.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "text")]
pub enum RetentionClass {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
}

impl RetentionClass {
    /// R10 is kept for ever: its clock is `never` and nothing deletes it.
    pub fn is_permanent(self) -> bool {
        self == RetentionClass::R10
    }
}

/// What starts the retention period of a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RetentionClock {
    Partition,
    Age,
    RelationshipEnd,
    Closure,
    Decision,
    Retirement,
    Owner,
    Never,
}

impl RetentionClock {
    /// The clocks that read a column of the row (the CHECK of `admin.retention_assignment`).
    pub fn reads_a_column(self) -> bool {
        matches!(
            self,
            RetentionClock::Partition
                | RetentionClock::Age
                | RetentionClock::RelationshipEnd
                | RetentionClock::Closure
                | RetentionClock::Decision
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct RetentionAssignment {
    pub schema_name: String,
    pub table_name: String,
    pub retention_class: RetentionClass,
    pub clock: RetentionClock,
    pub clock_column: Option<String>,
    pub note: Option<String>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn only_r10_is_permanent() {
        assert!(RetentionClass::R10.is_permanent());
        assert!(!RetentionClass::R9.is_permanent());
        assert!(!RetentionClass::R12.is_permanent());
    }

    #[test]
    fn the_clocks_that_read_a_column_match_the_table_check() {
        for clock in [
            RetentionClock::Retirement,
            RetentionClock::Owner,
            RetentionClock::Never,
        ] {
            assert!(!clock.reads_a_column());
        }
        assert!(RetentionClock::RelationshipEnd.reads_a_column());
        assert_eq!(
            serde_json::to_string(&RetentionClock::RelationshipEnd).unwrap(),
            "\"relationship_end\""
        );
    }
}
