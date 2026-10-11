//! akr-s3guard: the allow-list proxy in front of the object store (TS-22).
//!
//! It passes the S3 operations the product uses and refuses every other request with 403, before the
//! store reads it. It checks no signature: the store does. What it forwards, it forwards as it arrived
//! (path, query and `Host` are what the signature covers), in one part of known length.

pub mod cidr;
pub mod rules;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use akr_common::infra::{self, ReadinessCheck, ReadinessProbe};
use akr_objectstore::client::GUARD_RULE_HEADER;
use akr_telemetry::metrics::S3GUARD_REJECTED_TOTAL;
use async_trait::async_trait;
use axum::Router;
use axum::body::Body;
use axum::extract::{ConnectInfo, Request, State};
use axum::response::{IntoResponse, Response};
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode, header};
use url::Url;

use crate::cidr::Cidr;
use crate::rules::{RequestView, Rule, RuleBookError, evaluate};

pub const UPSTREAM_VAR: &str = "AKR_S3_UPSTREAM";
pub const INSTALLER_CIDRS_VAR: &str = "AKR_S3GUARD_INSTALLER_CIDRS";
pub const UPSTREAM_HEALTH_VAR: &str = "AKR_S3_UPSTREAM_HEALTH";

const DEFAULT_HEALTH_PATH: &str = "/minio/health/live";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
/// The longest the store may stay silent in the middle of an answer.
const READ_TIMEOUT: Duration = Duration::from_secs(60);

const REFUSED_BODY: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
    <Error><Code>AccessDenied</Code><Message>Access Denied.</Message></Error>";
const UNAVAILABLE_BODY: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
    <Error><Code>ServiceUnavailable</Code><Message>The object store did not answer.</Message></Error>";

/// Headers that belong to one connection, and headers by which a client could speak for the proxy.
const NOT_FORWARDED: [&str; 14] = [
    "connection",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "te",
    "trailer",
    "transfer-encoding",
    "upgrade",
    "expect",
    "content-length",
    "forwarded",
    "x-forwarded-for",
    "x-forwarded-host",
    "x-forwarded-proto",
];
const REAL_IP: &str = "x-real-ip";
const FORWARDED_FOR: &str = "x-forwarded-for";

#[derive(Debug, thiserror::Error)]
pub enum GuardError {
    #[error("{var} is required")]
    Missing { var: &'static str },
    #[error("{var} is not valid: {reason}")]
    Invalid { var: &'static str, reason: String },
    #[error(transparent)]
    Rules(#[from] RuleBookError),
    #[error("the HTTP client cannot be built: {0}")]
    Client(String),
}

#[derive(Debug, Clone)]
pub struct GuardConfig {
    /// The store: scheme, host and port.
    pub upstream: Url,
    /// Where the installer runs. Empty: nobody creates a bucket through the guard.
    pub installer_cidrs: Vec<Cidr>,
    /// A path of the store that answers 2xx when it is up. The guard's own probe: a client cannot ask
    /// for it.
    pub health_path: String,
}

impl GuardConfig {
    pub fn from_env() -> Result<Self, GuardError> {
        Self::from_map(&std::env::vars().collect())
    }

    pub fn from_map(vars: &HashMap<String, String>) -> Result<Self, GuardError> {
        let get = |name: &str| vars.get(name).map(|v| v.trim()).filter(|v| !v.is_empty());
        let invalid = |var: &'static str, reason: String| GuardError::Invalid { var, reason };

        let upstream = get(UPSTREAM_VAR).ok_or(GuardError::Missing { var: UPSTREAM_VAR })?;
        let upstream: Url = upstream
            .parse()
            .map_err(|e: url::ParseError| invalid(UPSTREAM_VAR, e.to_string()))?;
        if !matches!(upstream.scheme(), "http" | "https")
            || upstream.host_str().is_none()
            || !matches!(upstream.path(), "" | "/")
            || upstream.query().is_some()
        {
            return Err(invalid(
                UPSTREAM_VAR,
                "expected http(s)://host:port with no path".to_owned(),
            ));
        }

        let installer_cidrs = cidr::parse_list(get(INSTALLER_CIDRS_VAR).unwrap_or_default())
            .map_err(|reason| invalid(INSTALLER_CIDRS_VAR, reason))?;

        let health_path = get(UPSTREAM_HEALTH_VAR).unwrap_or(DEFAULT_HEALTH_PATH);
        if !health_path.starts_with('/') {
            return Err(invalid(
                UPSTREAM_HEALTH_VAR,
                "a path starts with `/`".to_owned(),
            ));
        }

        Ok(Self {
            upstream,
            installer_cidrs,
            health_path: health_path.to_owned(),
        })
    }
}

enum Failure {
    Refused(Rule),
    Upstream(String),
}

impl From<Rule> for Failure {
    fn from(rule: Rule) -> Self {
        Failure::Refused(rule)
    }
}

pub struct Guard {
    /// `scheme://host[:port]` of the store.
    origin: String,
    health_url: String,
    installer_cidrs: Vec<Cidr>,
    http: reqwest::Client,
}

impl Guard {
    pub fn new(config: GuardConfig) -> Result<Self, GuardError> {
        let host = config.upstream.host_str().ok_or(GuardError::Invalid {
            var: UPSTREAM_VAR,
            reason: "no host".to_owned(),
        })?;
        let origin = match config.upstream.port() {
            Some(port) => format!("{}://{host}:{port}", config.upstream.scheme()),
            None => format!("{}://{host}", config.upstream.scheme()),
        };
        let http = reqwest::Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .read_timeout(READ_TIMEOUT)
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .http1_only()
            .build()
            .map_err(|e| GuardError::Client(e.to_string()))?;
        Ok(Self {
            health_url: format!("{origin}{}", config.health_path),
            origin,
            installer_cidrs: config.installer_cidrs,
            http,
        })
    }

    /// The S3 listener: the proxy, with the guard's own `/healthz` and `/readyz` in front of it.
    pub fn router(self: Arc<Self>) -> Router {
        let probe: Arc<dyn ReadinessProbe> = self.clone();
        Router::new()
            .fallback(proxy)
            .with_state(self)
            .merge(infra::router(probe))
    }

    async fn forward(&self, peer: SocketAddr, request: Request) -> Result<Response, Failure> {
        let (parts, body) = request.into_parts();
        if parts.uri.scheme().is_some() || parts.uri.authority().is_some() {
            return Err(Rule::OperationNotAllowed.into());
        }
        let raw_path = parts.uri.path();
        let raw_query = parts.uri.query();
        let operation = evaluate(&RequestView {
            method: &parts.method,
            raw_path,
            raw_query,
            headers: &parts.headers,
            from_installer: self
                .installer_cidrs
                .iter()
                .any(|block| block.contains(peer.ip())),
        })?;

        if !connection_is_plain(&parts.headers) || !parts.headers.contains_key(header::HOST) {
            return Err(Rule::OperationNotAllowed.into());
        }

        let target = match raw_query {
            Some(query) => format!("{}{raw_path}?{query}", self.origin),
            None => format!("{}{raw_path}", self.origin),
        };
        let url = Url::parse(&target).map_err(|_| Rule::OperationNotAllowed)?;
        if url.path() != raw_path || url.query() != raw_query {
            return Err(Rule::OperationNotAllowed.into());
        }

        let mut headers = HeaderMap::with_capacity(parts.headers.len() + 1);
        for (name, value) in &parts.headers {
            if !NOT_FORWARDED.contains(&name.as_str()) && name.as_str() != REAL_IP {
                headers.append(name.clone(), value.clone());
            }
        }
        if let Ok(peer_ip) = HeaderValue::from_str(&peer.ip().to_string()) {
            headers.insert(HeaderName::from_static(FORWARDED_FOR), peer_ip);
        }

        let length = content_length(&parts.headers);
        let mut upstream = self.http.request(parts.method.clone(), url);
        if operation.has_body() {
            // No Content-Length and no Transfer-Encoding is a request with no body.
            headers.insert(
                header::CONTENT_LENGTH,
                HeaderValue::from(length.unwrap_or(0)),
            );
            upstream = upstream.body(reqwest::Body::wrap_stream(body.into_data_stream()));
        } else if length.is_some_and(|length| length > 0) {
            return Err(Rule::OperationNotAllowed.into());
        }

        let answer = upstream
            .headers(headers)
            .send()
            .await
            .map_err(|e| Failure::Upstream(e.without_url().to_string()))?;

        let mut response = Response::builder().status(answer.status());
        if let Some(out) = response.headers_mut() {
            for (name, value) in answer.headers() {
                if !NOT_FORWARDED[..8].contains(&name.as_str()) {
                    out.append(name.clone(), value.clone());
                }
            }
        }
        response
            .body(Body::from_stream(answer.bytes_stream()))
            .map_err(|e| Failure::Upstream(e.to_string()))
    }
}

/// `Connection` may say `keep-alive` or `close`. Anything else names headers the proxy would have to
/// drop after it has judged the request with them.
fn connection_is_plain(headers: &HeaderMap) -> bool {
    !headers.contains_key(header::UPGRADE)
        && headers.get_all(header::CONNECTION).iter().all(|value| {
            value.to_str().is_ok_and(|text| {
                text.split(',').map(str::trim).all(|token| {
                    token.eq_ignore_ascii_case("keep-alive") || token.eq_ignore_ascii_case("close")
                })
            })
        })
}

fn content_length(headers: &HeaderMap) -> Option<u64> {
    headers
        .get(header::CONTENT_LENGTH)?
        .to_str()
        .ok()?
        .trim()
        .parse()
        .ok()
}

async fn proxy(
    State(guard): State<Arc<Guard>>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    request: Request,
) -> Response {
    let method = request.method().clone();
    match guard.forward(peer, request).await {
        Ok(response) => response,
        Err(Failure::Refused(rule)) => {
            metrics::counter!(S3GUARD_REJECTED_TOTAL, "rule" => rule.id()).increment(1);
            tracing::warn!(rule = rule.id(), method = %method, peer = %peer.ip(), "request refused");
            (
                StatusCode::FORBIDDEN,
                [
                    (header::CONTENT_TYPE.as_str(), "application/xml"),
                    (GUARD_RULE_HEADER, rule.id()),
                ],
                REFUSED_BODY,
            )
                .into_response()
        }
        Err(Failure::Upstream(reason)) => {
            tracing::error!(method = %method, reason, "the object store did not answer");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                [(header::CONTENT_TYPE, "application/xml")],
                UNAVAILABLE_BODY,
            )
                .into_response()
        }
    }
}

/// Registers one series per rule at zero, so a rule that has never fired is a `0` and not a gap.
pub fn register_metrics() {
    metrics::describe_counter!(
        S3GUARD_REJECTED_TOTAL,
        "Requests the S3 guard refused, by rule"
    );
    for rule in Rule::ALL {
        metrics::counter!(S3GUARD_REJECTED_TOTAL, "rule" => rule.id()).increment(0);
    }
}

#[async_trait]
impl ReadinessProbe for Guard {
    async fn checks(&self) -> Vec<ReadinessCheck> {
        vec![
            infra::check("upstream", async {
                let answer = self
                    .http
                    .get(&self.health_url)
                    .send()
                    .await
                    .map_err(|e| e.without_url().to_string())?;
                if answer.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("the store answered {}", answer.status()))
                }
            })
            .await,
        ]
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect()
    }

    #[test]
    fn the_upstream_is_required_and_is_an_origin() {
        assert!(matches!(
            GuardConfig::from_map(&vars(&[])),
            Err(GuardError::Missing { var: UPSTREAM_VAR })
        ));
        for bad in [
            "objectstore:9000",
            "http://objectstore:9000/data",
            "ftp://objectstore",
        ] {
            assert!(
                matches!(
                    GuardConfig::from_map(&vars(&[(UPSTREAM_VAR, bad)])),
                    Err(GuardError::Invalid {
                        var: UPSTREAM_VAR,
                        ..
                    })
                ),
                "{bad}"
            );
        }
        let config =
            GuardConfig::from_map(&vars(&[(UPSTREAM_VAR, "http://objectstore:9000")])).unwrap();
        assert!(config.installer_cidrs.is_empty());
        assert_eq!(config.health_path, "/minio/health/live");
        let guard = Guard::new(config).unwrap();
        assert_eq!(guard.origin, "http://objectstore:9000");
        assert_eq!(
            guard.health_url,
            "http://objectstore:9000/minio/health/live"
        );
    }

    #[test]
    fn the_installer_blocks_and_the_probe_path_are_checked() {
        let config = GuardConfig::from_map(&vars(&[
            (UPSTREAM_VAR, "http://objectstore:9000"),
            (INSTALLER_CIDRS_VAR, "10.200.20.0/28, 127.0.0.1"),
            (UPSTREAM_HEALTH_VAR, "/health"),
        ]))
        .unwrap();
        assert_eq!(config.installer_cidrs.len(), 2);
        assert_eq!(config.health_path, "/health");

        assert!(matches!(
            GuardConfig::from_map(&vars(&[
                (UPSTREAM_VAR, "http://objectstore:9000"),
                (INSTALLER_CIDRS_VAR, "everyone"),
            ])),
            Err(GuardError::Invalid {
                var: INSTALLER_CIDRS_VAR,
                ..
            })
        ));
        assert!(matches!(
            GuardConfig::from_map(&vars(&[
                (UPSTREAM_VAR, "http://objectstore:9000"),
                (UPSTREAM_HEALTH_VAR, "health"),
            ])),
            Err(GuardError::Invalid {
                var: UPSTREAM_HEALTH_VAR,
                ..
            })
        ));
    }

    #[test]
    fn a_connection_header_may_only_say_keep_alive_or_close() {
        let with = |value: &'static str| {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONNECTION, HeaderValue::from_static(value));
            headers
        };
        assert!(connection_is_plain(&HeaderMap::new()));
        assert!(connection_is_plain(&with("keep-alive")));
        assert!(connection_is_plain(&with("Close")));
        assert!(!connection_is_plain(&with("close, x-amz-content-sha256")));
        assert!(!connection_is_plain(&with("upgrade")));

        let mut upgrade = HeaderMap::new();
        upgrade.insert(header::UPGRADE, HeaderValue::from_static("h2c"));
        assert!(!connection_is_plain(&upgrade));
    }
}
