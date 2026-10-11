//! `POST /api/v1/_echo`: the route the idempotency ledger is exercised with before a business route
//! exists. It is mounted outside production only, and writes nothing.

use akr_common::AppError;
use akr_common::errors::ProblemResponse;
use akr_common::ids::new_id;
use axum::Json;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

use crate::routes::INFRA_TAG;
use crate::state::AppState;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(echo))
}

/// Any JSON object.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(value_type = Object)]
pub struct EchoRequest(pub Map<String, Value>);

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EchoResponse {
    /// The object that was sent.
    #[schema(value_type = Object)]
    pub echo: Map<String, Value>,
    /// New on every run of the handler: a replay shows the id of the first run.
    pub execution_id: Uuid,
}

/// Echo (development)
///
/// Answers with the object it was sent. The body is read as JSON whatever its content type says.
#[utoipa::path(
    post,
    path = "/_echo",
    tag = INFRA_TAG,
    request_body = EchoRequest,
    params(
        ("Idempotency-Key" = String, Header, description = "8 to 255 printable ASCII characters"),
    ),
    responses(
        (status = 200, description = "The object that was sent", body = EchoResponse),
        (status = 400, response = ProblemResponse),
        (status = 401, response = ProblemResponse),
        (status = 409, response = ProblemResponse),
        (status = 413, response = ProblemResponse),
        (status = 422, response = ProblemResponse),
    )
)]
pub async fn echo(body: Bytes) -> Result<Json<EchoResponse>, AppError> {
    let EchoRequest(echo) =
        serde_json::from_slice(&body).map_err(|_| AppError::invalid("", "not_a_json_object"))?;
    Ok(Json(EchoResponse {
        echo,
        execution_id: new_id(),
    }))
}
