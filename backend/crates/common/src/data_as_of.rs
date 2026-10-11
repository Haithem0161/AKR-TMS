//! Data freshness (C-29, DR-39).
//!
//! A response that carries data derived from the CBS says how fresh that data is: the source business date
//! of the newest batch behind it, never the AKR clock. It is in the body as `data_as_of` with a `stale` flag,
//! and in the `X-Data-As-Of` header.

use chrono::NaiveDate;
use http::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const DATA_AS_OF_HEADER: HeaderName = HeaderName::from_static("x-data-as-of");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct DataAsOf {
    /// The source business date of the newest batch behind the response.
    pub data_as_of: NaiveDate,
    /// True when the source has stopped moving.
    pub stale: bool,
}

impl DataAsOf {
    pub fn header_value(&self) -> HeaderValue {
        HeaderValue::from_str(&self.data_as_of.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|_| HeaderValue::from_static(""))
    }

    pub fn apply(&self, headers: &mut HeaderMap) {
        headers.insert(DATA_AS_OF_HEADER, self.header_value());
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn the_header_is_the_business_date() {
        let as_of = DataAsOf {
            data_as_of: NaiveDate::from_ymd_opt(2026, 10, 9).unwrap(),
            stale: true,
        };
        let mut headers = HeaderMap::new();
        as_of.apply(&mut headers);
        assert_eq!(headers["x-data-as-of"], "2026-10-09");
        assert_eq!(
            serde_json::to_value(as_of).unwrap(),
            serde_json::json!({ "data_as_of": "2026-10-09", "stale": true })
        );
    }
}
