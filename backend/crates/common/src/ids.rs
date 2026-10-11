//! Identifiers. Every id is a UUIDv7 made by the application, never a database default (DATA-MODEL.md):
//! ids sort by creation time, and a row can be named before it is inserted.

use uuid::Uuid;

pub fn new_id() -> Uuid {
    Uuid::now_v7()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_version_7_and_sort_by_creation() {
        let a = new_id();
        let b = new_id();
        assert_eq!(a.get_version_num(), 7);
        assert!(a < b);
    }
}
