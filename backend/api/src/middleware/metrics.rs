//! Request counts and durations, labelled by the route template (`/api/v1/cases/{id}`), never by the path
//! a client sent: a label per id would be a series per case, and would put the id in the metrics.

use std::time::Instant;

use akr_telemetry::metrics::{HTTP_REQUEST_DURATION_SECONDS, HTTP_REQUESTS_TOTAL};
use axum::extract::{MatchedPath, Request};
use axum::middleware::Next;
use axum::response::Response;

/// The route label of a request that matched no route.
pub const UNMATCHED: &str = "unmatched";

pub async fn layer(req: Request, next: Next) -> Response {
    let method = req.method().as_str().to_owned();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map_or_else(|| UNMATCHED.to_owned(), |path| path.as_str().to_owned());

    let started = Instant::now();
    let response = next.run(req).await;
    let elapsed = started.elapsed().as_secs_f64();

    metrics::counter!(
        HTTP_REQUESTS_TOTAL,
        "method" => method.clone(),
        "route" => route.clone(),
        "status" => response.status().as_u16().to_string(),
    )
    .increment(1);
    metrics::histogram!(
        HTTP_REQUEST_DURATION_SECONDS,
        "method" => method,
        "route" => route,
    )
    .record(elapsed);

    response
}
