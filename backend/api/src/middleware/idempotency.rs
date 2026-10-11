//! Runs every mutation under the idempotency ledger (DR-21, DR-412).
//!
//! The claim on the key, the handler's writes and the stored answer are one transaction. A 2xx answer
//! commits it; any other answer, an error or a dropped request rolls it back, so nothing a failed attempt
//! did survives and the client may retry with the same key.

use std::error::Error as _;

use akr_common::AppError;
use akr_common::idempotency::{
    Claim, IDEMPOTENCY_KEY_HEADER, PrincipalId, REPLAY_HEADER, RequestTx, StoredResponse,
    is_valid_key, request_hash,
};
use akr_telemetry::metrics::IDEMPOTENCY_REPLAYS_TOTAL;
use axum::body::{Body, to_bytes};
use axum::extract::{MatchedPath, OriginalUri, Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use http::{HeaderValue, Method, StatusCode, header};
use http_body_util::LengthLimitError;
use sqlx::{Postgres, Transaction};

use crate::state::AppState;

/// Route templates that take no key. A route belongs here only when it runs before a principal exists;
/// phase 01 adds the sign-in routes.
const EXEMPT: &[&str] = &[];

pub async fn layer(State(state): State<AppState>, req: Request, next: Next) -> Response {
    if !is_mutating(req.method()) {
        return next.run(req).await;
    }
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|path| path.as_str().to_owned());
    if route
        .as_deref()
        .is_some_and(|route| EXEMPT.contains(&route))
    {
        return next.run(req).await;
    }
    match guarded(state, req, next, route).await {
        Ok(response) => response,
        Err(e) => e.into_response(),
    }
}

fn is_mutating(method: &Method) -> bool {
    matches!(
        *method,
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    )
}

async fn guarded(
    state: AppState,
    req: Request,
    next: Next,
    route: Option<String>,
) -> Result<Response, AppError> {
    let method = req.method().clone();
    // The nest strips its prefix from `req.uri()`; the hash covers the path the client sent.
    let target = req
        .extensions()
        .get::<OriginalUri>()
        .map_or_else(|| req.uri(), |original| &original.0)
        .path_and_query()
        .map_or_else(|| req.uri().path().to_owned(), |pq| pq.as_str().to_owned());
    let endpoint = format!("{method} {}", route.as_deref().unwrap_or("unmatched"));

    let Some(PrincipalId(principal)) = req.extensions().get::<PrincipalId>().copied() else {
        tracing::error!(
            endpoint,
            "a mutation reached the idempotency ledger with no principal"
        );
        return Err(AppError::Internal);
    };

    let key = match req.headers().get(IDEMPOTENCY_KEY_HEADER) {
        None => return Err(AppError::IdempotencyRequired),
        Some(sent) => match sent.to_str() {
            Ok(key) if is_valid_key(key) => key.to_owned(),
            _ => return Err(AppError::BadRequest("idempotency_key_malformed")),
        },
    };

    let (parts, body) = req.into_parts();
    let body = to_bytes(body, state.config.max_body_bytes)
        .await
        .map_err(|e| {
            if e.source()
                .is_some_and(|cause| cause.is::<LengthLimitError>())
            {
                AppError::PayloadTooLarge
            } else {
                AppError::BadRequest("body_unreadable")
            }
        })?;
    let hash = request_hash(method.as_str(), &target, principal, &body);

    let lease = state.idempotency.in_flight_lease().await;
    let mut tx = state.pool.begin().await?;
    match state
        .idempotency
        .claim(&mut tx, principal, &key, &endpoint, &hash, lease)
        .await
    {
        Ok(Claim::Run) => {}
        Ok(Claim::Replay(stored)) => {
            roll_back(tx).await;
            metrics::counter!(IDEMPOTENCY_REPLAYS_TOTAL).increment(1);
            return replay(stored);
        }
        Err(e) => {
            roll_back(tx).await;
            return Err(e);
        }
    }

    let request_tx = RequestTx::new(tx);
    let mut req = Request::from_parts(parts, Body::from(body));
    req.extensions_mut().insert(request_tx.clone());
    let response = next.run(req).await;

    if !response.status().is_success() {
        if let Some(tx) = request_tx.take() {
            roll_back(tx).await;
        }
        return Ok(response);
    }

    let (parts, body) = response.into_parts();
    let body = to_bytes(body, state.config.max_body_bytes)
        .await
        .map_err(|e| {
            tracing::error!(endpoint, error = %e, "the answer of a mutation cannot be kept for replay");
            AppError::Internal
        })?;
    let Some(mut tx) = request_tx.take() else {
        tracing::error!(endpoint, "a handler still holds the request transaction");
        return Err(AppError::Internal);
    };
    let stored = StoredResponse {
        status: parts.status.as_u16(),
        content_type: parts
            .headers
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned),
        body: body.to_vec(),
    };
    state
        .idempotency
        .complete(&mut tx, principal, &key, &stored)
        .await?;

    #[cfg(feature = "fault-injection")]
    if state.faults.take_die_before_commit() {
        drop(tx);
        return Err(AppError::Internal);
    }

    tx.commit().await?;
    for work in request_tx.take_after_commit() {
        work.await;
    }
    Ok(Response::from_parts(parts, Body::from(body)))
}

async fn roll_back(tx: Transaction<'static, Postgres>) {
    if let Err(e) = tx.rollback().await {
        tracing::debug!(error = %e, "a request transaction did not roll back cleanly");
    }
}

/// The stored answer, byte for byte, marked as a replay.
fn replay(stored: StoredResponse) -> Result<Response, AppError> {
    let status = StatusCode::from_u16(stored.status)
        .map_err(|_| AppError::internal("a stored status is not HTTP"))?;
    let mut response = Response::new(Body::from(stored.body));
    *response.status_mut() = status;
    let headers = response.headers_mut();
    if let Some(content_type) = stored
        .content_type
        .as_deref()
        .and_then(|value| HeaderValue::from_str(value).ok())
    {
        headers.insert(header::CONTENT_TYPE, content_type);
    }
    headers.insert(REPLAY_HEADER, HeaderValue::from_static("true"));
    Ok(response)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn only_methods_that_change_state_are_guarded() {
        for method in [Method::POST, Method::PUT, Method::PATCH, Method::DELETE] {
            assert!(is_mutating(&method));
        }
        for method in [Method::GET, Method::HEAD, Method::OPTIONS] {
            assert!(!is_mutating(&method));
        }
    }

    #[test]
    fn no_route_is_exempt_in_this_phase() {
        assert!(EXEMPT.is_empty());
    }

    #[tokio::test]
    async fn a_replay_carries_the_stored_status_type_and_bytes() {
        let response = replay(StoredResponse {
            status: 201,
            content_type: Some("application/json".to_owned()),
            body: br#"{"a":1}"#.to_vec(),
        })
        .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
        assert_eq!(response.headers()[REPLAY_HEADER], "true");
        let body = to_bytes(response.into_body(), 64).await.unwrap();
        assert_eq!(&body[..], br#"{"a":1}"#);
    }

    #[test]
    fn a_stored_status_outside_http_is_refused() {
        let refused = replay(StoredResponse {
            status: 99,
            content_type: None,
            body: Vec::new(),
        });
        assert_eq!(refused.unwrap_err(), AppError::Internal);
    }
}
