//! OpenBao over HTTP: AppRole login, KV v2 reads, transit and token calls (RQ-02: the calls of the DBS Vault
//! client, against OpenBao).
//!
//! One KV v2 mount, `akr/`; every KV path is relative to it (DR-356). The token travels in a header, never in a
//! query string, which would land in access logs.

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::{Value, json};
use zeroize::Zeroizing;

use crate::error::CryptoError;

pub const KV_MOUNT: &str = "akr";
pub const TRANSIT_MOUNT: &str = "transit";
pub const APPROLE_MOUNT: &str = "approle";
/// The transit key that wraps data keys.
pub const KEK: &str = "akr-kek";

const TOKEN_HEADER: &str = "X-Vault-Token";
const PEM_CERTIFICATE: &[u8] = b"-----BEGIN CERTIFICATE-----";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const RENEW_FLOOR: Duration = Duration::from_secs(5);
const RETRY_AFTER_FAILURE: Duration = Duration::from_secs(15);

/// The AppRole of one binary. The secret id is read from its file at every login, so an operator can replace
/// the file without a restart.
#[derive(Debug, Clone)]
pub struct AppRole {
    pub role_id: String,
    pub secret_id_file: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SealStatus {
    pub initialized: bool,
    pub sealed: bool,
}

/// A data key as transit returns it: wrapped by the KEK, never in the clear.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrappedKey {
    /// `vault:v<n>:<base64>`.
    pub ciphertext: String,
    pub key_version: u32,
}

#[derive(Clone)]
pub struct BaoClient {
    http: reqwest::Client,
    addr: String,
    token: Arc<RwLock<Zeroizing<String>>>,
}

impl std::fmt::Debug for BaoClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaoClient")
            .field("addr", &self.addr)
            .finish_non_exhaustive()
    }
}

impl BaoClient {
    /// `ca_cert_pem` is the CA the OpenBao listener's certificate chains to (`BAO_CACERT`); with it, nothing
    /// else is trusted.
    pub fn new(addr: &str, ca_cert_pem: Option<&[u8]>) -> Result<Self, CryptoError> {
        let mut builder = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .no_proxy();
        if let Some(pem) = ca_cert_pem {
            // reqwest reads the PEM lazily and would build a client that trusts nothing.
            if !pem
                .windows(PEM_CERTIFICATE.len())
                .any(|w| w == PEM_CERTIFICATE)
            {
                return Err(unavailable(
                    "the OpenBao CA file holds no certificate".into(),
                ));
            }
            let cert = reqwest::Certificate::from_pem(pem).map_err(|e| {
                unavailable(format!("the OpenBao CA certificate does not parse: {e}"))
            })?;
            builder = builder.tls_certs_only([cert]);
        }
        let http = builder
            .build()
            .map_err(|e| unavailable(format!("the OpenBao client cannot be built: {e}")))?;
        Ok(Self {
            http,
            addr: addr.trim_end_matches('/').to_owned(),
            token: Arc::new(RwLock::new(Zeroizing::new(String::new()))),
        })
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }

    /// Logs in with the AppRole and keeps the token. Returns the token's time to live.
    pub async fn login(&self, role: &AppRole) -> Result<Duration, CryptoError> {
        let secret_id = Zeroizing::new(
            tokio::fs::read_to_string(&role.secret_id_file)
                .await
                .map_err(|e| {
                    unavailable(format!("the AppRole secret id file cannot be read: {e}"))
                })?,
        );
        let body = json!({ "role_id": role.role_id, "secret_id": secret_id.trim() });
        let res = self
            .send(
                reqwest::Method::POST,
                &format!("auth/{APPROLE_MOUNT}/login"),
                Some(body),
                None,
            )
            .await?;
        let token = text_at(&res, "/auth/client_token")?;
        self.set_token(token);
        Ok(lease_at(&res))
    }

    /// Exchanges the login token for a token of `token_role` and revokes the login token, so the process
    /// keeps transit and renewal and loses its KV reads for the rest of the run.
    pub async fn narrow_to(&self, token_role: &str) -> Result<Duration, CryptoError> {
        let res = self
            .post(&format!("auth/token/create/{token_role}"), json!({}))
            .await?;
        let narrow = text_at(&res, "/auth/client_token")?;
        let login_token = self.token();
        self.set_token(narrow);
        if let Err(e) = self
            .send(
                reqwest::Method::POST,
                "auth/token/revoke-self",
                None,
                Some(login_token.as_str()),
            )
            .await
        {
            tracing::warn!(error = %e, "the OpenBao login token was not revoked; it expires by its own time to live");
        }
        Ok(lease_at(&res))
    }

    pub async fn renew_self(&self) -> Result<Duration, CryptoError> {
        let res = self.post("auth/token/renew-self", json!({})).await?;
        Ok(lease_at(&res))
    }

    /// The remaining life of the token this client holds. An error means it is not valid.
    pub async fn lookup_self(&self) -> Result<Duration, CryptoError> {
        let res = self.get("auth/token/lookup-self").await?;
        Ok(Duration::from_secs(
            res.pointer("/data/ttl")
                .and_then(Value::as_u64)
                .unwrap_or(0),
        ))
    }

    pub async fn seal_status(&self) -> Result<SealStatus, CryptoError> {
        let res = self
            .send(reqwest::Method::GET, "sys/seal-status", None, Some(""))
            .await?;
        Ok(SealStatus {
            initialized: res
                .get("initialized")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            sealed: res.get("sealed").and_then(Value::as_bool).unwrap_or(true),
        })
    }

    /// Renews the token at half of its life for as long as the process runs, and logs in again when a renewal
    /// is refused (the token reached its maximum life, or OpenBao was restored from a snapshot).
    pub async fn keep_alive(
        &self,
        role: AppRole,
        token_role: Option<String>,
        mut ttl: Duration,
    ) -> Infallible {
        loop {
            tokio::time::sleep((ttl / 2).max(RENEW_FLOOR)).await;
            ttl = match self.renew_self().await {
                Ok(ttl) => ttl,
                Err(renew_error) => match self.login_again(&role, token_role.as_deref()).await {
                    Ok(ttl) => {
                        tracing::warn!(error = %renew_error, "the OpenBao token was not renewed; logged in again");
                        ttl
                    }
                    Err(login_error) => {
                        tracing::error!(
                            renew_error = %renew_error,
                            login_error = %login_error,
                            "the OpenBao token is lost and login fails; retrying"
                        );
                        RETRY_AFTER_FAILURE * 2
                    }
                },
            };
        }
    }

    async fn login_again(
        &self,
        role: &AppRole,
        token_role: Option<&str>,
    ) -> Result<Duration, CryptoError> {
        let ttl = self.login(role).await?;
        match token_role {
            Some(token_role) => self.narrow_to(token_role).await,
            None => Ok(ttl),
        }
    }

    /// One KV v2 item's map. `path` is relative to the `akr/` mount (`db/akr_app`, `keys/cursor-hmac`).
    pub async fn kv_read(&self, path: &str) -> Result<Value, CryptoError> {
        let res = self.get(&format!("{KV_MOUNT}/data/{path}")).await?;
        res.pointer("/data/data")
            .cloned()
            .ok_or_else(|| unavailable(format!("KV item {path} has no data")))
    }

    /// A new 256-bit data key, wrapped by `kek`. The key itself is not in the answer.
    pub async fn datakey_wrapped(&self, kek: &str) -> Result<WrappedKey, CryptoError> {
        let res = self
            .post(
                &format!("{TRANSIT_MOUNT}/datakey/wrapped/{kek}"),
                json!({ "bits": 256 }),
            )
            .await?;
        Ok(WrappedKey {
            ciphertext: text_at(&res, "/data/ciphertext")?,
            key_version: res
                .pointer("/data/key_version")
                .and_then(Value::as_u64)
                .and_then(|v| u32::try_from(v).ok())
                .ok_or_else(|| unavailable("the data key answer has no key version".into()))?,
        })
    }

    /// Unwraps transit ciphertext. The plaintext exists only in the returned buffer, which wipes itself.
    pub async fn decrypt(
        &self,
        kek: &str,
        ciphertext: &str,
    ) -> Result<Zeroizing<Vec<u8>>, CryptoError> {
        let res = self
            .post(
                &format!("{TRANSIT_MOUNT}/decrypt/{kek}"),
                json!({ "ciphertext": ciphertext }),
            )
            .await?;
        let b64 = Zeroizing::new(text_at(&res, "/data/plaintext")?);
        // The decode error is not echoed: on a malformed answer its text could carry key material.
        BASE64
            .decode(b64.as_bytes())
            .map(Zeroizing::new)
            .map_err(|_| unavailable("the unwrapped key is not base64".into()))
    }

    /// The transit signature string, `vault:v<n>:<base64>`. No prehash algorithm is sent: Ed25519 signs the
    /// message itself.
    pub async fn sign(&self, key: &str, msg: &[u8]) -> Result<String, CryptoError> {
        let res = self
            .post(
                &format!("{TRANSIT_MOUNT}/sign/{key}"),
                json!({ "input": BASE64.encode(msg) }),
            )
            .await?;
        text_at(&res, "/data/signature")
    }

    pub async fn verify(
        &self,
        key: &str,
        msg: &[u8],
        signature: &str,
    ) -> Result<bool, CryptoError> {
        let body = json!({ "input": BASE64.encode(msg), "signature": signature });
        let res = self
            .post(&format!("{TRANSIT_MOUNT}/verify/{key}"), body)
            .await?;
        res.pointer("/data/valid")
            .and_then(Value::as_bool)
            .ok_or_else(|| unavailable("the verify answer has no verdict".into()))
    }

    /// Every public key version of a transit signing key. Rotation keeps the old versions, which is what lets
    /// a signature made before a rotation still verify offline.
    pub async fn public_keys(&self, key: &str) -> Result<BTreeMap<u32, [u8; 32]>, CryptoError> {
        let res = self.get(&format!("{TRANSIT_MOUNT}/keys/{key}")).await?;
        let versions = res
            .pointer("/data/keys")
            .and_then(Value::as_object)
            .ok_or_else(|| unavailable(format!("transit key {key} lists no versions")))?;
        let mut out = BTreeMap::new();
        for (version, entry) in versions {
            let public = entry
                .get("public_key")
                .and_then(Value::as_str)
                .and_then(|b64| BASE64.decode(b64).ok())
                .and_then(|raw| <[u8; 32]>::try_from(raw).ok());
            if let (Ok(version), Some(public)) = (version.parse::<u32>(), public) {
                out.insert(version, public);
            }
        }
        if out.is_empty() {
            return Err(unavailable(format!(
                "transit key {key} has no usable public key"
            )));
        }
        Ok(out)
    }

    async fn get(&self, path: &str) -> Result<Value, CryptoError> {
        self.send(reqwest::Method::GET, path, None, None).await
    }

    async fn post(&self, path: &str, body: Value) -> Result<Value, CryptoError> {
        self.send(reqwest::Method::POST, path, Some(body), None)
            .await
    }

    /// `token`: `None` uses the client's token; `Some("")` sends none.
    async fn send(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<Value>,
        token: Option<&str>,
    ) -> Result<Value, CryptoError> {
        let own = self.token();
        let token = token.unwrap_or(own.as_str());
        let mut request = self
            .http
            .request(method, format!("{}/v1/{path}", self.addr));
        if !token.is_empty() {
            request = request.header(TOKEN_HEADER, token);
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        let response = request
            .send()
            .await
            .map_err(|e| unavailable(format!("OpenBao {path}: {}", transport(&e))))?;
        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|e| unavailable(format!("OpenBao {path}: {}", transport(&e))))?;
        if !status.is_success() {
            // An OpenBao error body names the policy or key at fault and never holds key material.
            let detail: String = text.chars().take(300).collect();
            return Err(unavailable(format!(
                "OpenBao {path} answered {status}: {}",
                detail.trim()
            )));
        }
        if text.is_empty() {
            return Ok(Value::Null);
        }
        serde_json::from_str(&text)
            .map_err(|_| unavailable(format!("OpenBao {path} did not answer JSON")))
    }

    fn token(&self) -> Zeroizing<String> {
        self.token
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    fn set_token(&self, token: String) {
        *self
            .token
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Zeroizing::new(token);
    }
}

fn unavailable(message: String) -> CryptoError {
    CryptoError::Unavailable(message)
}

/// The cause without the URL: a URL in an error ends up in logs.
fn transport(error: &reqwest::Error) -> &'static str {
    if error.is_timeout() {
        "timed out"
    } else if error.is_connect() {
        "connection failed"
    } else {
        "request failed"
    }
}

fn text_at(value: &Value, pointer: &str) -> Result<String, CryptoError> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| unavailable(format!("the OpenBao answer has no {pointer}")))
}

fn lease_at(value: &Value) -> Duration {
    Duration::from_secs(
        value
            .pointer("/auth/lease_duration")
            .and_then(Value::as_u64)
            .unwrap_or(0),
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn the_address_is_normalised_so_a_trailing_slash_cannot_double_up() {
        let bao = BaoClient::new("https://openbao:8200/", None).unwrap();
        assert_eq!(bao.addr(), "https://openbao:8200");
    }

    #[test]
    fn debug_output_never_shows_the_token() {
        let bao = BaoClient::new("https://openbao:8200", None).unwrap();
        bao.set_token("s.very-secret".into());
        assert!(!format!("{bao:?}").contains("very-secret"));
    }

    #[test]
    fn a_ca_that_does_not_parse_is_refused() {
        assert!(BaoClient::new("https://openbao:8200", Some(b"not a certificate")).is_err());
    }
}
