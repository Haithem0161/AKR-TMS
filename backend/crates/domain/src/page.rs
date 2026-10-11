//! Keyset pagination: the position a list method takes and the page it returns.
//!
//! A list is read in the order of its sort columns. A position is the values of those columns on the last
//! row already seen, so a page costs the same however deep it is and a row inserted meanwhile never shifts
//! the rows after it. `akr-common` signs a position into the opaque `cursor` of the API.

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::errors::{DomainError, FieldViolation};

#[derive(Debug, Clone, PartialEq)]
pub struct Keyset {
    after: Vec<Value>,
    limit: u32,
}

impl Keyset {
    pub const DEFAULT_LIMIT: u32 = 50;
    pub const MAX_LIMIT: u32 = 200;

    /// The first page.
    pub fn first(limit: u32) -> Self {
        Self::after(Vec::new(), limit)
    }

    /// The page after the row whose sort columns hold `values`.
    pub fn after(values: Vec<Value>, limit: u32) -> Self {
        Self {
            after: values,
            limit: limit.clamp(1, Self::MAX_LIMIT),
        }
    }

    pub fn limit(&self) -> i64 {
        i64::from(self.limit)
    }

    /// How many rows a query reads: one more than the page, to learn whether another page follows.
    pub fn fetch(&self) -> i64 {
        self.limit() + 1
    }

    pub fn is_first(&self) -> bool {
        self.after.is_empty()
    }

    pub fn values(&self) -> &[Value] {
        &self.after
    }

    /// The value of sort column `index`, or `None` on the first page. A position made for another list
    /// (too few values, or a value of another type) is a validation failure of `cursor`.
    pub fn value<T: DeserializeOwned>(&self, index: usize) -> Result<Option<T>, DomainError> {
        if self.is_first() {
            return Ok(None);
        }
        self.after
            .get(index)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .map(Some)
            .ok_or_else(|| {
                DomainError::Validation(vec![FieldViolation::new("/cursor", "not_a_cursor")])
            })
    }
}

impl Default for Keyset {
    fn default() -> Self {
        Self::first(Self::DEFAULT_LIMIT)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Page<T> {
    pub items: Vec<T>,
    /// The position of the next page; `None` on the last page.
    pub next: Option<Keyset>,
}

impl<T> Page<T> {
    /// Builds a page from rows read with [`Keyset::fetch`]. `key` gives the sort column values of a row, in
    /// the order of the list's `ORDER BY`.
    pub fn from_rows(mut rows: Vec<T>, page: &Keyset, key: impl Fn(&T) -> Vec<Value>) -> Self {
        let limit = page.limit as usize;
        let more = rows.len() > limit;
        rows.truncate(limit);
        let next = match rows.last() {
            Some(last) if more => Some(Keyset::after(key(last), page.limit)),
            _ => None,
        };
        Page { items: rows, next }
    }

    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Page<U> {
        Page {
            items: self.items.into_iter().map(f).collect(),
            next: self.next,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn the_limit_is_held_between_one_and_the_maximum() {
        assert_eq!(Keyset::first(0).limit(), 1);
        assert_eq!(Keyset::first(50).limit(), 50);
        assert_eq!(Keyset::first(10_000).limit(), 200);
        assert_eq!(Keyset::default().limit(), 50);
        assert_eq!(Keyset::first(50).fetch(), 51);
    }

    #[test]
    fn a_full_read_gives_a_next_position_and_a_short_read_ends_the_list() {
        let page = Keyset::first(2);
        let full = Page::from_rows(vec![1, 2, 3], &page, |n| vec![json!(n)]);
        assert_eq!(full.items, vec![1, 2]);
        let next = full.next.unwrap();
        assert_eq!(next.values(), &[json!(2)]);
        assert_eq!(next.limit(), 2);

        let last = Page::from_rows(vec![3], &next, |n| vec![json!(n)]);
        assert_eq!(last.items, vec![3]);
        assert_eq!(last.next, None);

        let exact = Page::from_rows(vec![1, 2], &page, |n| vec![json!(n)]);
        assert_eq!(exact.next, None);
    }

    #[test]
    fn a_position_gives_typed_values_and_refuses_one_of_another_list() {
        assert_eq!(Keyset::first(10).value::<i64>(0).unwrap(), None);
        let page = Keyset::after(vec![json!("2026-10-11T00:00:00Z"), json!(7)], 10);
        assert_eq!(page.value::<i64>(1).unwrap(), Some(7));
        assert!(page.value::<i64>(0).is_err());
        assert!(page.value::<i64>(2).is_err());
    }

    #[test]
    fn map_keeps_the_position() {
        let page = Page::from_rows(vec![1, 2, 3], &Keyset::first(2), |n| vec![json!(n)]);
        let mapped = page.clone().map(|n| n.to_string());
        assert_eq!(mapped.items, vec!["1", "2"]);
        assert_eq!(mapped.next, page.next);
    }
}
