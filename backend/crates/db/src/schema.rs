//! The schemas the product owns. The coverage checks run over them.

/// A phase that creates a schema adds it here in the same change.
pub const AKR_SCHEMAS: [&str; 12] = [
    "admin", "audit", "raw", "canon", "feat", "scn", "alert", "casework", "kyc", "ref", "rpt",
    "assess",
];

/// The ledger of the migration tool. It is not a table of the product: it carries no classification and
/// no retention class.
pub const MIGRATION_LEDGER: &str = "_sqlx_migrations";

pub(crate) fn akr_schemas() -> Vec<String> {
    AKR_SCHEMAS.iter().map(|s| (*s).to_owned()).collect()
}
