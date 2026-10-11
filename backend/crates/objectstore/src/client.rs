//! The S3 client: path-style requests, header signing, one signed part per body.
//!
//! [`ObjectStore`] is what the application uses. The inherent methods are the installer's (`akr-ctl store
//! init`) and the conformance suite's: buckets, retention, legal holds and versions.

use std::time::Duration;

use akr_common::config::S3Credential;
use async_trait::async_trait;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use bytes::Bytes;
use chrono::{DateTime, SecondsFormat, Utc};
use reqwest::header::HeaderMap;
use reqwest::{Method, Response};
use sha2::{Digest, Sha256};
use url::Url;

use crate::sigv4::{Canonical, Signer, amz_date, canonical_query, uri_encode};
use crate::xml::{blocks, element};
use crate::{
    LockMode, ObjectMeta, ObjectStore, ObjectStoreError, Retention, RetentionMode, VersionId,
    check_key, check_ttl,
};

const DEFAULT_REGION: &str = "us-east-1";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const XMLNS: &str = "http://s3.amazonaws.com/doc/2006-03-01/";
const ERROR_BODY_LIMIT: usize = 16 * 1024;
const NOT_FOUND: u16 = 404;
const CONFLICT: u16 = 409;

/// The header the S3 guard sets on a request it refused, naming the rule.
pub const GUARD_RULE_HEADER: &str = "x-akr-s3guard-rule";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetentionPeriod {
    Days(u32),
    Years(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultRetention {
    pub mode: RetentionMode,
    pub period: RetentionPeriod,
}

/// The object-lock configuration of a bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BucketLock {
    pub enabled: bool,
    pub default: Option<DefaultRetention>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectVersion {
    pub key: String,
    pub version_id: VersionId,
    pub is_latest: bool,
    pub delete_marker: bool,
    pub size: u64,
}

pub struct S3Client {
    http: reqwest::Client,
    /// `scheme://host[:port]`.
    origin: String,
    /// The `Host` header the signature covers.
    host: String,
    region: String,
    credential: S3Credential,
}

impl std::fmt::Debug for S3Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("S3Client")
            .field("origin", &self.origin)
            .field("region", &self.region)
            .field("access_key_id", &self.credential.access_key_id)
            .finish_non_exhaustive()
    }
}

struct Call<'a> {
    operation: &'static str,
    method: Method,
    bucket: &'a str,
    key: Option<&'a str>,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    body: Bytes,
    sha256: Option<[u8; 32]>,
}

impl<'a> Call<'a> {
    fn new(operation: &'static str, method: Method, bucket: &'a str) -> Self {
        Self {
            operation,
            method,
            bucket,
            key: None,
            query: Vec::new(),
            headers: Vec::new(),
            body: Bytes::new(),
            sha256: None,
        }
    }

    fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    fn query(mut self, name: &str, value: &str) -> Self {
        self.query.push((name.to_owned(), value.to_owned()));
        self
    }

    fn version(self, version: Option<&VersionId>) -> Self {
        match version {
            Some(version) => self.query("versionId", version.as_str()),
            None => self,
        }
    }

    /// `name` in lower case: every header set here is signed.
    fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_owned(), value.to_owned()));
        self
    }

    fn xml(mut self, document: String) -> Self {
        let body = Bytes::from(document);
        self = self
            .header("content-type", "application/xml")
            .header("content-md5", &md5_base64(&body));
        self.body = body;
        self
    }
}

impl S3Client {
    /// `endpoint` is the S3 guard: scheme, host and port, with no path.
    pub fn new(endpoint: &Url, credential: S3Credential) -> Result<Self, ObjectStoreError> {
        let config = |reason: &str| ObjectStoreError::Config(format!("{endpoint}: {reason}"));
        if !matches!(endpoint.scheme(), "http" | "https") {
            return Err(config("the scheme is not http or https"));
        }
        if !matches!(endpoint.path(), "" | "/") || endpoint.query().is_some() {
            return Err(config("the endpoint has a path or a query"));
        }
        let host = endpoint
            .host_str()
            .ok_or_else(|| config("the endpoint has no host"))?;
        let host = match endpoint.port() {
            Some(port) => format!("{host}:{port}"),
            None => host.to_owned(),
        };
        let http = reqwest::Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(|e| ObjectStoreError::Config(e.to_string()))?;
        Ok(Self {
            http,
            origin: format!("{}://{host}", endpoint.scheme()),
            host,
            region: DEFAULT_REGION.to_owned(),
            credential,
        })
    }

    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = region.into();
        self
    }

    fn signer(&self) -> Signer<'_> {
        Signer {
            access_key_id: &self.credential.access_key_id,
            secret_access_key: &self.credential.secret_access_key,
            region: &self.region,
        }
    }

    /// Signs and sends. Every answer outside 2xx comes back as [`ObjectStoreError::Refused`].
    async fn send(&self, call: Call<'_>) -> Result<Response, ObjectStoreError> {
        let path = object_path(call.bucket, call.key)?;
        let now = Utc::now();
        let payload_hash = hex::encode(
            call.sha256
                .unwrap_or_else(|| Sha256::digest(&call.body).into()),
        );

        let mut headers = call.headers;
        headers.push(("host".to_owned(), self.host.clone()));
        headers.push(("x-amz-content-sha256".to_owned(), payload_hash.clone()));
        headers.push(("x-amz-date".to_owned(), amz_date(now)));
        let authorization = self.signer().authorization(
            now,
            &Canonical {
                method: call.method.as_str(),
                path: &path,
                query: &call.query,
                headers: &headers,
                payload_hash: &payload_hash,
            },
        )?;

        let query = canonical_query(&call.query);
        let url = if query.is_empty() {
            format!("{}{path}", self.origin)
        } else {
            format!("{}{path}?{query}", self.origin)
        };
        let mut request = self
            .http
            .request(call.method, url)
            .header("authorization", authorization);
        for (name, value) in headers.iter().filter(|(name, _)| name != "host") {
            request = request.header(name, value);
        }
        let response = request.body(call.body).send().await.map_err(unavailable)?;

        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        let guard_rule = response
            .headers()
            .get(GUARD_RULE_HEADER)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let code = match guard_rule {
            Some(rule) => format!("s3guard:{rule}"),
            None => error_code(response).await,
        };
        tracing::debug!(
            operation = call.operation,
            status = status.as_u16(),
            code,
            "the object store refused a request"
        );
        Err(ObjectStoreError::Refused {
            operation: call.operation,
            status: status.as_u16(),
            code,
        })
    }

    async fn read(
        &self,
        bucket: &str,
        key: &str,
        version: Option<&VersionId>,
    ) -> Result<Bytes, ObjectStoreError> {
        let call = Call::new("GetObject", Method::GET, bucket)
            .key(key)
            .version(version)
            .header("x-amz-checksum-mode", "ENABLED");
        let response = missing_is_not_found(self.send(call).await, || format!("{bucket}/{key}"))?;
        let recorded = checksum_of(response.headers());
        let bytes = response.bytes().await.map_err(unavailable)?;
        if let Some(recorded) = recorded {
            let actual: [u8; 32] = Sha256::digest(&bytes).into();
            if actual != recorded {
                return Err(ObjectStoreError::ChecksumMismatch {
                    key: key.to_owned(),
                    side: "the digest the store recorded",
                });
            }
        }
        Ok(bytes)
    }

    /// One named version, checked like [`ObjectStore::get`].
    pub async fn get_version(
        &self,
        bucket: &str,
        key: &str,
        version: &VersionId,
    ) -> Result<Bytes, ObjectStoreError> {
        self.read(bucket, key, Some(version)).await
    }

    /// `at` is the signing time: the suite that proves URLs expire signs one in the past.
    pub fn presign_get_at(
        &self,
        bucket: &str,
        key: &str,
        ttl: Duration,
        at: DateTime<Utc>,
    ) -> Result<Url, ObjectStoreError> {
        let expires_s = check_ttl(ttl)?;
        let path = object_path(bucket, Some(key))?;
        let query = self
            .signer()
            .presigned_query(at, "GET", &path, &self.host, expires_s, &[])?;
        Url::parse(&format!("{}{path}?{query}", self.origin))
            .map_err(|e| ObjectStoreError::Config(e.to_string()))
    }

    /// `false` when the bucket was already there and is this credential's.
    pub async fn create_bucket(
        &self,
        bucket: &str,
        object_lock: bool,
    ) -> Result<bool, ObjectStoreError> {
        let mut call = Call::new("CreateBucket", Method::PUT, bucket);
        if object_lock {
            call = call.header("x-amz-bucket-object-lock-enabled", "true");
        }
        match self.send(call).await {
            Ok(_) => Ok(true),
            Err(ObjectStoreError::Refused {
                status: CONFLICT,
                code,
                ..
            }) if code == "BucketAlreadyOwnedByYou" => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_bucket(&self, bucket: &str) -> Result<(), ObjectStoreError> {
        let call = Call::new("DeleteBucket", Method::DELETE, bucket);
        missing_is_not_found(self.send(call).await, || bucket.to_owned()).map(drop)
    }

    pub async fn put_default_retention(
        &self,
        bucket: &str,
        retention: DefaultRetention,
    ) -> Result<(), ObjectStoreError> {
        let period = match retention.period {
            RetentionPeriod::Days(days) => format!("<Days>{days}</Days>"),
            RetentionPeriod::Years(years) => format!("<Years>{years}</Years>"),
        };
        let document = format!(
            "<ObjectLockConfiguration xmlns=\"{XMLNS}\"><ObjectLockEnabled>Enabled</ObjectLockEnabled>\
             <Rule><DefaultRetention><Mode>{}</Mode>{period}</DefaultRetention></Rule>\
             </ObjectLockConfiguration>",
            retention.mode.as_str()
        );
        let call = Call::new("PutObjectLockConfiguration", Method::PUT, bucket)
            .query("object-lock", "")
            .xml(document);
        missing_is_not_found(self.send(call).await, || bucket.to_owned()).map(drop)
    }

    pub async fn object_lock(&self, bucket: &str) -> Result<BucketLock, ObjectStoreError> {
        const OPERATION: &str = "GetObjectLockConfiguration";
        let call = Call::new(OPERATION, Method::GET, bucket).query("object-lock", "");
        let response = match self.send(call).await {
            Ok(response) => response,
            Err(ObjectStoreError::Refused {
                status: NOT_FOUND,
                code,
                ..
            }) if code == "ObjectLockConfigurationNotFoundError" => {
                return Ok(BucketLock {
                    enabled: false,
                    default: None,
                });
            }
            Err(e) => return missing_is_not_found(Err(e), || bucket.to_owned()),
        };
        let document = response.text().await.map_err(unavailable)?;
        let enabled = element(&document, "ObjectLockEnabled").as_deref() == Some("Enabled");
        let default = match blocks(&document, "DefaultRetention").next() {
            None => None,
            Some(rule) => {
                let mode = element(rule, "Mode")
                    .as_deref()
                    .and_then(RetentionMode::parse)
                    .ok_or_else(|| malformed(OPERATION, "a default retention with no mode"))?;
                let number = |tag: &str| element(rule, tag).and_then(|text| text.parse().ok());
                let period = match (number("Days"), number("Years")) {
                    (Some(days), None) => RetentionPeriod::Days(days),
                    (None, Some(years)) => RetentionPeriod::Years(years),
                    _ => return Err(malformed(OPERATION, "a default retention with no period")),
                };
                Some(DefaultRetention { mode, period })
            }
        };
        Ok(BucketLock { enabled, default })
    }

    /// Deletes one version, or with no version adds a delete marker (in an unversioned bucket: deletes).
    pub async fn delete_object(
        &self,
        bucket: &str,
        key: &str,
        version: Option<&VersionId>,
        bypass_governance: bool,
    ) -> Result<(), ObjectStoreError> {
        let mut call = Call::new("DeleteObject", Method::DELETE, bucket)
            .key(key)
            .version(version);
        if bypass_governance {
            call = call.header("x-amz-bypass-governance-retention", "true");
        }
        missing_is_not_found(self.send(call).await, || format!("{bucket}/{key}")).map(drop)
    }

    pub async fn put_retention(
        &self,
        bucket: &str,
        key: &str,
        version: Option<&VersionId>,
        retention: Retention,
        bypass_governance: bool,
    ) -> Result<(), ObjectStoreError> {
        let document = format!(
            "<Retention xmlns=\"{XMLNS}\"><Mode>{}</Mode><RetainUntilDate>{}</RetainUntilDate></Retention>",
            retention.mode.as_str(),
            timestamp(retention.until)
        );
        let mut call = Call::new("PutObjectRetention", Method::PUT, bucket)
            .key(key)
            .query("retention", "")
            .version(version)
            .xml(document);
        if bypass_governance {
            call = call.header("x-amz-bypass-governance-retention", "true");
        }
        missing_is_not_found(self.send(call).await, || format!("{bucket}/{key}")).map(drop)
    }

    /// `None` when the version carries no retention.
    pub async fn retention(
        &self,
        bucket: &str,
        key: &str,
        version: Option<&VersionId>,
    ) -> Result<Option<Retention>, ObjectStoreError> {
        const OPERATION: &str = "GetObjectRetention";
        let call = Call::new(OPERATION, Method::GET, bucket)
            .key(key)
            .query("retention", "")
            .version(version);
        let response = match self.send(call).await {
            Ok(response) => response,
            Err(ObjectStoreError::Refused {
                status: NOT_FOUND,
                code,
                ..
            }) if code == "NoSuchObjectLockConfiguration" => return Ok(None),
            Err(e) => return missing_is_not_found(Err(e), || format!("{bucket}/{key}")),
        };
        let document = response.text().await.map_err(unavailable)?;
        let mode = element(&document, "Mode")
            .as_deref()
            .and_then(RetentionMode::parse);
        let until = element(&document, "RetainUntilDate")
            .as_deref()
            .and_then(parse_timestamp);
        match (mode, until) {
            (Some(mode), Some(until)) => Ok(Some(Retention { mode, until })),
            (None, None) => Ok(None),
            _ => Err(malformed(OPERATION, "a retention with half its fields")),
        }
    }

    pub async fn put_legal_hold(
        &self,
        bucket: &str,
        key: &str,
        version: Option<&VersionId>,
        on: bool,
    ) -> Result<(), ObjectStoreError> {
        let document = format!(
            "<LegalHold xmlns=\"{XMLNS}\"><Status>{}</Status></LegalHold>",
            if on { "ON" } else { "OFF" }
        );
        let call = Call::new("PutObjectLegalHold", Method::PUT, bucket)
            .key(key)
            .query("legal-hold", "")
            .version(version)
            .xml(document);
        missing_is_not_found(self.send(call).await, || format!("{bucket}/{key}")).map(drop)
    }

    pub async fn legal_hold(
        &self,
        bucket: &str,
        key: &str,
        version: Option<&VersionId>,
    ) -> Result<bool, ObjectStoreError> {
        let call = Call::new("GetObjectLegalHold", Method::GET, bucket)
            .key(key)
            .query("legal-hold", "")
            .version(version);
        let response = match self.send(call).await {
            Ok(response) => response,
            Err(ObjectStoreError::Refused {
                status: NOT_FOUND,
                code,
                ..
            }) if code == "NoSuchObjectLockConfiguration" => return Ok(false),
            Err(e) => return missing_is_not_found(Err(e), || format!("{bucket}/{key}")),
        };
        let document = response.text().await.map_err(unavailable)?;
        Ok(element(&document, "Status").as_deref() == Some("ON"))
    }

    /// Every version and delete marker under `prefix`, newest first within a key.
    pub async fn list_versions(
        &self,
        bucket: &str,
        prefix: &str,
    ) -> Result<Vec<ObjectVersion>, ObjectStoreError> {
        const OPERATION: &str = "ListObjectVersions";
        let mut versions = Vec::new();
        let mut marker: Option<(String, String)> = None;
        loop {
            let mut call = Call::new(OPERATION, Method::GET, bucket)
                .query("versions", "")
                .query("prefix", prefix);
            if let Some((key, version)) = &marker {
                call = call
                    .query("key-marker", key)
                    .query("version-id-marker", version);
            }
            let response = missing_is_not_found(self.send(call).await, || bucket.to_owned())?;
            let document = response.text().await.map_err(unavailable)?;

            for (tag, delete_marker) in [("Version", false), ("DeleteMarker", true)] {
                for block in blocks(&document, tag) {
                    let key = element(block, "Key")
                        .ok_or_else(|| malformed(OPERATION, "a version with no key"))?;
                    let version_id = element(block, "VersionId")
                        .ok_or_else(|| malformed(OPERATION, "a version with no id"))?;
                    versions.push(ObjectVersion {
                        key,
                        version_id: VersionId(version_id),
                        is_latest: element(block, "IsLatest").as_deref() == Some("true"),
                        delete_marker,
                        size: element(block, "Size")
                            .and_then(|size| size.parse().ok())
                            .unwrap_or(0),
                    });
                }
            }

            if element(&document, "IsTruncated").as_deref() != Some("true") {
                return Ok(versions);
            }
            let next_key = element(&document, "NextKeyMarker")
                .ok_or_else(|| malformed(OPERATION, "a truncated page with no marker"))?;
            let next_version = element(&document, "NextVersionIdMarker").unwrap_or_default();
            marker = Some((next_key, next_version));
        }
    }

    /// The store's clock, read off an answer: retention runs on it, so it is compared with ours.
    pub async fn server_time(&self, bucket: &str) -> Result<DateTime<Utc>, ObjectStoreError> {
        const OPERATION: &str = "HeadBucket";
        let call = Call::new(OPERATION, Method::HEAD, bucket);
        let response = missing_is_not_found(self.send(call).await, || bucket.to_owned())?;
        response
            .headers()
            .get(reqwest::header::DATE)
            .and_then(|value| value.to_str().ok())
            .and_then(|text| DateTime::parse_from_rfc2822(text).ok())
            .map(|at| at.with_timezone(&Utc))
            .ok_or_else(|| malformed(OPERATION, "no Date header"))
    }
}

#[async_trait]
impl ObjectStore for S3Client {
    async fn put(
        &self,
        bucket: &str,
        key: &str,
        bytes: Bytes,
        sha256: [u8; 32],
        lock: LockMode,
    ) -> Result<VersionId, ObjectStoreError> {
        let actual: [u8; 32] = Sha256::digest(&bytes).into();
        if actual != sha256 {
            return Err(ObjectStoreError::ChecksumMismatch {
                key: key.to_owned(),
                side: "the digest its writer gave",
            });
        }
        let mut call = Call::new("PutObject", Method::PUT, bucket)
            .key(key)
            .header("content-type", "application/octet-stream")
            .header("content-md5", &md5_base64(&bytes))
            .header("x-amz-checksum-sha256", &BASE64.encode(sha256));
        if let Some(retention) = lock.retention() {
            call = call
                .header("x-amz-object-lock-mode", retention.mode.as_str())
                .header(
                    "x-amz-object-lock-retain-until-date",
                    &timestamp(retention.until),
                );
        }
        call.body = bytes;
        call.sha256 = Some(sha256);
        let response = missing_is_not_found(self.send(call).await, || bucket.to_owned())?;
        Ok(version_of(response.headers()))
    }

    async fn get(&self, bucket: &str, key: &str) -> Result<Bytes, ObjectStoreError> {
        self.read(bucket, key, None).await
    }

    async fn head(&self, bucket: &str, key: &str) -> Result<Option<ObjectMeta>, ObjectStoreError> {
        const OPERATION: &str = "HeadObject";
        let call = Call::new(OPERATION, Method::HEAD, bucket)
            .key(key)
            .header("x-amz-checksum-mode", "ENABLED");
        let response = match self.send(call).await {
            Ok(response) => response,
            Err(ObjectStoreError::Refused {
                status: NOT_FOUND, ..
            }) => return Ok(None),
            Err(e) => return Err(e),
        };
        let headers = response.headers();
        let text = |name: &str| headers.get(name).and_then(|value| value.to_str().ok());
        let size = text("content-length")
            .and_then(|length| length.parse().ok())
            .ok_or_else(|| malformed(OPERATION, "no content length"))?;
        let retention = match (
            text("x-amz-object-lock-mode").and_then(RetentionMode::parse),
            text("x-amz-object-lock-retain-until-date").and_then(parse_timestamp),
        ) {
            (Some(mode), Some(until)) => Some(Retention { mode, until }),
            _ => None,
        };
        Ok(Some(ObjectMeta {
            size,
            version_id: version_of(headers),
            sha256: checksum_of(headers),
            retention,
            legal_hold: text("x-amz-object-lock-legal-hold") == Some("ON"),
        }))
    }

    fn presign_get(&self, bucket: &str, key: &str, ttl: Duration) -> Result<Url, ObjectStoreError> {
        self.presign_get_at(bucket, key, ttl, Utc::now())
    }

    async fn head_bucket(&self, bucket: &str) -> Result<(), ObjectStoreError> {
        let call = Call::new("HeadBucket", Method::HEAD, bucket);
        missing_is_not_found(self.send(call).await, || bucket.to_owned()).map(drop)
    }
}

/// `/bucket` or `/bucket/key`, encoded as it is signed and sent.
fn object_path(bucket: &str, key: Option<&str>) -> Result<String, ObjectStoreError> {
    let bytes = bucket.as_bytes();
    let edge =
        |byte: Option<&u8>| byte.is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit());
    let sound = (3..=63).contains(&bytes.len())
        && edge(bytes.first())
        && edge(bytes.last())
        && bytes
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-');
    if !sound {
        return Err(ObjectStoreError::Config(format!(
            "`{bucket}` is not a bucket name"
        )));
    }
    match key {
        None => Ok(format!("/{bucket}")),
        Some(key) => {
            check_key(key)?;
            Ok(format!("/{bucket}/{}", uri_encode(key, true)))
        }
    }
}

fn unavailable(error: reqwest::Error) -> ObjectStoreError {
    ObjectStoreError::Unavailable(error.without_url().to_string())
}

fn malformed(operation: &'static str, reason: &str) -> ObjectStoreError {
    ObjectStoreError::Malformed {
        operation,
        reason: reason.to_owned(),
    }
}

fn missing_is_not_found<T>(
    result: Result<T, ObjectStoreError>,
    resource: impl FnOnce() -> String,
) -> Result<T, ObjectStoreError> {
    match result {
        Err(ObjectStoreError::Refused {
            status: NOT_FOUND, ..
        }) => Err(ObjectStoreError::NotFound(resource())),
        other => other,
    }
}

/// The `Code` of an S3 error document; empty for an answer with no body (every HEAD).
async fn error_code(mut response: Response) -> String {
    let mut body = Vec::new();
    while body.len() < ERROR_BODY_LIMIT {
        match response.chunk().await {
            Ok(Some(chunk)) => body.extend_from_slice(&chunk),
            _ => break,
        }
    }
    element(&String::from_utf8_lossy(&body), "Code").unwrap_or_default()
}

fn version_of(headers: &HeaderMap) -> VersionId {
    VersionId(
        headers
            .get("x-amz-version-id")
            .and_then(|value| value.to_str().ok())
            .filter(|version| !version.is_empty())
            .unwrap_or(VersionId::UNVERSIONED)
            .to_owned(),
    )
}

/// The whole-object SHA-256 the store recorded. A multipart object carries a digest of digests, which is
/// not comparable with the bytes and is left out.
fn checksum_of(headers: &HeaderMap) -> Option<[u8; 32]> {
    let text = headers.get("x-amz-checksum-sha256")?.to_str().ok()?;
    BASE64.decode(text).ok()?.try_into().ok()
}

fn timestamp(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn parse_timestamp(text: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(text)
        .ok()
        .map(|at| at.with_timezone(&Utc))
}

fn md5_base64(bytes: &[u8]) -> String {
    use md5::Digest as _;
    BASE64.encode(md5::Md5::digest(bytes))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use zeroize::Zeroizing;

    use super::*;

    fn credential() -> S3Credential {
        S3Credential {
            access_key_id: "AKIAIOSFODNN7EXAMPLE".to_owned(),
            secret_access_key: Zeroizing::new(
                "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_owned(),
            ),
        }
    }

    fn client(endpoint: &str) -> S3Client {
        S3Client::new(&Url::parse(endpoint).unwrap(), credential()).unwrap()
    }

    #[test]
    fn the_host_header_carries_a_port_only_when_it_is_not_the_default() {
        assert_eq!(client("http://s3guard:9000").host, "s3guard:9000");
        assert_eq!(client("http://s3guard:9000").origin, "http://s3guard:9000");
        assert_eq!(client("https://s3guard.akr.svc/").host, "s3guard.akr.svc");
        assert_eq!(client("http://s3guard:80").host, "s3guard");
    }

    #[test]
    fn an_endpoint_with_a_path_or_another_scheme_is_refused() {
        for endpoint in [
            "http://s3guard:9000/akr-evidence",
            "http://s3guard:9000/?x=1",
            "ftp://s3guard",
        ] {
            let refused = S3Client::new(&Url::parse(endpoint).unwrap(), credential());
            assert!(
                matches!(refused, Err(ObjectStoreError::Config(_))),
                "{endpoint}"
            );
        }
    }

    #[test]
    fn a_path_is_encoded_once_and_a_bad_name_never_leaves() {
        assert_eq!(object_path("akr-evidence", None).unwrap(), "/akr-evidence");
        assert_eq!(
            object_path("akr-evidence", Some("cases/2026/a b+c.pdf")).unwrap(),
            "/akr-evidence/cases/2026/a%20b%2Bc.pdf"
        );
        for bucket in ["", "ab", "Akr", "akr_evidence", "-akr", "akr-", "a/b"] {
            assert!(object_path(bucket, None).is_err(), "{bucket:?}");
        }
        assert!(matches!(
            object_path("akr-evidence", Some("a/../b")),
            Err(ObjectStoreError::InvalidKey(_))
        ));
    }

    #[test]
    fn a_presigned_url_is_a_get_on_the_endpoint_with_the_signature_last() {
        let client = client("http://s3guard:9000");
        let at = DateTime::parse_from_rfc3339("2026-10-11T08:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let url = client
            .presign_get_at(
                "akr-exports",
                "reports/a b.pdf",
                Duration::from_secs(120),
                at,
            )
            .unwrap();
        assert_eq!(url.host_str(), Some("s3guard"));
        assert_eq!(url.port(), Some(9000));
        assert_eq!(url.path(), "/akr-exports/reports/a%20b.pdf");
        let query = url.query().unwrap();
        assert!(query.contains("X-Amz-Date=20261011T080000Z"), "{query}");
        assert!(query.contains("X-Amz-Expires=120"), "{query}");
        assert!(query.contains("X-Amz-SignedHeaders=host"), "{query}");
        let (_, signature) = query.rsplit_once("&X-Amz-Signature=").unwrap();
        assert_eq!(signature.len(), 64);
        assert!(!query.contains("wJalrXUtnFEMI"));

        assert!(matches!(
            client.presign_get("akr-exports", "a", Duration::from_secs(301)),
            Err(ObjectStoreError::PresignTtl(301))
        ));
    }

    #[test]
    fn a_checksum_header_is_read_only_when_it_is_a_whole_object_digest() {
        let digest: [u8; 32] = Sha256::digest(b"abc").into();
        let mut headers = HeaderMap::new();
        assert_eq!(checksum_of(&headers), None);
        headers.insert(
            "x-amz-checksum-sha256",
            BASE64.encode(digest).parse().unwrap(),
        );
        assert_eq!(checksum_of(&headers), Some(digest));
        headers.insert(
            "x-amz-checksum-sha256",
            format!("{}-3", BASE64.encode(digest)).parse().unwrap(),
        );
        assert_eq!(checksum_of(&headers), None);
    }

    #[test]
    fn an_unversioned_answer_names_the_null_version() {
        let mut headers = HeaderMap::new();
        assert_eq!(version_of(&headers).as_str(), "null");
        headers.insert("x-amz-version-id", "3HL4kqtJlcpXroDTDmJ".parse().unwrap());
        assert_eq!(version_of(&headers).as_str(), "3HL4kqtJlcpXroDTDmJ");
    }

    #[test]
    fn a_transfer_digest_is_the_base64_md5() {
        assert_eq!(md5_base64(b""), "1B2M2Y8AsgTpgAmY7PhCfg==");
    }

    #[test]
    fn the_debug_form_holds_no_secret() {
        let shown = format!("{:?}", client("http://s3guard:9000"));
        assert!(shown.contains("AKIAIOSFODNN7EXAMPLE"));
        assert!(!shown.contains("wJalr"));
    }
}
