//! The opaque `cursor` of the API (docs/API.md section 1, 7.26).
//!
//! A cursor is a [`Keyset`] position, signed: `base64url(values) "." base64url(HMAC-SHA-256)`. The signature
//! covers the name of the list, so a cursor of one list is refused by another, and a client cannot build a
//! position by hand to probe rows it was not shown. The key is the KV entry `keys/cursor-hmac`.

use akr_crypto::purpose::hmac_sha256;
pub use akr_domain::page::{Keyset, Page};
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64URL;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use subtle::ConstantTimeEq;
use utoipa::{IntoParams, ToSchema};
use zeroize::Zeroizing;

use crate::errors::{AppError, ValidationError};

/// The KV entry that holds the signing key.
pub const CURSOR_KEY_PATH: &str = "keys/cursor-hmac";

const MIN_KEY_LEN: usize = 32;
const MAX_CURSOR_LEN: usize = 2048;

/// The pagination parameters of a list route.
#[derive(Debug, Clone, Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PageQuery {
    /// Rows per page, 1 to 200. Default 50.
    pub limit: Option<u32>,
    /// The `next_cursor` of the page before. Absent for the first page.
    pub cursor: Option<String>,
}

/// The body of a list response.
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
pub struct Paged<T> {
    pub items: Vec<T>,
    /// Send it back as `cursor` for the next page. Absent on the last page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

pub struct CursorCodec {
    key: Zeroizing<Vec<u8>>,
}

impl CursorCodec {
    pub fn new(key: Zeroizing<Vec<u8>>) -> Result<Self, AppError> {
        if key.len() < MIN_KEY_LEN {
            return Err(AppError::internal(
                "the cursor key is shorter than 256 bits",
            ));
        }
        Ok(Self { key })
    }

    /// The position a request asks for. `list` names the list (its route template).
    pub fn decode(&self, list: &str, query: &PageQuery) -> Result<Keyset, AppError> {
        let limit = match query.limit {
            None => Keyset::DEFAULT_LIMIT,
            Some(n) if (1..=Keyset::MAX_LIMIT).contains(&n) => n,
            Some(_) => {
                return Err(AppError::Validation(vec![
                    ValidationError::new("/limit", "out_of_range")
                        .param("min", 1)
                        .param("max", Keyset::MAX_LIMIT),
                ]));
            }
        };
        let Some(cursor) = query.cursor.as_deref() else {
            return Ok(Keyset::first(limit));
        };
        let values = self
            .open(list, cursor)
            .ok_or(AppError::BadRequest("cursor_malformed"))?;
        Ok(Keyset::after(values, limit))
    }

    pub fn encode(&self, list: &str, position: &Keyset) -> Result<String, AppError> {
        let payload = serde_json::to_vec(position.values()).map_err(AppError::internal)?;
        let mac = self.mac(list, &payload)?;
        Ok(format!(
            "{}.{}",
            BASE64URL.encode(&payload),
            BASE64URL.encode(mac)
        ))
    }

    /// The response body of a page.
    pub fn paged<T>(&self, list: &str, page: Page<T>) -> Result<Paged<T>, AppError> {
        let next_cursor = page
            .next
            .as_ref()
            .map(|next| self.encode(list, next))
            .transpose()?;
        Ok(Paged {
            items: page.items,
            next_cursor,
        })
    }

    fn open(&self, list: &str, cursor: &str) -> Option<Vec<Value>> {
        if cursor.len() > MAX_CURSOR_LEN {
            return None;
        }
        let (payload, mac) = cursor.split_once('.')?;
        let payload = BASE64URL.decode(payload).ok()?;
        let mac = BASE64URL.decode(mac).ok()?;
        let expected = self.mac(list, &payload).ok()?;
        if mac.len() != expected.len() || !bool::from(mac.ct_eq(&expected)) {
            return None;
        }
        let values: Vec<Value> = serde_json::from_slice(&payload).ok()?;
        (!values.is_empty()).then_some(values)
    }

    fn mac(&self, list: &str, payload: &[u8]) -> Result<[u8; 32], AppError> {
        hmac_sha256(&self.key, &[list.as_bytes(), &[0u8], payload]).map_err(AppError::internal)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::*;

    const LIST: &str = "GET /api/v1/admin/jobs";

    fn codec(byte: u8) -> CursorCodec {
        CursorCodec::new(Zeroizing::new(vec![byte; 32])).unwrap()
    }

    fn query(limit: Option<u32>, cursor: Option<String>) -> PageQuery {
        PageQuery { limit, cursor }
    }

    #[test]
    fn a_short_key_is_refused() {
        assert!(CursorCodec::new(Zeroizing::new(vec![1; 16])).is_err());
    }

    #[test]
    fn no_cursor_is_the_first_page_with_the_default_limit() {
        let page = codec(1).decode(LIST, &query(None, None)).unwrap();
        assert!(page.is_first());
        assert_eq!(page.limit(), 50);
    }

    #[test]
    fn a_limit_outside_1_to_200_is_a_validation_failure() {
        for bad in [0, 201, 10_000] {
            let err = codec(1).decode(LIST, &query(Some(bad), None)).unwrap_err();
            let AppError::Validation(errors) = err else {
                panic!("expected a validation failure");
            };
            assert_eq!(errors[0].pointer, "/limit");
            assert_eq!(errors[0].params["max"], 200);
        }
        assert_eq!(
            codec(1)
                .decode(LIST, &query(Some(200), None))
                .unwrap()
                .limit(),
            200
        );
    }

    #[test]
    fn a_position_round_trips() {
        let c = codec(1);
        let position = Keyset::after(vec![json!("2026-10-11T09:00:00Z"), json!("0198c1de")], 25);
        let cursor = c.encode(LIST, &position).unwrap();
        assert!(
            cursor
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
        );
        let back = c.decode(LIST, &query(Some(25), Some(cursor))).unwrap();
        assert_eq!(back, position);
    }

    #[test]
    fn a_cursor_of_another_list_another_key_or_a_changed_byte_is_refused() {
        let c = codec(1);
        let position = Keyset::after(vec![json!(7)], 50);
        let cursor = c.encode(LIST, &position).unwrap();
        let refused = Err(AppError::BadRequest("cursor_malformed"));

        assert_eq!(
            c.decode("GET /api/v1/cases", &query(None, Some(cursor.clone()))),
            refused
        );
        assert_eq!(
            codec(2).decode(LIST, &query(None, Some(cursor.clone()))),
            refused
        );

        let forged = format!(
            "{}.{}",
            BASE64URL.encode(b"[8]"),
            cursor.split_once('.').unwrap().1
        );
        assert_eq!(c.decode(LIST, &query(None, Some(forged))), refused);

        for junk in ["", ".", "abc", "a.b.c", &"x".repeat(5000)] {
            assert_eq!(
                c.decode(LIST, &query(None, Some(junk.to_owned()))),
                refused,
                "{junk}"
            );
        }
    }

    #[test]
    fn the_last_page_has_no_next_cursor() {
        let c = codec(1);
        let more = Page::from_rows(vec![1, 2, 3], &Keyset::first(2), |n| vec![json!(n)]);
        let body = c.paged(LIST, more).unwrap();
        assert_eq!(body.items, vec![1, 2]);
        let next = body.next_cursor.unwrap();
        assert_eq!(
            c.decode(LIST, &query(Some(2), Some(next)))
                .unwrap()
                .values(),
            &[json!(2)]
        );

        let last = Page::from_rows(vec![3], &Keyset::first(2), |n| vec![json!(n)]);
        let body = c.paged(LIST, last).unwrap();
        assert_eq!(body.next_cursor, None);
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({ "items": [3] })
        );
    }
}
