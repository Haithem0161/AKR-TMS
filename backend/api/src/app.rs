//! Assembles the application: the routes, the request pipeline and the OpenAPI document.

use std::any::Any;
use std::sync::{Arc, OnceLock};

use akr_common::AppError;
use akr_common::errors::{Problem, ProblemResponse, ValidationError};
use axum::Router;
use axum::body::Body;
use axum::extract::{DefaultBodyLimit, MatchedPath};
use axum::middleware::{from_fn, from_fn_with_state};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use bytes::Bytes;
use http::{HeaderName, Request, header};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tower_http::trace::TraceLayer;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::middleware;
use crate::routes;
use crate::state::AppState;

pub const API_PREFIX: &str = "/api/v1";
pub const OPENAPI_PATH: &str = "/api/v1/openapi.json";
pub const SWAGGER_UI_PATH: &str = "/swagger-ui";

pub const SESSION_COOKIE: &str = "akr_session";
pub const STEPUP_HEADER: &str = "x-stepup-assertion";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "AKR-TMS API",
        description = "Transaction monitoring, screening and case management for one bank."
    ),
    servers((url = "/", description = "This installation")),
    components(schemas(Problem, ValidationError), responses(ProblemResponse)),
    modifiers(&SecuritySchemes),
    tags((name = "infra", description = "Liveness and readiness"))
)]
pub struct ApiDoc;

/// Declares the three ways a request proves who sends it. Later phases only reference them.
struct SecuritySchemes;

impl Modify for SecuritySchemes {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .get_or_insert_with(utoipa::openapi::Components::default);
        components.add_security_scheme(
            "session_cookie",
            SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::with_description(
                SESSION_COOKIE,
                "The browser session. An unsafe request also needs the X-CSRF-Token header.",
            ))),
        );
        components.add_security_scheme(
            "bearer_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("A client-credentials token of an integration."))
                    .build(),
            ),
        );
        components.add_security_scheme(
            "stepup_assertion",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                "X-StepUp-Assertion",
                "A single-use proof of a fresh second factor, for the operations marked step-up.",
            ))),
        );
    }
}

/// The routes of `/api/v1`. A later phase merges its router here.
fn v1_routes(extra: OpenApiRouter<AppState>) -> OpenApiRouter<AppState> {
    OpenApiRouter::new().merge(extra)
}

fn tree(v1: OpenApiRouter<AppState>) -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(routes::infra::router())
        .nest(API_PREFIX, v1)
}

/// The document of the production surface: what a client is generated from.
pub fn openapi() -> utoipa::openapi::OpenApi {
    tree(v1_routes(OpenApiRouter::new())).into_openapi()
}

/// The application as the binary serves it.
pub fn build_app(state: AppState) -> Router {
    let development = if state.config.dev_routes() {
        routes::echo::router()
    } else {
        OpenApiRouter::new()
    };
    build_app_with(state, development)
}

/// The application with `extra` mounted under `/api/v1`, behind the whole pipeline.
pub fn build_app_with(state: AppState, extra: OpenApiRouter<AppState>) -> Router {
    let document = Arc::new(OnceLock::<Bytes>::new());

    // A layer wraps the routes that exist when it is added, and the last one added runs first: the auth
    // slot, then idempotency, then the handler. The document route is added after them, so it needs
    // neither a principal nor a key. A path that matches no route never reaches these layers: it is
    // answered by the fallback below.
    let v1 = v1_routes(extra)
        .layer(from_fn_with_state(
            state.clone(),
            middleware::idempotency::layer,
        ))
        .layer(from_fn_with_state(
            state.auth.clone(),
            middleware::principal::layer,
        ))
        .route(
            "/openapi.json",
            get({
                let document = document.clone();
                move || serve_document(document.clone())
            }),
        );

    let (router, api) = tree(v1).split_for_parts();
    match serde_json::to_vec(&api) {
        Ok(json) => {
            let _ = document.set(Bytes::from(json));
        }
        Err(e) => tracing::error!(error = %e, "the OpenAPI document cannot be serialised"),
    }

    let router = if state.config.dev_routes() {
        router.merge(
            SwaggerUi::new(SWAGGER_UI_PATH).config(utoipa_swagger_ui::Config::from(OPENAPI_PATH)),
        )
    } else {
        router
    };

    // The last layer added is the outermost: the body limit, then correlation, the trace span and the
    // metrics. The panic guard is innermost, so a handler that panics is still counted and traced.
    router
        .method_not_allowed_fallback(method_not_allowed)
        .fallback(not_found)
        .layer(CatchPanicLayer::custom(panicked))
        .layer(from_fn(middleware::metrics::layer))
        .layer(TraceLayer::new_for_http().make_span_with(route_span))
        .layer(from_fn(middleware::correlation::layer))
        .layer(SetSensitiveRequestHeadersLayer::new([
            header::AUTHORIZATION,
            header::COOKIE,
            HeaderName::from_static(STEPUP_HEADER),
        ]))
        .layer(DefaultBodyLimit::max(state.config.max_body_bytes))
        .with_state(state)
}

async fn serve_document(document: Arc<OnceLock<Bytes>>) -> Response {
    match document.get() {
        Some(json) => ([(header::CONTENT_TYPE, "application/json")], json.clone()).into_response(),
        None => AppError::Internal.into_response(),
    }
}

/// Names the span by the route template, so a path with an id in it never reaches the traces.
fn route_span(req: &Request<Body>) -> tracing::Span {
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map_or(middleware::metrics::UNMATCHED, MatchedPath::as_str);
    tracing::info_span!("http", method = %req.method(), route)
}

async fn not_found() -> Response {
    AppError::NotFound.into_response()
}

async fn method_not_allowed() -> Response {
    AppError::BadRequest("method_not_allowed").into_response()
}

fn panicked(_cause: Box<dyn Any + Send + 'static>) -> Response {
    tracing::error!("a handler panicked");
    AppError::Internal.into_response()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn the_document_is_the_shell_the_plan_names() {
        let document = serde_json::to_value(openapi()).unwrap();
        assert_eq!(document["info"]["title"], "AKR-TMS API");
        assert_eq!(document["info"]["version"], env!("CARGO_PKG_VERSION"));
        assert_eq!(document["tags"][0]["name"], "infra");

        let schemas = &document["components"]["schemas"];
        for name in ["Problem", "ValidationError", "Health", "Readiness"] {
            assert!(schemas.get(name).is_some(), "schema {name} is missing");
        }
        let schemes = &document["components"]["securitySchemes"];
        assert_eq!(schemes["session_cookie"]["in"], "cookie");
        assert_eq!(schemes["session_cookie"]["name"], SESSION_COOKIE);
        assert_eq!(schemes["bearer_token"]["scheme"], "bearer");
        assert_eq!(schemes["stepup_assertion"]["in"], "header");
        assert_eq!(schemes["stepup_assertion"]["name"], "X-StepUp-Assertion");
    }

    #[test]
    fn the_production_document_has_the_probes_and_no_development_route() {
        let document = serde_json::to_value(openapi()).unwrap();
        let paths = document["paths"].as_object().unwrap();
        assert!(paths.contains_key("/healthz"));
        assert!(paths.contains_key("/readyz"));
        assert!(!paths.keys().any(|path| path.contains("_echo")));
        assert!(!paths.contains_key("/metrics"));
    }

    #[test]
    fn the_development_route_is_documented_under_the_prefix() {
        let document =
            serde_json::to_value(tree(v1_routes(routes::echo::router())).into_openapi()).unwrap();
        let echo = &document["paths"]["/api/v1/_echo"]["post"];
        assert_eq!(echo["tags"][0], "infra");
        assert!(echo["responses"].get("422").is_some());
    }
}
