//! Bootstrap configuration: the environment a binary starts from and the secrets it reads from OpenBao once.
//!
//! Environment variables carry bootstrap facts only (which environment, where to listen, where OpenBao, Valkey
//! and the S3 guard are). Everything the bank may want different next month is a row of `admin.settings`.
//! Secrets come from OpenBao: a binary logs in with its AppRole, reads its database credentials and keys from
//! KV, then exchanges the login token for one that can no longer read KV.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use akr_crypto::bao::AppRole;
use akr_crypto::openbao::{HashKeys, entry_key};
use akr_crypto::{BaoClient, CryptoError, Purpose};
use rustix::process::{DumpableBehavior, Resource, getrlimit, set_dumpable_behavior};
use serde_json::Value;
use url::Url;
use zeroize::Zeroizing;

use crate::cursor::CURSOR_KEY_PATH;

/// The host name of the S3 guard on every network that reaches it (`deploy/zones.yaml`).
pub const S3_GUARD_HOST: &str = "s3guard";

/// The seed of the development profile's keys when no OpenBao is configured. It protects nothing: the
/// profile it serves is refused in production.
pub const DEV_SEED: [u8; 32] = *b"akr-tms development profile seed";

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("{var} is required")]
    Missing { var: &'static str },
    #[error("{var} is not valid: {reason}")]
    Invalid { var: &'static str, reason: String },
    #[error("the secret {path} is not usable: {reason}")]
    Secret { path: String, reason: String },
    /// Every reason this configuration may not run in its environment.
    #[error("refusing to start: {}", .0.join("; "))]
    Refused(Vec<String>),
    #[error(transparent)]
    Bao(#[from] CryptoError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Env {
    Development,
    Test,
    Staging,
    Production,
}

impl Env {
    pub fn as_str(self) -> &'static str {
        match self {
            Env::Development => "development",
            Env::Test => "test",
            Env::Staging => "staging",
            Env::Production => "production",
        }
    }

    pub fn is_production(self) -> bool {
        self == Env::Production
    }

    /// An installation, as opposed to a developer's machine or a test run.
    pub fn is_deployed(self) -> bool {
        matches!(self, Env::Staging | Env::Production)
    }
}

impl FromStr for Env {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(Env::Development),
            "test" => Ok(Env::Test),
            "staging" => Ok(Env::Staging),
            "production" => Ok(Env::Production),
            other => Err(format!(
                "`{other}` is not one of development, test, staging, production"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binary {
    Api,
    Worker,
    Ingestd,
    Assess,
    S3Guard,
    Ctl,
}

impl Binary {
    pub fn name(self) -> &'static str {
        match self {
            Binary::Api => "akr-api",
            Binary::Worker => "akr-worker",
            Binary::Ingestd => "akr-ingestd",
            Binary::Assess => "akr-assess",
            Binary::S3Guard => "akr-s3guard",
            Binary::Ctl => "akr-ctl",
        }
    }

    /// The database role the binary connects as. The guard has no database; the CLI connects as the role
    /// its command names.
    pub fn db_role(self) -> Option<&'static str> {
        match self {
            Binary::Api | Binary::Worker => Some("akr_app"),
            Binary::Ingestd => Some("akr_ingest"),
            Binary::Assess => Some("akr_assess"),
            Binary::S3Guard | Binary::Ctl => None,
        }
    }

    /// The OpenBao token role of the run token: transit and renewal, no KV.
    pub fn token_role(self) -> String {
        format!("{}-run", self.name())
    }

    /// The KV item holding the binary's object-store credential, when it has one.
    pub fn s3_credential(self) -> Option<&'static str> {
        match self {
            Binary::Api => Some("s3/presign"),
            Binary::Worker => Some("s3/writer"),
            _ => None,
        }
    }

    /// The binary reads a secret at start-up, so in production it needs OpenBao. The guard reads none.
    pub fn reads_secrets(self) -> bool {
        self.db_role().is_some() || self.s3_credential().is_some()
    }

    /// Only the API signs list cursors.
    fn reads_cursor_key(self) -> bool {
        self == Binary::Api
    }

    fn default_bind(self) -> &'static str {
        match self {
            Binary::S3Guard => "0.0.0.0:9000",
            _ => "0.0.0.0:8080",
        }
    }
}

const DEFAULT_METRICS_BIND: &str = "0.0.0.0:9101";

/// The public listener's address from `AKR_BIND`, or the binary's default. The container health probe
/// reads this and nothing else.
pub fn public_bind(binary: Binary, value: Option<&str>) -> Result<SocketAddr, ConfigError> {
    let value = value.map(str::trim).filter(|v| !v.is_empty());
    parse(value.unwrap_or(binary.default_bind()), "AKR_BIND")
}

#[derive(Debug, Clone)]
pub struct BaoConfig {
    pub addr: String,
    pub role: AppRole,
    pub cacert: Option<PathBuf>,
}

/// The bootstrap facts every binary shares. A binary wraps it in its own struct with what only it reads.
#[derive(Debug, Clone)]
pub struct CoreConfig {
    pub binary: Binary,
    pub env: Env,
    /// The public listener.
    pub bind: SocketAddr,
    /// The management listener (`/metrics`), on the management network only.
    pub metrics_bind: SocketAddr,
    /// `None` outside an installation: the development fallback of [`boot`] applies.
    pub bao: Option<BaoConfig>,
    pub valkey_url: Option<String>,
    /// The S3 guard. Never the store itself.
    pub s3_endpoint: Option<Url>,
    pub otlp_endpoint: Option<Url>,
    pub instance_id: String,
    /// The `X-Dev-Principal` stub of the auth slot is on.
    pub dev_principal: bool,
    /// Development fallback for the database when no OpenBao is configured.
    pub database_url: Option<Zeroizing<String>>,
    /// Development fallback for the object-store credential (access key id, secret key).
    pub dev_s3_credential: Option<S3Credential>,
}

impl CoreConfig {
    pub fn from_env(binary: Binary) -> Result<Self, ConfigError> {
        Self::from_map(binary, &std::env::vars().collect())
    }

    pub fn from_map(binary: Binary, vars: &HashMap<String, String>) -> Result<Self, ConfigError> {
        let get = |name: &str| vars.get(name).map(|v| v.trim()).filter(|v| !v.is_empty());

        let env: Env = parse(
            get("AKR_ENV").ok_or(ConfigError::Missing { var: "AKR_ENV" })?,
            "AKR_ENV",
        )?;
        let bind = public_bind(binary, get("AKR_BIND"))?;
        let metrics_bind = parse(
            get("AKR_METRICS_BIND").unwrap_or(DEFAULT_METRICS_BIND),
            "AKR_METRICS_BIND",
        )?;

        let bao = match get("BAO_ADDR") {
            None => None,
            Some(addr) => Some(BaoConfig {
                addr: addr.trim_end_matches('/').to_owned(),
                role: AppRole {
                    role_id: match (get("BAO_ROLE_ID"), get("BAO_ROLE_ID_FILE")) {
                        (Some(id), _) => id.to_owned(),
                        (None, Some(path)) => role_id_from(Path::new(path))?,
                        (None, None) => return Err(ConfigError::Missing { var: "BAO_ROLE_ID" }),
                    },
                    secret_id_file: get("BAO_SECRET_ID_FILE")
                        .ok_or(ConfigError::Missing {
                            var: "BAO_SECRET_ID_FILE",
                        })?
                        .into(),
                },
                cacert: get("BAO_CACERT").map(PathBuf::from),
            }),
        };

        let dev_principal = match get("AKR_DEV_PRINCIPAL") {
            None => !env.is_deployed(),
            Some("true" | "1") => true,
            Some("false" | "0") => false,
            Some(other) => {
                return Err(ConfigError::Invalid {
                    var: "AKR_DEV_PRINCIPAL",
                    reason: format!("`{other}` is not true or false"),
                });
            }
        };

        let dev_s3_credential = match (get("S3_ACCESS_KEY_ID"), get("S3_SECRET_ACCESS_KEY")) {
            (Some(id), Some(secret)) => Some(S3Credential {
                access_key_id: id.to_owned(),
                secret_access_key: Zeroizing::new(secret.to_owned()),
            }),
            _ => None,
        };

        Ok(Self {
            binary,
            env,
            bind,
            metrics_bind,
            bao,
            valkey_url: get("VALKEY_URL").map(str::to_owned),
            s3_endpoint: get("S3_ENDPOINT")
                .map(|v| parse(v, "S3_ENDPOINT"))
                .transpose()?,
            otlp_endpoint: get("OTLP_ENDPOINT")
                .map(|v| parse(v, "OTLP_ENDPOINT"))
                .transpose()?,
            instance_id: get("AKR_INSTANCE_ID").map_or_else(host_name, str::to_owned),
            dev_principal,
            database_url: get("DATABASE_URL").map(|v| Zeroizing::new(v.to_owned())),
            dev_s3_credential,
        })
    }

    pub fn valkey_url(&self) -> Result<&str, ConfigError> {
        self.valkey_url
            .as_deref()
            .ok_or(ConfigError::Missing { var: "VALKEY_URL" })
    }

    pub fn s3_endpoint(&self) -> Result<&Url, ConfigError> {
        self.s3_endpoint
            .as_ref()
            .ok_or(ConfigError::Missing { var: "S3_ENDPOINT" })
    }

    /// Refuses a configuration that may not run in production. Every reason is reported at once, so an
    /// operator fixes them in one pass.
    pub fn validate(&self, db: Option<&DbSecret>, wiring: &Wiring) -> Result<(), ConfigError> {
        if !self.env.is_production() {
            return Ok(());
        }
        let mut refusals = Vec::new();
        let mut refuse = |reason: &str| refusals.push(reason.to_owned());

        if self.dev_principal {
            refuse("the X-Dev-Principal stub is on (AKR_DEV_PRINCIPAL)");
        }
        match &self.bao {
            None if self.binary.reads_secrets() => {
                refuse("BAO_ADDR is not set: secrets must come from OpenBao");
            }
            None => {}
            Some(bao) if !bao.addr.starts_with("https://") => refuse("BAO_ADDR is not https"),
            Some(_) => {}
        }
        if self.database_url.is_some() {
            refuse("DATABASE_URL is set: database credentials must come from OpenBao");
        }
        if self.dev_s3_credential.is_some() {
            refuse("S3_ACCESS_KEY_ID is set: object-store credentials must come from OpenBao");
        }
        if db.is_some_and(|db| db.sslmode != SslMode::VerifyFull) {
            refuse("the database connection is not sslmode=verify-full");
        }
        if self.s3_endpoint.as_ref().is_some_and(|url| !is_guard(url)) {
            refuse("S3_ENDPOINT does not point at the S3 guard");
        }
        if !core_dumps_disabled() {
            refuse("core dumps are possible (RLIMIT_CORE is not 0)");
        }
        if wiring.tracing_sink {
            refuse("the audit sink is the tracing stub");
        }
        if wiring.in_memory_crypto {
            refuse("the crypto port is the in-memory implementation");
        }
        if wiring.allow_all_authorizer {
            refuse("the authorizer allows everything");
        }

        if refusals.is_empty() {
            Ok(())
        } else {
            Err(ConfigError::Refused(refusals))
        }
    }
}

/// The stand-ins a binary was wired with. Each is refused in production.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Wiring {
    pub tracing_sink: bool,
    pub in_memory_crypto: bool,
    pub allow_all_authorizer: bool,
}

fn parse<T: FromStr>(value: &str, var: &'static str) -> Result<T, ConfigError>
where
    T::Err: std::fmt::Display,
{
    value.parse().map_err(|e: T::Err| ConfigError::Invalid {
        var,
        reason: e.to_string(),
    })
}

/// A container takes its role id from a mounted file, beside the secret id.
fn role_id_from(path: &Path) -> Result<String, ConfigError> {
    let invalid = |reason: String| ConfigError::Invalid {
        var: "BAO_ROLE_ID_FILE",
        reason,
    };
    let text =
        std::fs::read_to_string(path).map_err(|e| invalid(format!("{}: {e}", path.display())))?;
    let id = text.trim();
    if id.is_empty() {
        return Err(invalid(format!("{} is empty", path.display())));
    }
    Ok(id.to_owned())
}

fn host_name() -> String {
    rustix::system::uname()
        .nodename()
        .to_string_lossy()
        .into_owned()
}

/// True when the first label of the host is the guard's name (`s3guard`, `s3guard.akr.svc`).
fn is_guard(url: &Url) -> bool {
    url.host_str()
        .and_then(|host| host.split('.').next())
        .is_some_and(|label| label == S3_GUARD_HOST)
}

fn core_dumps_disabled() -> bool {
    getrlimit(Resource::Core).current == Some(0)
}

/// Makes the process undumpable in an installation, so a crash or a debugger cannot read keys out of its
/// memory. A developer's machine keeps its debugger.
pub fn harden_process(env: Env) -> Result<(), ConfigError> {
    if !env.is_deployed() {
        return Ok(());
    }
    set_dumpable_behavior(DumpableBehavior::NotDumpable).map_err(|e| {
        ConfigError::Refused(vec![format!("the process cannot be made undumpable: {e}")])
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SslMode {
    Disable,
    Allow,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl SslMode {
    pub fn as_str(self) -> &'static str {
        match self {
            SslMode::Disable => "disable",
            SslMode::Allow => "allow",
            SslMode::Prefer => "prefer",
            SslMode::Require => "require",
            SslMode::VerifyCa => "verify-ca",
            SslMode::VerifyFull => "verify-full",
        }
    }
}

impl FromStr for SslMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "disable" => Ok(SslMode::Disable),
            "allow" => Ok(SslMode::Allow),
            "prefer" => Ok(SslMode::Prefer),
            "require" => Ok(SslMode::Require),
            "verify-ca" => Ok(SslMode::VerifyCa),
            "verify-full" => Ok(SslMode::VerifyFull),
            other => Err(format!("`{other}` is not an sslmode")),
        }
    }
}

/// How to reach the database as one role: the KV item `db/<role>`.
#[derive(Clone)]
pub struct DbSecret {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: Zeroizing<String>,
    pub sslmode: SslMode,
    pub sslrootcert: Option<PathBuf>,
}

impl std::fmt::Debug for DbSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbSecret")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("database", &self.database)
            .field("username", &self.username)
            .field("sslmode", &self.sslmode)
            .finish_non_exhaustive()
    }
}

impl DbSecret {
    /// Reads the KV item `path`: `{host, port, database, username, password, sslmode, sslrootcert?}`.
    pub fn from_kv(path: &str, item: &Value) -> Result<Self, ConfigError> {
        let bad = |reason: String| ConfigError::Secret {
            path: path.to_owned(),
            reason,
        };
        let text = |field: &str| {
            item.get(field)
                .and_then(Value::as_str)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| bad(format!("`{field}` is missing")))
        };
        let port = match item.get("port") {
            Some(Value::Number(n)) => n.as_u64().and_then(|n| u16::try_from(n).ok()),
            Some(Value::String(s)) => s.parse().ok(),
            _ => None,
        }
        .ok_or_else(|| bad("`port` is missing or not a port".to_owned()))?;
        Ok(Self {
            host: text("host")?.to_owned(),
            port,
            database: text("database")?.to_owned(),
            username: text("username")?.to_owned(),
            password: Zeroizing::new(text("password")?.to_owned()),
            sslmode: text("sslmode")?.parse().map_err(bad)?,
            sslrootcert: item
                .get("sslrootcert")
                .and_then(Value::as_str)
                .filter(|v| !v.is_empty())
                .map(PathBuf::from),
        })
    }

    /// Reads a `postgres://user:password@host:port/database?sslmode=..&sslrootcert=..` URL (the development
    /// fallback). The connection is never written back as a URL.
    pub fn from_url(database_url: &str) -> Result<Self, ConfigError> {
        let bad = |reason: &str| ConfigError::Invalid {
            var: "DATABASE_URL",
            reason: reason.to_owned(),
        };
        let url = Url::parse(database_url).map_err(|_| bad("it is not a URL"))?;
        if !matches!(url.scheme(), "postgres" | "postgresql") {
            return Err(bad("the scheme is not postgres"));
        }
        let query: HashMap<_, _> = url.query_pairs().collect();
        let sslmode = match query.get("sslmode") {
            Some(mode) => mode.parse().map_err(|_| bad("the sslmode is not known"))?,
            None => SslMode::Prefer,
        };
        Ok(Self {
            host: url
                .host_str()
                .ok_or_else(|| bad("it names no host"))?
                .to_owned(),
            port: url.port().unwrap_or(5432),
            database: percent_decode(url.path().trim_start_matches('/')),
            username: percent_decode(url.username()),
            password: Zeroizing::new(percent_decode(url.password().unwrap_or_default())),
            sslmode,
            sslrootcert: query.get("sslrootcert").map(|p| PathBuf::from(p.as_ref())),
        })
    }
}

fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let hex = (bytes[i] == b'%')
            .then(|| text.get(i + 1..i + 3))
            .flatten()
            .and_then(|pair| u8::from_str_radix(pair, 16).ok());
        match hex {
            Some(byte) => {
                out.push(byte);
                i += 3;
            }
            None => {
                out.push(bytes[i]);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// A credential of the object store: the KV item `s3/<name>` (`{access_key_id, secret_access_key}`).
#[derive(Clone)]
pub struct S3Credential {
    pub access_key_id: String,
    pub secret_access_key: Zeroizing<String>,
}

impl std::fmt::Debug for S3Credential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("S3Credential")
            .field("access_key_id", &self.access_key_id)
            .finish_non_exhaustive()
    }
}

impl S3Credential {
    pub fn from_kv(path: &str, item: &Value) -> Result<Self, ConfigError> {
        let text = |field: &str| {
            item.get(field)
                .and_then(Value::as_str)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| ConfigError::Secret {
                    path: path.to_owned(),
                    reason: format!("`{field}` is missing"),
                })
        };
        Ok(Self {
            access_key_id: text("access_key_id")?.to_owned(),
            secret_access_key: Zeroizing::new(text("secret_access_key")?.to_owned()),
        })
    }
}

/// The live OpenBao session of a binary: the client holding the run token, and the keyed-hash keys read
/// while KV was still readable.
pub struct BaoSession {
    pub client: BaoClient,
    pub hash_keys: HashKeys,
}

/// What a binary knows after start-up: where its database is, its keys and its OpenBao session.
pub struct Boot {
    pub db: Option<DbSecret>,
    /// The key that signs list cursors; only the API has one.
    pub cursor_key: Option<Zeroizing<Vec<u8>>>,
    pub s3: Option<S3Credential>,
    /// `None` under the development fallback: the binary wires `InMemoryCrypto` from [`DEV_SEED`].
    pub bao: Option<BaoSession>,
}

/// Reads the binary's secrets.
///
/// With OpenBao configured: logs in with the AppRole, reads `db/<role>`, the cursor key, the keys of
/// `purposes` and the object-store credential, exchanges the login token for the run token (transit and
/// renewal only) and keeps that token alive for as long as the process runs.
///
/// Without it, outside production: the database comes from `DATABASE_URL` and the keys from [`DEV_SEED`].
pub async fn boot(core: &CoreConfig, purposes: &[Purpose]) -> Result<Boot, ConfigError> {
    let binary = core.binary;
    let Some(bao) = &core.bao else {
        if core.env.is_production() {
            return Err(ConfigError::Missing { var: "BAO_ADDR" });
        }
        let db = match (binary.db_role(), &core.database_url) {
            (Some(_), Some(url)) => Some(DbSecret::from_url(url)?),
            (Some(_), None) => {
                return Err(ConfigError::Missing {
                    var: "DATABASE_URL",
                });
            }
            (None, _) => None,
        };
        return Ok(Boot {
            db,
            cursor_key: binary
                .reads_cursor_key()
                .then(|| Zeroizing::new(DEV_SEED.to_vec())),
            s3: core.dev_s3_credential.clone(),
            bao: None,
        });
    };

    let ca = match &bao.cacert {
        Some(path) => Some(
            tokio::fs::read(path)
                .await
                .map_err(|e| ConfigError::Invalid {
                    var: "BAO_CACERT",
                    reason: e.to_string(),
                })?,
        ),
        None => None,
    };
    let client = BaoClient::new(&bao.addr, ca.as_deref())?;
    client.login(&bao.role).await?;

    let db = match binary.db_role() {
        Some(role) => {
            let path = format!("db/{role}");
            Some(DbSecret::from_kv(&path, &client.kv_read(&path).await?)?)
        }
        None => None,
    };
    let cursor_key = if binary.reads_cursor_key() {
        Some(entry_key(
            CURSOR_KEY_PATH,
            &client.kv_read(CURSOR_KEY_PATH).await?,
        )?)
    } else {
        None
    };
    let hash_keys = HashKeys::load(&client, purposes).await?;
    let s3 = match binary.s3_credential() {
        Some(path) => Some(S3Credential::from_kv(path, &client.kv_read(path).await?)?),
        None => None,
    };

    let token_role = binary.token_role();
    let ttl = client.narrow_to(&token_role).await?;
    tokio::spawn({
        let client = client.clone();
        let role = bao.role.clone();
        async move {
            client.keep_alive(role, Some(token_role), ttl).await;
        }
    });

    Ok(Boot {
        db,
        cursor_key,
        s3,
        bao: Some(BaoSession { client, hash_keys }),
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::*;

    fn vars(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect()
    }

    fn production() -> HashMap<String, String> {
        vars(&[
            ("AKR_ENV", "production"),
            ("BAO_ADDR", "https://openbao:8200"),
            ("BAO_ROLE_ID", "role-id"),
            ("BAO_SECRET_ID_FILE", "/run/secrets/bao_secret_id"),
            ("VALKEY_URL", "rediss://valkey:6379"),
            ("S3_ENDPOINT", "https://s3guard:9000"),
        ])
    }

    fn verified_db() -> DbSecret {
        DbSecret::from_kv(
            "db/akr_app",
            &json!({"host": "postgres", "port": 5432, "database": "akr", "username": "akr_app_login",
                    "password": "pw", "sslmode": "verify-full", "sslrootcert": "/etc/akr/ca.crt"}),
        )
        .unwrap()
    }

    #[test]
    fn the_role_id_comes_from_a_file_when_the_variable_is_absent() {
        let path = std::env::temp_dir().join(format!("akr-role-id-{}", std::process::id()));
        std::fs::write(&path, "role-from-file\n").unwrap();
        let mut vars = production();
        vars.remove("BAO_ROLE_ID");
        assert!(matches!(
            CoreConfig::from_map(Binary::Api, &vars),
            Err(ConfigError::Missing { var: "BAO_ROLE_ID" })
        ));

        vars.insert("BAO_ROLE_ID_FILE".to_owned(), path.display().to_string());
        let core = CoreConfig::from_map(Binary::Api, &vars).unwrap();
        assert_eq!(core.bao.unwrap().role.role_id, "role-from-file");

        std::fs::write(&path, "\n").unwrap();
        assert!(matches!(
            CoreConfig::from_map(Binary::Api, &vars),
            Err(ConfigError::Invalid {
                var: "BAO_ROLE_ID_FILE",
                ..
            })
        ));
        std::fs::remove_file(&path).unwrap();
    }

    fn refusals(result: Result<(), ConfigError>) -> Vec<String> {
        match result {
            Err(ConfigError::Refused(list)) => list,
            other => panic!("expected a refusal, got {other:?}"),
        }
    }

    #[test]
    fn a_development_start_needs_only_the_environment() {
        let core = CoreConfig::from_map(Binary::Api, &vars(&[("AKR_ENV", "development")])).unwrap();
        assert_eq!(core.env, Env::Development);
        assert_eq!(core.bind, "0.0.0.0:8080".parse().unwrap());
        assert_eq!(core.metrics_bind, "0.0.0.0:9101".parse().unwrap());
        assert!(core.bao.is_none());
        assert!(core.dev_principal);
        assert!(!core.instance_id.is_empty());
        assert!(core.valkey_url().is_err());
    }

    #[test]
    fn what_is_missing_or_malformed_is_named() {
        assert!(matches!(
            CoreConfig::from_map(Binary::Api, &vars(&[])),
            Err(ConfigError::Missing { var: "AKR_ENV" })
        ));
        assert!(matches!(
            CoreConfig::from_map(Binary::Api, &vars(&[("AKR_ENV", "prod")])),
            Err(ConfigError::Invalid { var: "AKR_ENV", .. })
        ));
        assert!(matches!(
            CoreConfig::from_map(
                Binary::Api,
                &vars(&[("AKR_ENV", "test"), ("AKR_BIND", "8080")])
            ),
            Err(ConfigError::Invalid {
                var: "AKR_BIND",
                ..
            })
        ));
        assert!(matches!(
            CoreConfig::from_map(
                Binary::Api,
                &vars(&[("AKR_ENV", "test"), ("BAO_ADDR", "https://openbao:8200")])
            ),
            Err(ConfigError::Missing { var: "BAO_ROLE_ID" })
        ));
        assert!(matches!(
            CoreConfig::from_map(
                Binary::Api,
                &vars(&[("AKR_ENV", "test"), ("AKR_DEV_PRINCIPAL", "yes")])
            ),
            Err(ConfigError::Invalid {
                var: "AKR_DEV_PRINCIPAL",
                ..
            })
        ));
    }

    #[test]
    fn the_dev_principal_is_off_in_an_installation_unless_switched_on() {
        let core = CoreConfig::from_map(Binary::Api, &production()).unwrap();
        assert!(!core.dev_principal);
        let staging = CoreConfig::from_map(Binary::Api, &vars(&[("AKR_ENV", "staging")])).unwrap();
        assert!(!staging.dev_principal);
        let test = CoreConfig::from_map(Binary::Api, &vars(&[("AKR_ENV", "test")])).unwrap();
        assert!(test.dev_principal);
        let off = CoreConfig::from_map(
            Binary::Api,
            &vars(&[("AKR_ENV", "development"), ("AKR_DEV_PRINCIPAL", "false")]),
        )
        .unwrap();
        assert!(!off.dev_principal);
    }

    #[test]
    fn production_refuses_every_stand_in_and_names_them_all() {
        let mut env = production();
        env.insert("AKR_DEV_PRINCIPAL".to_owned(), "true".to_owned());
        env.insert("BAO_ADDR".to_owned(), "http://openbao:8200".to_owned());
        env.insert(
            "S3_ENDPOINT".to_owned(),
            "https://objectstore:9000".to_owned(),
        );
        env.insert(
            "DATABASE_URL".to_owned(),
            "postgres://u:p@postgres/akr".to_owned(),
        );
        let core = CoreConfig::from_map(Binary::Api, &env).unwrap();
        let weak_db = DbSecret::from_url("postgres://u:p@postgres/akr?sslmode=require").unwrap();
        let stubs = Wiring {
            tracing_sink: true,
            in_memory_crypto: true,
            allow_all_authorizer: true,
        };

        let list = refusals(core.validate(Some(&weak_db), &stubs));
        for needle in [
            "X-Dev-Principal",
            "BAO_ADDR is not https",
            "DATABASE_URL is set",
            "verify-full",
            "S3 guard",
            "tracing stub",
            "in-memory",
            "allows everything",
        ] {
            assert!(
                list.iter().any(|r| r.contains(needle)),
                "no refusal names {needle}: {list:?}"
            );
        }
    }

    #[test]
    fn a_clean_production_configuration_is_refused_only_for_core_dumps_if_at_all() {
        let core = CoreConfig::from_map(Binary::Api, &production()).unwrap();
        match core.validate(Some(&verified_db()), &Wiring::default()) {
            Ok(()) => assert!(core_dumps_disabled()),
            Err(ConfigError::Refused(list)) => {
                assert!(!core_dumps_disabled());
                assert_eq!(list.len(), 1, "{list:?}");
                assert!(list[0].contains("RLIMIT_CORE"));
            }
            Err(other) => panic!("{other:?}"),
        }
    }

    #[test]
    fn a_binary_that_reads_no_secret_needs_no_openbao_in_production() {
        let guard =
            CoreConfig::from_map(Binary::S3Guard, &vars(&[("AKR_ENV", "production")])).unwrap();
        assert!(!Binary::S3Guard.reads_secrets());
        assert_eq!(guard.bind, "0.0.0.0:9000".parse().unwrap());
        if let Err(ConfigError::Refused(list)) = guard.validate(None, &Wiring::default()) {
            assert_eq!(list.len(), 1, "{list:?}");
            assert!(list[0].contains("RLIMIT_CORE"));
        }

        let api = CoreConfig::from_map(Binary::Api, &vars(&[("AKR_ENV", "production")])).unwrap();
        assert!(Binary::Api.reads_secrets());
        let list = refusals(api.validate(None, &Wiring::default()));
        assert!(list.iter().any(|r| r.contains("BAO_ADDR is not set")));
    }

    #[test]
    fn outside_production_nothing_is_refused() {
        let core = CoreConfig::from_map(Binary::Api, &vars(&[("AKR_ENV", "development")])).unwrap();
        let stubs = Wiring {
            tracing_sink: true,
            in_memory_crypto: true,
            allow_all_authorizer: true,
        };
        assert!(core.validate(None, &stubs).is_ok());
    }

    #[test]
    fn the_guard_is_recognised_by_the_first_label_of_its_host() {
        let guard = |url: &str| is_guard(&Url::parse(url).unwrap());
        assert!(guard("https://s3guard:9000"));
        assert!(guard("https://s3guard.akr.svc.cluster.local:9000"));
        assert!(!guard("https://objectstore:9000"));
        assert!(!guard("https://s3guard-evil.example:9000"));
        assert!(!guard("https://10.0.0.7:9000"));
    }

    #[test]
    fn a_database_secret_reads_from_kv_and_names_what_is_missing() {
        let db = verified_db();
        assert_eq!(db.host, "postgres");
        assert_eq!(db.port, 5432);
        assert_eq!(db.sslmode, SslMode::VerifyFull);
        assert_eq!(db.sslrootcert, Some(PathBuf::from("/etc/akr/ca.crt")));
        assert!(!format!("{db:?}").contains("pw"));

        let missing = DbSecret::from_kv("db/akr_app", &json!({"host": "postgres", "port": "5432"}));
        match missing {
            Err(ConfigError::Secret { path, reason }) => {
                assert_eq!(path, "db/akr_app");
                assert!(reason.contains("database"));
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn a_database_url_reads_with_its_escapes_and_tls_options() {
        let db = DbSecret::from_url(
            "postgres://akr_app_login:p%40ss%2Fword@127.0.0.1:5470/akr_dev?sslmode=verify-full&sslrootcert=/tmp/ca.crt",
        )
        .unwrap();
        assert_eq!(db.username, "akr_app_login");
        assert_eq!(db.password.as_str(), "p@ss/word");
        assert_eq!(db.port, 5470);
        assert_eq!(db.database, "akr_dev");
        assert_eq!(db.sslmode, SslMode::VerifyFull);
        assert_eq!(db.sslrootcert, Some(PathBuf::from("/tmp/ca.crt")));

        assert!(DbSecret::from_url("mysql://u:p@h/db").is_err());
        assert!(DbSecret::from_url("postgres://u:p@h/db?sslmode=strict").is_err());
    }

    #[test]
    fn each_binary_knows_its_role_and_its_run_token() {
        assert_eq!(Binary::Api.db_role(), Some("akr_app"));
        assert_eq!(Binary::Worker.db_role(), Some("akr_app"));
        assert_eq!(Binary::Ingestd.db_role(), Some("akr_ingest"));
        assert_eq!(Binary::Assess.db_role(), Some("akr_assess"));
        assert_eq!(Binary::S3Guard.db_role(), None);
        assert_eq!(Binary::Worker.token_role(), "akr-worker-run");
        assert_eq!(Binary::Api.s3_credential(), Some("s3/presign"));
        assert_eq!(Binary::Worker.s3_credential(), Some("s3/writer"));
        assert_eq!(Binary::Ingestd.s3_credential(), None);
    }

    #[tokio::test]
    async fn without_openbao_a_development_binary_boots_from_the_database_url() {
        let core = CoreConfig::from_map(
            Binary::Api,
            &vars(&[
                ("AKR_ENV", "development"),
                (
                    "DATABASE_URL",
                    "postgres://akr_app_login:pw@127.0.0.1:5470/akr_dev?sslmode=verify-full",
                ),
            ]),
        )
        .unwrap();
        let boot = boot(&core, &[]).await.unwrap();
        assert_eq!(boot.db.unwrap().username, "akr_app_login");
        assert_eq!(boot.cursor_key.unwrap().len(), 32);
        assert!(boot.bao.is_none());

        let no_url = CoreConfig::from_map(Binary::Worker, &vars(&[("AKR_ENV", "test")])).unwrap();
        assert!(matches!(
            super::boot(&no_url, &[]).await,
            Err(ConfigError::Missing {
                var: "DATABASE_URL"
            })
        ));

        let guard = CoreConfig::from_map(Binary::S3Guard, &vars(&[("AKR_ENV", "test")])).unwrap();
        assert!(super::boot(&guard, &[]).await.unwrap().db.is_none());
    }

    #[tokio::test]
    async fn production_never_falls_back_to_the_environment() {
        let mut env = production();
        env.remove("BAO_ADDR");
        env.insert(
            "DATABASE_URL".to_owned(),
            "postgres://u:p@postgres/akr".to_owned(),
        );
        let core = CoreConfig::from_map(Binary::Api, &env).unwrap();
        assert!(matches!(
            boot(&core, &[]).await,
            Err(ConfigError::Missing { var: "BAO_ADDR" })
        ));
    }
}
