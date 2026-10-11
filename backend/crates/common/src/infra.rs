//! The infrastructure routes every binary serves: liveness and readiness on its public listener, and the
//! Prometheus exposition on the management listener, which the edge never routes.
//!
//! A readiness body holds names and booleans only. Why a check failed goes to the log, never to the caller.

use std::future::Future;
use std::net::SocketAddr;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use utoipa::ToSchema;

use crate::config::{Binary, public_bind};

/// How long one readiness check may take. A dependency that hangs is a dependency that is not ready.
pub const READINESS_TIMEOUT: Duration = Duration::from_secs(2);

const STATUS_OK: &str = "ok";
const STATUS_READY: &str = "ready";
const STATUS_NOT_READY: &str = "not_ready";

/// The Prometheus text exposition format.
const METRICS_CONTENT_TYPE: &str = "text/plain; version=0.0.4; charset=utf-8";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Health {
    #[schema(example = "ok")]
    pub status: String,
}

impl Health {
    pub fn ok() -> Self {
        Self {
            status: STATUS_OK.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ReadinessCheck {
    #[schema(example = "database")]
    pub name: String,
    pub ok: bool,
}

impl ReadinessCheck {
    pub fn new(name: &str, ok: bool) -> Self {
        Self {
            name: name.to_owned(),
            ok,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Readiness {
    /// `ready` when every check passed, else `not_ready`.
    #[schema(example = "ready")]
    pub status: String,
    pub checks: Vec<ReadinessCheck>,
}

impl Readiness {
    pub fn from_checks(checks: Vec<ReadinessCheck>) -> Self {
        let ready = checks.iter().all(|check| check.ok);
        Self {
            status: if ready {
                STATUS_READY
            } else {
                STATUS_NOT_READY
            }
            .to_owned(),
            checks,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.status == STATUS_READY
    }

    pub fn status_code(&self) -> StatusCode {
        if self.is_ready() {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}

/// The dependencies a binary needs before it takes work, in the order its readiness body lists them.
#[async_trait]
pub trait ReadinessProbe: Send + Sync + 'static {
    async fn checks(&self) -> Vec<ReadinessCheck>;
}

/// Runs one check under [`READINESS_TIMEOUT`] and logs why it failed.
pub async fn check<F, E>(name: &str, probe: F) -> ReadinessCheck
where
    F: Future<Output = Result<(), E>>,
    E: std::fmt::Display,
{
    let ok = match tokio::time::timeout(READINESS_TIMEOUT, probe).await {
        Ok(Ok(())) => true,
        Ok(Err(e)) => {
            tracing::warn!(check = name, error = %e, "readiness check failed");
            false
        }
        Err(_) => {
            tracing::warn!(check = name, "readiness check timed out");
            false
        }
    };
    ReadinessCheck::new(name, ok)
}

pub async fn healthz() -> Json<Health> {
    Json(Health::ok())
}

pub async fn readyz(probe: &dyn ReadinessProbe) -> (StatusCode, Json<Readiness>) {
    let readiness = Readiness::from_checks(probe.checks().await);
    (readiness.status_code(), Json(readiness))
}

/// `GET /healthz` and `GET /readyz`, for a binary with no API of its own.
pub fn router(probe: Arc<dyn ReadinessProbe>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route(
            "/readyz",
            get(|State(probe): State<Arc<dyn ReadinessProbe>>| async move {
                readyz(probe.as_ref()).await
            }),
        )
        .with_state(probe)
}

/// The argument that makes a binary the health probe of its own container: an image with no shell has
/// nothing else to ask with.
pub const HEALTHCHECK_COMMAND: &str = "healthcheck";

/// One limit for the connect, the request and the answer.
const HEALTHCHECK_TIMEOUT: Duration = Duration::from_secs(3);

/// Asks the process that listens on `bind` for `/readyz`, over loopback when it listens on every
/// address. True when the answer is 200.
pub fn probe_ready(bind: SocketAddr) -> std::io::Result<bool> {
    use std::io::{Read, Write};
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpStream};

    const READY: &[u8] = b"HTTP/1.1 200";

    let ip = match bind.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(ip) if ip.is_unspecified() => IpAddr::V6(Ipv6Addr::LOCALHOST),
        ip => ip,
    };
    let mut stream =
        TcpStream::connect_timeout(&SocketAddr::new(ip, bind.port()), HEALTHCHECK_TIMEOUT)?;
    stream.set_read_timeout(Some(HEALTHCHECK_TIMEOUT))?;
    stream.set_write_timeout(Some(HEALTHCHECK_TIMEOUT))?;
    stream.write_all(b"GET /readyz HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")?;
    let mut status = [0u8; READY.len()];
    stream.read_exact(&mut status)?;
    Ok(status == READY)
}

/// `<binary> healthcheck`: exit 0 when the process of this container is ready, 1 when it is not or does
/// not answer.
pub fn healthcheck(binary: Binary) -> ExitCode {
    let bind = match public_bind(binary, std::env::var("AKR_BIND").ok().as_deref()) {
        Ok(bind) => bind,
        Err(e) => {
            eprintln!("{}: {e}", binary.name());
            return ExitCode::FAILURE;
        }
    };
    match probe_ready(bind) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => {
            eprintln!("{}: not ready", binary.name());
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("{}: no answer on {bind}: {e}", binary.name());
            ExitCode::FAILURE
        }
    }
}

/// Renders the metrics exposition; a binary passes its Prometheus handle.
pub type MetricsRender = Arc<dyn Fn() -> String + Send + Sync>;

/// `GET /metrics`, for the management listener only.
pub fn management_router(render: MetricsRender) -> Router {
    Router::new()
        .route(
            "/metrics",
            get(|State(render): State<MetricsRender>| async move {
                ([(header::CONTENT_TYPE, METRICS_CONTENT_TYPE)], render()).into_response()
            }),
        )
        .with_state(render)
}

/// Serves `router` until `shutdown` resolves, then lets the requests in flight finish. A handler may ask
/// for the peer's address (`ConnectInfo<SocketAddr>`).
pub async fn serve<F>(listener: TcpListener, router: Router, shutdown: F) -> std::io::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown)
    .await
}

/// Resolves on SIGTERM or Ctrl-C: the moment a binary stops taking work.
pub async fn shutdown_signal() {
    let interrupt = async {
        if let Err(e) = tokio::signal::ctrl_c().await {
            tracing::error!(error = %e, "cannot listen for Ctrl-C");
            std::future::pending::<()>().await;
        }
    };
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
            }
            Err(e) => {
                tracing::error!(error = %e, "cannot listen for SIGTERM");
                std::future::pending::<()>().await;
            }
        }
    };
    tokio::select! {
        () = interrupt => {}
        () = terminate => {}
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    struct Fixed(Vec<(&'static str, bool)>);

    #[async_trait]
    impl ReadinessProbe for Fixed {
        async fn checks(&self) -> Vec<ReadinessCheck> {
            self.0
                .iter()
                .map(|(name, ok)| ReadinessCheck::new(name, *ok))
                .collect()
        }
    }

    async fn get_path(router: Router, path: &str) -> (StatusCode, String, Option<String>) {
        let response = router
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .map(|v| v.to_str().unwrap().to_owned());
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (
            status,
            String::from_utf8(body.to_vec()).unwrap(),
            content_type,
        )
    }

    #[tokio::test]
    async fn liveness_says_ok() {
        let (status, body, _) = get_path(router(Arc::new(Fixed(vec![]))), "/healthz").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#"{"status":"ok"}"#);
    }

    #[tokio::test]
    async fn readiness_lists_names_and_booleans_in_order() {
        let probe = Fixed(vec![
            ("database", true),
            ("migrations", true),
            ("valkey", true),
        ]);
        let (status, body, _) = get_path(router(Arc::new(probe)), "/readyz").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            body,
            r#"{"status":"ready","checks":[{"name":"database","ok":true},{"name":"migrations","ok":true},{"name":"valkey","ok":true}]}"#
        );
    }

    #[tokio::test]
    async fn one_failing_check_makes_the_binary_unready() {
        let probe = Fixed(vec![("database", true), ("valkey", false)]);
        let (status, body, _) = get_path(router(Arc::new(probe)), "/readyz").await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            body,
            r#"{"status":"not_ready","checks":[{"name":"database","ok":true},{"name":"valkey","ok":false}]}"#
        );
    }

    async fn probed(checks: Vec<(&'static str, bool)>) -> std::io::Result<bool> {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
        let served = tokio::spawn(serve(listener, router(Arc::new(Fixed(checks))), async {
            stopped.await.ok();
        }));
        // The binary listens on every address; the probe asks over loopback.
        let answer = tokio::task::spawn_blocking(move || {
            probe_ready(SocketAddr::from(([0, 0, 0, 0], port)))
        })
        .await
        .unwrap();
        stop.send(()).unwrap();
        served.await.unwrap().unwrap();
        answer
    }

    #[tokio::test]
    async fn the_container_probe_reads_readiness() {
        assert!(probed(vec![("database", true)]).await.unwrap());
        assert!(!probed(vec![("database", false)]).await.unwrap());
    }

    #[tokio::test]
    async fn the_container_probe_fails_when_nothing_listens() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let free = listener.local_addr().unwrap();
        drop(listener);
        let answer = tokio::task::spawn_blocking(move || probe_ready(free))
            .await
            .unwrap();
        assert!(answer.is_err());
    }

    #[tokio::test]
    async fn the_public_router_has_no_metrics_and_the_management_router_has_nothing_else() {
        let (status, _, _) = get_path(router(Arc::new(Fixed(vec![]))), "/metrics").await;
        assert_eq!(status, StatusCode::NOT_FOUND);

        let management = management_router(Arc::new(|| "akr_up 1\n".to_owned()));
        let (status, body, content_type) = get_path(management.clone(), "/metrics").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "akr_up 1\n");
        assert_eq!(content_type.as_deref(), Some(METRICS_CONTENT_TYPE));
        let (status, _, _) = get_path(management, "/healthz").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test(start_paused = true)]
    async fn a_check_that_hangs_or_fails_is_not_ok() {
        let hung = check("store", std::future::pending::<Result<(), String>>()).await;
        assert_eq!(hung, ReadinessCheck::new("store", false));
        let failed = check("store", async { Err::<(), _>("refused") }).await;
        assert!(!failed.ok);
        let passed = check("store", async { Ok::<(), String>(()) }).await;
        assert!(passed.ok);
    }
}
