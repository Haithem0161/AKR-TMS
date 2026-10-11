//! The error catalogue: every failure the API returns, as RFC 7807 `application/problem+json`.
//!
//! A problem body is `{ type, title, status, code, errors?, correlation_id }` (docs/API.md section 1). `code`
//! is the stable machine string of API.md section 6; `title` is a fixed English string per code, and the
//! frontend owns the Arabic and English text. Nothing about the cause leaves the process: it is logged with
//! the correlation id, which is how a failure a user saw is found again.

use std::time::Duration;

use akr_crypto::CryptoError;
use akr_domain::audit::AuditError;
use akr_domain::{DomainError, FieldViolation};
use axum::response::{IntoResponse, Response};
use http::{HeaderValue, StatusCode, header};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use utoipa::{ToResponse, ToSchema};
use uuid::Uuid;

use crate::correlation::{self, CorrelationId};

pub const PROBLEM_CONTENT_TYPE: &str = "application/problem+json";
const TYPE_BASE: &str = "https://akr-tms.local/errors/";

/// One failure of a request, located by JSON pointer (RFC 6901). An empty pointer means the request as a
/// whole: the reason behind a 400 or a 409 travels this way.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ValidationError {
    pub pointer: String,
    /// Stable machine code of the rule, e.g. `required`, `not_a_cron`, `governed`.
    pub code: String,
    /// The facts the rule's sentence needs that only the server holds.
    #[schema(value_type = Object)]
    pub params: Map<String, Value>,
}

impl ValidationError {
    pub fn new(pointer: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            pointer: pointer.into(),
            code: code.into(),
            params: Map::new(),
        }
    }

    pub fn param(mut self, name: &str, value: impl Into<Value>) -> Self {
        self.params.insert(name.to_owned(), value.into());
        self
    }
}

impl From<FieldViolation> for ValidationError {
    fn from(v: FieldViolation) -> Self {
        Self {
            pointer: v.pointer,
            code: v.code.to_owned(),
            params: v.params,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Problem {
    /// URI of the problem type.
    #[serde(rename = "type")]
    pub type_uri: String,
    /// Fixed English summary of the code.
    pub title: String,
    pub status: u16,
    /// The field a client branches on (API.md section 6).
    pub code: String,
    /// The places and rules that failed. Present for `E_VALIDATION`, and for a 400 or 409 that has a reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationError>>,
    /// The version the resource has now. Present for `E_VERSION_CONFLICT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_version: Option<i32>,
    pub correlation_id: Uuid,
}

/// The shared error response of every operation. A newtype, so the OpenAPI document holds one `$ref` and
/// the generated client one error type.
#[derive(ToResponse)]
#[response(
    description = "Error response (RFC 7807)",
    content_type = "application/problem+json"
)]
pub struct ProblemResponse(#[allow(dead_code)] Problem);

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum AppError {
    /// A malformed request. The reason is a stable sub-code, sent in `errors[0].code`.
    #[error("bad request: {0}")]
    BadRequest(&'static str),

    #[error("a mutation needs an Idempotency-Key")]
    IdempotencyRequired,

    #[error("no valid session or token")]
    Unauthenticated,

    #[error("a step-up assertion is required")]
    StepUpRequired,

    #[error("the session or token lacks the scope")]
    Scope,

    #[error("forbidden")]
    Forbidden,

    #[error("a maker cannot decide their own item")]
    SelfApproval,

    /// Missing, or masked: one body for both (phase 03).
    #[error("not found")]
    NotFound,

    #[error("version conflict (current version {current})")]
    VersionConflict { current: i32 },

    #[error("the same idempotency key is still executing")]
    IdempotencyInFlight,

    /// The request is well-formed and the resource's state refuses it. Carries the reason and its facts.
    #[error("not allowed in this state: {}", .0.code)]
    NotAllowedInState(ValidationError),

    #[error("payload too large")]
    PayloadTooLarge,

    #[error("validation failed")]
    Validation(Vec<ValidationError>),

    #[error("the idempotency key was used for another request")]
    IdempotencyReuse,

    #[error("the value is outside its regulatory limit")]
    BelowFloor,

    #[error("a versioned mutation needs If-Match")]
    PreconditionRequired,

    #[error("rate limited")]
    RateLimited { retry_after: Duration },

    /// An unexpected failure. Built with [`AppError::internal`], which logs the cause.
    #[error("internal error")]
    Internal,

    /// A dependency is down. Built with [`AppError::unavailable`], which logs the cause.
    #[error("a dependency is unavailable")]
    Unavailable,
}

impl AppError {
    /// Logs `cause` and returns `Internal`. The cause never reaches a client.
    pub fn internal(cause: impl std::fmt::Display) -> Self {
        tracing::error!(error = %cause, "internal error");
        AppError::Internal
    }

    /// Logs `cause` against the dependency and returns `Unavailable`.
    pub fn unavailable(dependency: &'static str, cause: impl std::fmt::Display) -> Self {
        tracing::error!(dependency, error = %cause, "dependency unavailable");
        AppError::Unavailable
    }

    pub fn not_allowed(reason: &'static str) -> Self {
        AppError::NotAllowedInState(ValidationError::new("", reason))
    }

    /// One field-level failure.
    pub fn invalid(pointer: &str, code: &'static str) -> Self {
        AppError::Validation(vec![ValidationError::new(pointer, code)])
    }

    pub fn status(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) | AppError::IdempotencyRequired => StatusCode::BAD_REQUEST,
            AppError::Unauthenticated | AppError::StepUpRequired => StatusCode::UNAUTHORIZED,
            AppError::Scope | AppError::Forbidden | AppError::SelfApproval => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::VersionConflict { .. }
            | AppError::IdempotencyInFlight
            | AppError::NotAllowedInState(_) => StatusCode::CONFLICT,
            AppError::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            AppError::Validation(_) | AppError::IdempotencyReuse | AppError::BelowFloor => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            AppError::PreconditionRequired => StatusCode::PRECONDITION_REQUIRED,
            AppError::RateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            AppError::BadRequest(_) => "E_BAD_REQUEST",
            AppError::IdempotencyRequired => "E_IDEMPOTENCY_REQUIRED",
            AppError::Unauthenticated => "E_UNAUTHENTICATED",
            AppError::StepUpRequired => "E_STEPUP_REQUIRED",
            AppError::Scope => "E_SCOPE",
            AppError::Forbidden => "E_FORBIDDEN",
            AppError::SelfApproval => "E_SELF_APPROVAL",
            AppError::NotFound => "E_NOT_FOUND",
            AppError::VersionConflict { .. } => "E_VERSION_CONFLICT",
            AppError::IdempotencyInFlight => "E_IDEMPOTENCY_IN_FLIGHT",
            AppError::NotAllowedInState(_) => "E_NOT_ALLOWED_IN_STATE",
            AppError::PayloadTooLarge => "E_PAYLOAD_TOO_LARGE",
            AppError::Validation(_) => "E_VALIDATION",
            AppError::IdempotencyReuse => "E_IDEMPOTENCY_REUSE",
            AppError::BelowFloor => "E_BELOW_FLOOR",
            AppError::PreconditionRequired => "E_PRECONDITION_REQUIRED",
            AppError::RateLimited { .. } => "E_RATE_LIMITED",
            AppError::Internal => "E_INTERNAL",
            AppError::Unavailable => "E_UNAVAILABLE",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            AppError::BadRequest(_) => "Bad Request",
            AppError::IdempotencyRequired => "Idempotency Key Required",
            AppError::Unauthenticated => "Unauthenticated",
            AppError::StepUpRequired => "Step-Up Required",
            AppError::Scope => "Insufficient Scope",
            AppError::Forbidden => "Forbidden",
            AppError::SelfApproval => "Self Approval",
            AppError::NotFound => "Not Found",
            AppError::VersionConflict { .. } => "Version Conflict",
            AppError::IdempotencyInFlight => "Request In Flight",
            AppError::NotAllowedInState(_) => "Not Allowed In This State",
            AppError::PayloadTooLarge => "Payload Too Large",
            AppError::Validation(_) => "Validation Failed",
            AppError::IdempotencyReuse => "Idempotency Key Reused",
            AppError::BelowFloor => "Below Floor",
            AppError::PreconditionRequired => "Precondition Required",
            AppError::RateLimited { .. } => "Rate Limited",
            AppError::Internal => "Internal Error",
            AppError::Unavailable => "Service Unavailable",
        }
    }

    pub fn to_problem(&self, correlation_id: Uuid) -> Problem {
        let errors = match self {
            AppError::Validation(errors) => Some(errors.clone()),
            AppError::NotAllowedInState(reason) => Some(vec![reason.clone()]),
            AppError::BadRequest(reason) => Some(vec![ValidationError::new("", *reason)]),
            _ => None,
        };
        let current_version = match self {
            AppError::VersionConflict { current } => Some(*current),
            _ => None,
        };
        Problem {
            type_uri: type_uri(self.code()),
            title: self.title().to_owned(),
            status: self.status().as_u16(),
            code: self.code().to_owned(),
            errors,
            current_version,
            correlation_id,
        }
    }
}

/// `E_VERSION_CONFLICT` -> `https://akr-tms.local/errors/version-conflict`.
fn type_uri(code: &str) -> String {
    let slug = code
        .strip_prefix("E_")
        .unwrap_or(code)
        .to_ascii_lowercase()
        .replace('_', "-");
    format!("{TYPE_BASE}{slug}")
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let correlation_id = correlation::current().unwrap_or_else(CorrelationId::generate);
        let problem = self.to_problem(correlation_id.as_uuid());
        let body = serde_json::to_vec(&problem).unwrap_or_else(|_| b"{}".to_vec());

        let mut response = (self.status(), body).into_response();
        let headers = response.headers_mut();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(PROBLEM_CONTENT_TYPE),
        );
        headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
        if let AppError::RateLimited { retry_after } = &self {
            let seconds = retry_after.as_secs().max(1);
            headers.insert(header::RETRY_AFTER, HeaderValue::from(seconds));
        }
        response
    }
}

impl From<DomainError> for AppError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::NotFound => AppError::NotFound,
            DomainError::VersionConflict { current } => AppError::VersionConflict { current },
            DomainError::Validation(violations) => {
                AppError::Validation(violations.into_iter().map(ValidationError::from).collect())
            }
            DomainError::BelowFloor { key } => {
                tracing::debug!(key, "a value outside its regulatory limit was refused");
                AppError::BelowFloor
            }
            DomainError::NotAllowedInState(reason) => AppError::not_allowed(reason),
            DomainError::Database(e) => AppError::from(e),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::Io(_) => {
                AppError::unavailable("database", e)
            }
            other => AppError::internal(other),
        }
    }
}

impl From<CryptoError> for AppError {
    fn from(e: CryptoError) -> Self {
        AppError::unavailable("crypto", e)
    }
}

impl From<AuditError> for AppError {
    fn from(e: AuditError) -> Self {
        match e {
            AuditError::Database(e) => AppError::from(e),
            other => AppError::internal(other),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::collections::HashSet;

    use http_body_util::BodyExt;

    use super::*;

    fn all() -> Vec<AppError> {
        vec![
            AppError::BadRequest("idempotency_key_malformed"),
            AppError::IdempotencyRequired,
            AppError::Unauthenticated,
            AppError::StepUpRequired,
            AppError::Scope,
            AppError::Forbidden,
            AppError::SelfApproval,
            AppError::NotFound,
            AppError::VersionConflict { current: 4 },
            AppError::IdempotencyInFlight,
            AppError::not_allowed("governed"),
            AppError::PayloadTooLarge,
            AppError::invalid("/value", "not_a_cron"),
            AppError::IdempotencyReuse,
            AppError::BelowFloor,
            AppError::PreconditionRequired,
            AppError::RateLimited {
                retry_after: Duration::from_secs(30),
            },
            AppError::Internal,
            AppError::Unavailable,
        ]
    }

    async fn body(response: Response) -> Value {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[test]
    fn the_catalogue_is_the_table_of_the_phase_file() {
        let table: Vec<(u16, &str)> = all()
            .iter()
            .map(|e| (e.status().as_u16(), e.code()))
            .collect();
        assert_eq!(
            table,
            vec![
                (400, "E_BAD_REQUEST"),
                (400, "E_IDEMPOTENCY_REQUIRED"),
                (401, "E_UNAUTHENTICATED"),
                (401, "E_STEPUP_REQUIRED"),
                (403, "E_SCOPE"),
                (403, "E_FORBIDDEN"),
                (403, "E_SELF_APPROVAL"),
                (404, "E_NOT_FOUND"),
                (409, "E_VERSION_CONFLICT"),
                (409, "E_IDEMPOTENCY_IN_FLIGHT"),
                (409, "E_NOT_ALLOWED_IN_STATE"),
                (413, "E_PAYLOAD_TOO_LARGE"),
                (422, "E_VALIDATION"),
                (422, "E_IDEMPOTENCY_REUSE"),
                (422, "E_BELOW_FLOOR"),
                (428, "E_PRECONDITION_REQUIRED"),
                (429, "E_RATE_LIMITED"),
                (500, "E_INTERNAL"),
                (503, "E_UNAVAILABLE"),
            ]
        );
        let codes: HashSet<_> = all().iter().map(AppError::code).collect();
        assert_eq!(codes.len(), 19);
    }

    #[tokio::test]
    async fn every_error_is_a_problem_document_with_a_correlation_id() {
        for error in all() {
            let code = error.code();
            let status = error.status();
            let response = error.into_response();
            assert_eq!(response.status(), status);
            assert_eq!(
                response.headers()[header::CONTENT_TYPE],
                PROBLEM_CONTENT_TYPE
            );
            assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
            let json = body(response).await;
            assert_eq!(json["code"], code);
            assert_eq!(json["status"], status.as_u16());
            assert!(json["title"].as_str().is_some_and(|t| !t.is_empty()));
            assert!(json["type"].as_str().unwrap().starts_with(TYPE_BASE));
            assert!(Uuid::try_parse(json["correlation_id"].as_str().unwrap()).is_ok());
            assert!(json.get("detail").is_none());
        }
    }

    #[tokio::test]
    async fn the_body_carries_the_id_of_the_running_request() {
        let id = CorrelationId::generate();
        let response = correlation::scope(id, async { AppError::NotFound.into_response() }).await;
        assert_eq!(body(response).await["correlation_id"], id.to_string());
    }

    #[tokio::test]
    async fn not_found_has_one_body_and_nothing_else() {
        let json = body(AppError::NotFound.into_response()).await;
        let mut keys: Vec<_> = json.as_object().unwrap().keys().cloned().collect();
        keys.sort();
        assert_eq!(keys, ["code", "correlation_id", "status", "title", "type"]);
        assert_eq!(json["title"], "Not Found");
        assert_eq!(json["type"], "https://akr-tms.local/errors/not-found");
    }

    #[tokio::test]
    async fn a_version_conflict_names_the_current_version() {
        let json = body(AppError::VersionConflict { current: 4 }.into_response()).await;
        assert_eq!(json["current_version"], 4);
    }

    #[tokio::test]
    async fn validation_failures_carry_pointer_code_and_params() {
        let error = AppError::Validation(vec![
            ValidationError::new("/value", "out_of_range").param("max", 200),
        ]);
        let json = body(error.into_response()).await;
        assert_eq!(json["errors"][0]["pointer"], "/value");
        assert_eq!(json["errors"][0]["code"], "out_of_range");
        assert_eq!(json["errors"][0]["params"]["max"], 200);
    }

    #[tokio::test]
    async fn a_reason_travels_in_errors_with_an_empty_pointer() {
        let json = body(AppError::not_allowed("governed").into_response()).await;
        assert_eq!(json["errors"][0]["pointer"], "");
        assert_eq!(json["errors"][0]["code"], "governed");
    }

    #[test]
    fn rate_limiting_sets_retry_after_in_whole_seconds() {
        let response = AppError::RateLimited {
            retry_after: Duration::from_millis(200),
        }
        .into_response();
        assert_eq!(response.headers()[header::RETRY_AFTER], "1");
        let response = AppError::RateLimited {
            retry_after: Duration::from_secs(45),
        }
        .into_response();
        assert_eq!(response.headers()[header::RETRY_AFTER], "45");
    }

    #[tokio::test]
    async fn an_internal_error_tells_the_client_nothing_about_the_cause() {
        let error = AppError::internal("relation admin.settings does not exist at 10.0.0.5");
        let text = String::from_utf8(
            error
                .into_response()
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .to_vec(),
        )
        .unwrap();
        assert!(!text.contains("admin.settings"));
        assert!(!text.contains("10.0.0.5"));
    }

    #[test]
    fn domain_failures_map_to_their_codes() {
        assert_eq!(AppError::from(DomainError::NotFound), AppError::NotFound);
        assert_eq!(
            AppError::from(DomainError::VersionConflict { current: 9 }),
            AppError::VersionConflict { current: 9 }
        );
        assert_eq!(
            AppError::from(DomainError::BelowFloor {
                key: "ops.backup_max_age_h".into()
            }),
            AppError::BelowFloor
        );
        assert_eq!(
            AppError::from(DomainError::NotAllowedInState("governed")),
            AppError::not_allowed("governed")
        );
        let invalid = DomainError::Validation(vec![FieldViolation::new("/value", "not_a_cron")]);
        assert_eq!(
            AppError::from(invalid),
            AppError::invalid("/value", "not_a_cron")
        );
        assert_eq!(
            AppError::from(DomainError::Database(sqlx::Error::RowNotFound)),
            AppError::Internal
        );
        assert_eq!(
            AppError::from(DomainError::Database(sqlx::Error::PoolTimedOut)),
            AppError::Unavailable
        );
    }

    #[test]
    fn a_crypto_failure_is_unavailable() {
        assert_eq!(AppError::from(CryptoError::Rejected), AppError::Unavailable);
        assert_eq!(
            AppError::from(CryptoError::Unavailable("sealed".into())),
            AppError::Unavailable
        );
    }
}
