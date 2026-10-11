//! Liveness and readiness. They sit outside `/api/v1`: a probe needs no principal and no idempotency key.
//! The metrics exposition is not here; it is served on the management listener only.

use akr_common::infra::{self, Health, Readiness};
use axum::Json;
use axum::extract::State;
use http::StatusCode;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::readiness::ApiProbe;
use crate::routes::INFRA_TAG;
use crate::state::AppState;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(healthz))
        .routes(routes!(readyz))
}

/// Liveness
///
/// The process is up. It does not look at any dependency: a database blip must not get a healthy process
/// restarted.
#[utoipa::path(
    get,
    path = "/healthz",
    tag = INFRA_TAG,
    responses((status = 200, description = "The process is up", body = Health))
)]
pub async fn healthz() -> Json<Health> {
    infra::healthz().await
}

/// Readiness
///
/// Whether this instance can take requests: the database answers, its schema is not older than this
/// build, and Valkey answers. The body holds names and booleans only.
#[utoipa::path(
    get,
    path = "/readyz",
    tag = INFRA_TAG,
    responses(
        (status = 200, description = "Ready", body = Readiness),
        (status = 503, description = "A dependency is not ready", body = Readiness),
    )
)]
pub async fn readyz(State(state): State<AppState>) -> (StatusCode, Json<Readiness>) {
    infra::readyz(&ApiProbe::new(state.pool, state.valkey)).await
}
