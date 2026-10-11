//! Entities, value objects and the repository ports of AKR-TMS.
//!
//! This crate has no infrastructure: it depends on neither axum nor a database runtime, so the layering of
//! `.claude/rules/ddd.md` is a compile error to break. `sqlx` is here for the `FromRow` and `Type` derives
//! and for the transaction type in port signatures; no SQL text lives in this crate.

pub mod audit;
pub mod classification;
pub mod errors;
pub mod flags;
pub mod page;
pub mod repositories;
pub mod retention;
pub mod settings;
pub mod text;

pub use errors::{DomainError, FieldViolation};
pub use page::{Keyset, Page};
pub use text::{I18nText, RoleCode};
