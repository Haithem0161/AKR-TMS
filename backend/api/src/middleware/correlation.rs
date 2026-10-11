//! Gives every request its correlation id: the client's when it is a UUID, else a new one. The id is the
//! scope of the request's task (so a problem body and a job the request enqueues carry it), a field of
//! its log lines, and a header of the answer.

use akr_common::correlation::{self, CORRELATION_HEADER, CorrelationId};
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use http::HeaderValue;
use tracing::Instrument;

pub async fn layer(mut req: Request, next: Next) -> Response {
    let sent = req
        .headers()
        .get(CORRELATION_HEADER)
        .and_then(|value| value.to_str().ok());
    let id = CorrelationId::from_header_or_generate(sent);
    req.extensions_mut().insert(id);

    let span = tracing::info_span!("request", correlation_id = %id);
    let mut response = correlation::scope(id, next.run(req)).instrument(span).await;

    if let Ok(value) = HeaderValue::from_str(&id.to_string()) {
        response.headers_mut().insert(CORRELATION_HEADER, value);
    }
    response
}
