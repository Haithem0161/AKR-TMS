//! Optimistic concurrency on the wire: `If-Match` in, `ETag` out.
//!
//! A mutation of a versioned resource names the version it read as `If-Match: "<version>"`. A missing header
//! is 428 `E_PRECONDITION_REQUIRED`; a version that moved on is 409 `E_VERSION_CONFLICT` (the repository
//! reports it). Every response that carries a versioned resource carries its `ETag`.

use axum::extract::FromRequestParts;
use http::request::Parts;
use http::{HeaderMap, HeaderValue, header};

use crate::errors::AppError;

/// The `ETag` of a version: the number in double quotes, a strong validator.
pub fn etag(version: i32) -> HeaderValue {
    HeaderValue::from_str(&format!("\"{version}\""))
        .unwrap_or_else(|_| HeaderValue::from_static("\"0\""))
}

pub fn set_etag(headers: &mut HeaderMap, version: i32) {
    headers.insert(header::ETAG, etag(version));
}

/// The version an `If-Match` header names. `*` and weak validators are refused: the caller must say which
/// version it read.
pub fn if_match(headers: &HeaderMap) -> Result<i32, AppError> {
    let raw = headers
        .get(header::IF_MATCH)
        .ok_or(AppError::PreconditionRequired)?
        .to_str()
        .map_err(|_| AppError::BadRequest("if_match_malformed"))?;
    raw.trim()
        .strip_prefix('"')
        .and_then(|v| v.strip_suffix('"'))
        .and_then(|v| v.parse::<i32>().ok())
        .filter(|v| *v >= 1)
        .ok_or(AppError::BadRequest("if_match_malformed"))
}

/// Extractor for the version of `If-Match`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IfMatch(pub i32);

impl<S: Send + Sync> FromRequestParts<S> for IfMatch {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if_match(&parts.headers).map(IfMatch)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn headers(value: Option<&str>) -> HeaderMap {
        let mut h = HeaderMap::new();
        if let Some(v) = value {
            h.insert(header::IF_MATCH, HeaderValue::from_str(v).unwrap());
        }
        h
    }

    #[test]
    fn the_etag_is_the_quoted_version() {
        assert_eq!(etag(7), "\"7\"");
    }

    #[test]
    fn a_missing_header_is_428() {
        assert_eq!(
            if_match(&headers(None)),
            Err(AppError::PreconditionRequired)
        );
    }

    #[test]
    fn the_quoted_version_is_read_back() {
        assert_eq!(if_match(&headers(Some("\"7\""))), Ok(7));
        assert_eq!(if_match(&headers(Some("  \"12\" "))), Ok(12));
    }

    #[test]
    fn a_wildcard_a_weak_validator_or_a_bare_number_is_400() {
        for bad in [
            "*",
            "W/\"7\"",
            "7",
            "\"\"",
            "\"abc\"",
            "\"0\"",
            "\"-1\"",
            "\"7\", \"8\"",
        ] {
            assert_eq!(
                if_match(&headers(Some(bad))),
                Err(AppError::BadRequest("if_match_malformed")),
                "{bad}"
            );
        }
    }

    #[test]
    fn what_a_response_sends_a_request_can_send_back() {
        let mut h = HeaderMap::new();
        set_etag(&mut h, 3);
        let sent = h[header::ETAG].clone();
        let mut request = HeaderMap::new();
        request.insert(header::IF_MATCH, sent);
        assert_eq!(if_match(&request), Ok(3));
    }
}
