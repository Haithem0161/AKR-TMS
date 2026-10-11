//! What the guard passes and what it refuses.
//!
//! [`evaluate`] is a pure function of the request line and headers. It names the one operation a request
//! is, or the rule that refuses it. The seven rules that answer published advisories run first, so a
//! refused request is counted under the flaw it resembles; the allow-list runs last and refuses whatever
//! is left that the product does not use.
//!
//! The guard and the store must read a request the same way, or one could approve what the other then
//! runs as something else. So a path the two might parse differently (an encoded slash, a dot segment, an
//! empty segment, a broken escape) is refused, not interpreted.

use std::collections::BTreeSet;

use akr_objectstore::buckets;
use http::{HeaderMap, Method};
use serde::Deserialize;

/// The table that ships with the store build.
pub const RULES_TOML: &str = include_str!("../rules.toml");

const UNSIGNED_TRAILER: &str = "STREAMING-UNSIGNED-PAYLOAD";
const SIGV4: &str = "AWS4-HMAC-SHA256";

/// The query parameters of a presigned request.
const PRESIGN: [&str; 6] = [
    "X-Amz-Algorithm",
    "X-Amz-Credential",
    "X-Amz-Date",
    "X-Amz-Expires",
    "X-Amz-SignedHeaders",
    "X-Amz-Signature",
];

/// Query parameters that carry a credential or a signature, in lower case.
const QUERY_CREDENTIALS: [&str; 6] = [
    "x-amz-credential",
    "x-amz-signature",
    "x-amz-algorithm",
    "x-amz-security-token",
    "awsaccesskeyid",
    "signature",
];

/// A request may carry each of these once.
const SINGLE_HEADERS: [&str; 4] = [
    "authorization",
    "host",
    "content-length",
    "x-amz-content-sha256",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rule {
    S3Select,
    SnowballAutoExtract,
    UnsignedPayloadTrailer,
    QueryCredentialsOnWrite,
    StoreInternalHeader,
    UnsignedAmzHeader,
    AdminPath,
    OperationNotAllowed,
    DeleteOutsideQuarantine,
    BucketWriteOutsideInstaller,
}

impl Rule {
    pub const ALL: [Rule; 10] = [
        Rule::S3Select,
        Rule::SnowballAutoExtract,
        Rule::UnsignedPayloadTrailer,
        Rule::QueryCredentialsOnWrite,
        Rule::StoreInternalHeader,
        Rule::UnsignedAmzHeader,
        Rule::AdminPath,
        Rule::OperationNotAllowed,
        Rule::DeleteOutsideQuarantine,
        Rule::BucketWriteOutsideInstaller,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Rule::S3Select => "s3_select",
            Rule::SnowballAutoExtract => "snowball_auto_extract",
            Rule::UnsignedPayloadTrailer => "unsigned_payload_trailer",
            Rule::QueryCredentialsOnWrite => "query_credentials_on_write",
            Rule::StoreInternalHeader => "store_internal_header",
            Rule::UnsignedAmzHeader => "unsigned_amz_header",
            Rule::AdminPath => "admin_path",
            Rule::OperationNotAllowed => "operation_not_allowed",
            Rule::DeleteOutsideQuarantine => "delete_outside_quarantine",
            Rule::BucketWriteOutsideInstaller => "bucket_write_outside_installer",
        }
    }
}

/// The S3 operations the product uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    HeadBucket,
    ListObjectVersions,
    CreateBucket,
    GetObjectLockConfiguration,
    PutObjectLockConfiguration,
    GetObject,
    HeadObject,
    GetObjectRetention,
    GetObjectLegalHold,
    PutObject,
    CopyObject,
    PutObjectRetention,
    PutObjectLegalHold,
    CreateMultipartUpload,
    UploadPart,
    CompleteMultipartUpload,
    AbortMultipartUpload,
    DeleteObject,
}

impl Operation {
    /// The request carries a body, so it must say how long the body is.
    pub fn has_body(self) -> bool {
        matches!(
            self,
            Operation::CreateBucket
                | Operation::PutObjectLockConfiguration
                | Operation::PutObject
                | Operation::CopyObject
                | Operation::PutObjectRetention
                | Operation::PutObjectLegalHold
                | Operation::CreateMultipartUpload
                | Operation::UploadPart
                | Operation::CompleteMultipartUpload
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequestView<'a> {
    pub method: &'a Method,
    /// The path as it arrived, still percent-encoded.
    pub raw_path: &'a str,
    pub raw_query: Option<&'a str>,
    pub headers: &'a HeaderMap,
    /// The peer is in the installer's network.
    pub from_installer: bool,
}

enum Target<'a> {
    Root,
    Bucket,
    Object { bucket: &'a str },
}

pub fn evaluate(request: &RequestView<'_>) -> Result<Operation, Rule> {
    let path = decode_path(request.raw_path).ok_or(Rule::OperationNotAllowed)?;
    let names = query_names(request.raw_query).ok_or(Rule::OperationNotAllowed)?;
    let headers = request.headers;
    let method = request.method;

    let first_segment = path[1..].split('/').next().unwrap_or_default();
    if first_segment == "minio"
        || names.iter().any(|name| name.eq_ignore_ascii_case("action"))
        || (path == "/" && method == Method::POST)
    {
        return Err(Rule::AdminPath);
    }

    if names
        .iter()
        .any(|name| name.eq_ignore_ascii_case("select") || name.eq_ignore_ascii_case("select-type"))
    {
        return Err(Rule::S3Select);
    }

    if headers
        .keys()
        .any(|name| name.as_str().contains("snowball"))
    {
        return Err(Rule::SnowballAutoExtract);
    }

    let payload_hash = headers
        .get("x-amz-content-sha256")
        .map(|value| value.to_str().unwrap_or_default().trim());
    if payload_hash.is_some_and(|hash| {
        hash.get(..UNSIGNED_TRAILER.len())
            .is_some_and(|start| start.eq_ignore_ascii_case(UNSIGNED_TRAILER))
    }) {
        return Err(Rule::UnsignedPayloadTrailer);
    }

    let reads = method == Method::GET || method == Method::HEAD;
    let query_signed = names.iter().any(|name| {
        QUERY_CREDENTIALS
            .iter()
            .any(|credential| name.eq_ignore_ascii_case(credential))
    });
    if query_signed && !reads {
        return Err(Rule::QueryCredentialsOnWrite);
    }

    if headers.keys().any(|name| {
        name.as_str().starts_with("x-minio-") || name.as_str() == "x-amz-bucket-replication-status"
    }) {
        return Err(Rule::StoreInternalHeader);
    }

    if SINGLE_HEADERS
        .iter()
        .any(|name| headers.get_all(*name).iter().count() > 1)
    {
        return Err(Rule::OperationNotAllowed);
    }
    let signed = signed_headers(headers, request.raw_query, query_signed)?;
    if headers
        .keys()
        .map(|name| name.as_str())
        .any(|name| name.starts_with("x-amz-") && !signed.contains(name))
    {
        return Err(Rule::UnsignedAmzHeader);
    }

    if payload_hash.is_some_and(|hash| !is_hex_sha256(hash)) {
        return Err(Rule::OperationNotAllowed);
    }
    if headers.contains_key("transfer-encoding") {
        return Err(Rule::OperationNotAllowed);
    }

    let operation = match target(&path)? {
        Target::Root => return Err(Rule::OperationNotAllowed),
        Target::Bucket => bucket_operation(method, &names, request.from_installer)?,
        Target::Object { bucket } => object_operation(method, &names, headers, bucket)?,
    };
    if operation.has_body() && payload_hash.is_none() {
        return Err(Rule::OperationNotAllowed);
    }
    Ok(operation)
}

fn bucket_operation(
    method: &Method,
    names: &BTreeSet<String>,
    from_installer: bool,
) -> Result<Operation, Rule> {
    const LISTING: [&str; 7] = [
        "versions",
        "prefix",
        "key-marker",
        "version-id-marker",
        "max-keys",
        "delimiter",
        "encoding-type",
    ];
    let installer = |operation| {
        if from_installer {
            Ok(operation)
        } else {
            Err(Rule::BucketWriteOutsideInstaller)
        }
    };
    let exactly = |wanted: &[&str]| exactly(names, wanted);

    if method == Method::HEAD && names.is_empty() {
        Ok(Operation::HeadBucket)
    } else if method == Method::GET && names.contains("versions") && only(names, &LISTING) {
        Ok(Operation::ListObjectVersions)
    } else if method == Method::GET && exactly(&["object-lock"]) {
        installer(Operation::GetObjectLockConfiguration)
    } else if method == Method::PUT && names.is_empty() {
        installer(Operation::CreateBucket)
    } else if method == Method::PUT && exactly(&["object-lock"]) {
        installer(Operation::PutObjectLockConfiguration)
    } else {
        Err(Rule::OperationNotAllowed)
    }
}

fn object_operation(
    method: &Method,
    names: &BTreeSet<String>,
    headers: &HeaderMap,
    bucket: &str,
) -> Result<Operation, Rule> {
    let exactly = |wanted: &[&str]| exactly(names, wanted);
    let within = |allowed: &[&str]| only(names, allowed);
    let read: Vec<&str> = [
        "versionId",
        "response-content-type",
        "response-content-disposition",
    ]
    .into_iter()
    .chain(PRESIGN)
    .collect();
    let head: Vec<&str> = ["versionId"].into_iter().chain(PRESIGN).collect();

    let operation = if method == Method::GET {
        if names.contains("retention") && within(&["retention", "versionId"]) {
            Operation::GetObjectRetention
        } else if names.contains("legal-hold") && within(&["legal-hold", "versionId"]) {
            Operation::GetObjectLegalHold
        } else if within(&read) {
            Operation::GetObject
        } else {
            return Err(Rule::OperationNotAllowed);
        }
    } else if method == Method::HEAD {
        if within(&head) {
            Operation::HeadObject
        } else {
            return Err(Rule::OperationNotAllowed);
        }
    } else if method == Method::PUT {
        if names.is_empty() {
            if headers.contains_key("x-amz-copy-source") {
                Operation::CopyObject
            } else {
                Operation::PutObject
            }
        } else if names.contains("retention") && within(&["retention", "versionId"]) {
            Operation::PutObjectRetention
        } else if names.contains("legal-hold") && within(&["legal-hold", "versionId"]) {
            Operation::PutObjectLegalHold
        } else if exactly(&["partNumber", "uploadId"]) {
            Operation::UploadPart
        } else {
            return Err(Rule::OperationNotAllowed);
        }
    } else if method == Method::POST {
        if exactly(&["uploads"]) {
            Operation::CreateMultipartUpload
        } else if exactly(&["uploadId"]) {
            Operation::CompleteMultipartUpload
        } else {
            return Err(Rule::OperationNotAllowed);
        }
    } else if method == Method::DELETE {
        if exactly(&["uploadId"]) {
            Operation::AbortMultipartUpload
        } else if within(&["versionId"]) {
            if bucket != buckets::QUARANTINE {
                return Err(Rule::DeleteOutsideQuarantine);
            }
            Operation::DeleteObject
        } else {
            return Err(Rule::OperationNotAllowed);
        }
    } else {
        return Err(Rule::OperationNotAllowed);
    };
    Ok(operation)
}

fn only(names: &BTreeSet<String>, allowed: &[&str]) -> bool {
    names.iter().all(|name| allowed.contains(&name.as_str()))
}

fn exactly(names: &BTreeSet<String>, wanted: &[&str]) -> bool {
    names.len() == wanted.len() && only(names, wanted)
}

/// The path with its escapes resolved, or `None` for a path the store might read another way.
fn decode_path(raw: &str) -> Option<String> {
    if !raw.starts_with('/') || raw.contains('\\') {
        return None;
    }
    let lower = raw.to_ascii_lowercase();
    if ["%2f", "%5c", "%00"]
        .iter()
        .any(|escape| lower.contains(escape))
    {
        return None;
    }
    let decoded = percent_decode(raw, false)?;
    (!decoded.chars().any(char::is_control)).then_some(decoded)
}

fn target(path: &str) -> Result<Target<'_>, Rule> {
    let rest = &path[1..];
    if rest.is_empty() {
        return Ok(Target::Root);
    }
    let (bucket, key) = match rest.split_once('/') {
        Some((bucket, key)) => (bucket, key),
        None => (rest, ""),
    };
    let bytes = bucket.as_bytes();
    let edge =
        |byte: Option<&u8>| byte.is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit());
    let named = (3..=63).contains(&bytes.len())
        && edge(bytes.first())
        && edge(bytes.last())
        && bytes
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-');
    if !named {
        return Err(Rule::OperationNotAllowed);
    }
    if key.is_empty() {
        return Ok(Target::Bucket);
    }
    if key
        .split('/')
        .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err(Rule::OperationNotAllowed);
    }
    Ok(Target::Object { bucket })
}

/// The names of the query parameters, decoded. `None` for a query with a broken escape.
fn query_names(raw: Option<&str>) -> Option<BTreeSet<String>> {
    let mut names = BTreeSet::new();
    for pair in raw
        .unwrap_or_default()
        .split('&')
        .filter(|pair| !pair.is_empty())
    {
        let name = pair.split_once('=').map_or(pair, |(name, _)| name);
        names.insert(percent_decode(name, true)?);
    }
    Some(names)
}

fn query_value(raw: Option<&str>, wanted: &str) -> Option<String> {
    raw?.split('&').find_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        (percent_decode(name, true)? == wanted)
            .then(|| percent_decode(value, true))
            .flatten()
    })
}

fn percent_decode(text: &str, plus_is_space: bool) -> Option<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' => {
                let pair = text.get(i + 1..i + 3)?;
                out.push(u8::from_str_radix(pair, 16).ok()?);
                i += 3;
            }
            b'+' if plus_is_space => {
                out.push(b' ');
                i += 1;
            }
            byte => {
                out.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8(out).ok()
}

fn is_hex_sha256(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|b| b.is_ascii_hexdigit())
}

/// The headers the request's signature covers. A request signed two ways, or signed with anything but
/// Signature Version 4, is refused: the product sends neither.
fn signed_headers(
    headers: &HeaderMap,
    raw_query: Option<&str>,
    query_signed: bool,
) -> Result<BTreeSet<String>, Rule> {
    let list = match headers.get("authorization") {
        Some(_) if query_signed => return Err(Rule::OperationNotAllowed),
        Some(value) => {
            let value = value.to_str().map_err(|_| Rule::OperationNotAllowed)?;
            if !value.starts_with(SIGV4) {
                return Err(Rule::OperationNotAllowed);
            }
            value
                .split_once("SignedHeaders=")
                .map(|(_, rest)| rest.split(',').next().unwrap_or_default().trim().to_owned())
                .ok_or(Rule::OperationNotAllowed)?
        }
        None if query_signed => {
            if query_value(raw_query, "X-Amz-Algorithm").as_deref() != Some(SIGV4) {
                return Err(Rule::OperationNotAllowed);
            }
            query_value(raw_query, "X-Amz-SignedHeaders").ok_or(Rule::OperationNotAllowed)?
        }
        None => String::new(),
    };
    Ok(list
        .split(';')
        .filter(|name| !name.is_empty())
        .map(str::to_ascii_lowercase)
        .collect())
}

#[derive(Debug, Deserialize)]
pub struct RuleBook {
    /// The store build the table was checked against.
    pub store: String,
    #[serde(rename = "rule")]
    pub rules: Vec<RuleEntry>,
}

#[derive(Debug, Deserialize)]
pub struct RuleEntry {
    pub id: String,
    pub rejects: String,
    pub advisories: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum RuleBookError {
    #[error("rules.toml does not parse: {0}")]
    Parse(String),
    #[error("rules.toml and the guard name different rules: {0}")]
    Drift(String),
}

/// Reads the shipped table and confirms it names exactly the rules the code enforces.
pub fn rule_book() -> Result<RuleBook, RuleBookError> {
    let book: RuleBook =
        toml::from_str(RULES_TOML).map_err(|e| RuleBookError::Parse(e.to_string()))?;
    let listed: Vec<&str> = book.rules.iter().map(|rule| rule.id.as_str()).collect();
    let enforced: Vec<&str> = Rule::ALL.iter().map(|rule| rule.id()).collect();
    if listed != enforced {
        return Err(RuleBookError::Drift(format!(
            "the file lists {listed:?}, the code enforces {enforced:?}"
        )));
    }
    Ok(book)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use http::HeaderValue;

    use super::*;

    const EMPTY: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

    /// A request as the product's client signs it: a hex body digest, and every `x-amz-*` header named
    /// in SignedHeaders.
    fn signed(extra: &[(&str, &str)]) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("host", HeaderValue::from_static("s3guard:9000"));
        headers.insert("x-amz-content-sha256", HeaderValue::from_static(EMPTY));
        headers.insert("x-amz-date", HeaderValue::from_static("20261011T080000Z"));
        for (name, value) in extra {
            headers.insert(
                http::HeaderName::from_bytes(name.as_bytes()).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }
        let mut names: Vec<&str> = headers.keys().map(|name| name.as_str()).collect();
        names.sort_unstable();
        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential=akr-writer/20261011/us-east-1/s3/aws4_request, \
             SignedHeaders={}, Signature={EMPTY}",
            names.join(";")
        );
        headers.insert(
            "authorization",
            HeaderValue::from_str(&authorization).unwrap(),
        );
        headers
    }

    fn check(
        method: Method,
        target: &str,
        headers: &HeaderMap,
        installer: bool,
    ) -> Result<Operation, Rule> {
        let (path, query) = match target.split_once('?') {
            Some((path, query)) => (path, Some(query)),
            None => (target, None),
        };
        evaluate(&RequestView {
            method: &method,
            raw_path: path,
            raw_query: query,
            headers,
            from_installer: installer,
        })
    }

    fn app(method: Method, target: &str) -> Result<Operation, Rule> {
        check(method, target, &signed(&[]), false)
    }

    #[test]
    fn the_file_and_the_code_name_the_same_rules() {
        let book = rule_book().unwrap();
        assert_eq!(book.store, "Silo RELEASE.2026-09-16T00-00-00Z");
        assert_eq!(book.rules.len(), Rule::ALL.len());
        let ids: BTreeSet<&str> = Rule::ALL.iter().map(|rule| rule.id()).collect();
        assert_eq!(ids.len(), Rule::ALL.len());
        for rule in &book.rules[..7] {
            assert!(!rule.advisories.is_empty(), "{}", rule.id);
            assert!(!rule.rejects.is_empty());
        }
    }

    #[test]
    fn every_operation_the_product_uses_passes() {
        use Operation::*;
        let cases = [
            (Method::HEAD, "/akr-evidence", HeadBucket),
            (Method::HEAD, "/akr-evidence/", HeadBucket),
            (
                Method::GET,
                "/akr-quarantine?versions=&prefix=staging%2F",
                ListObjectVersions,
            ),
            (Method::GET, "/akr-evidence/cases/1/a%20b.pdf", GetObject),
            (
                Method::GET,
                "/akr-evidence/a?versionId=3HL4kqtJlcpXroDTDmJ",
                GetObject,
            ),
            (Method::HEAD, "/akr-evidence/a", HeadObject),
            (
                Method::GET,
                "/akr-evidence/a?retention=&versionId=v1",
                GetObjectRetention,
            ),
            (
                Method::GET,
                "/akr-evidence/a?legal-hold=",
                GetObjectLegalHold,
            ),
            (Method::PUT, "/akr-evidence/a", PutObject),
            (
                Method::PUT,
                "/akr-evidence/a?retention=",
                PutObjectRetention,
            ),
            (
                Method::PUT,
                "/akr-evidence/a?legal-hold=&versionId=v1",
                PutObjectLegalHold,
            ),
            (
                Method::POST,
                "/akr-evidence/a?uploads=",
                CreateMultipartUpload,
            ),
            (
                Method::PUT,
                "/akr-evidence/a?partNumber=1&uploadId=u",
                UploadPart,
            ),
            (
                Method::POST,
                "/akr-evidence/a?uploadId=u",
                CompleteMultipartUpload,
            ),
            (
                Method::DELETE,
                "/akr-evidence/a?uploadId=u",
                AbortMultipartUpload,
            ),
            (Method::DELETE, "/akr-quarantine/staging/a", DeleteObject),
            (
                Method::DELETE,
                "/akr-quarantine/staging/a?versionId=null",
                DeleteObject,
            ),
        ];
        for (method, target, operation) in cases {
            assert_eq!(
                app(method.clone(), target),
                Ok(operation),
                "{method} {target}"
            );
        }

        let copy = signed(&[("x-amz-copy-source", "/akr-quarantine/staging/a")]);
        assert_eq!(
            check(Method::PUT, "/akr-evidence/a", &copy, false),
            Ok(CopyObject)
        );
    }

    #[test]
    fn a_presigned_get_passes_and_a_presigned_write_does_not() {
        let presigned = "/akr-exports/r.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256\
            &X-Amz-Credential=akr-presign%2F20261011%2Fus-east-1%2Fs3%2Faws4_request\
            &X-Amz-Date=20261011T080000Z&X-Amz-Expires=120&X-Amz-SignedHeaders=host&X-Amz-Signature=ab";
        let mut browser = HeaderMap::new();
        browser.insert("host", HeaderValue::from_static("s3guard:9000"));

        assert_eq!(
            check(Method::GET, presigned, &browser, false),
            Ok(Operation::GetObject)
        );
        assert_eq!(
            check(Method::HEAD, presigned, &browser, false),
            Ok(Operation::HeadObject)
        );
        for method in [Method::PUT, Method::POST, Method::DELETE] {
            assert_eq!(
                check(method.clone(), presigned, &browser, false),
                Err(Rule::QueryCredentialsOnWrite),
                "{method}"
            );
        }
        assert_eq!(
            check(
                Method::PUT,
                "/akr-exports/r.pdf?AWSAccessKeyId=k&Signature=s&Expires=1",
                &browser,
                false
            ),
            Err(Rule::QueryCredentialsOnWrite)
        );
    }

    #[test]
    fn each_advisory_shape_is_refused_under_its_rule() {
        assert_eq!(
            app(Method::POST, "/akr-evidence/a?select&select-type=2"),
            Err(Rule::S3Select)
        );
        assert_eq!(
            app(Method::POST, "/akr-evidence/a?Select=&select-type=2"),
            Err(Rule::S3Select)
        );

        let snowball = signed(&[("x-amz-meta-snowball-auto-extract", "true")]);
        assert_eq!(
            check(Method::PUT, "/akr-evidence/a.tar", &snowball, false),
            Err(Rule::SnowballAutoExtract)
        );

        let mut trailer = signed(&[]);
        trailer.insert(
            "x-amz-content-sha256",
            HeaderValue::from_static("STREAMING-UNSIGNED-PAYLOAD-TRAILER"),
        );
        assert_eq!(
            check(Method::PUT, "/akr-evidence/a", &trailer, false),
            Err(Rule::UnsignedPayloadTrailer)
        );

        for header in ["x-minio-source-replication-request", "x-minio-anything"] {
            let internal = signed(&[(header, "true")]);
            assert_eq!(
                check(Method::PUT, "/akr-evidence/a", &internal, false),
                Err(Rule::StoreInternalHeader),
                "{header}"
            );
        }
        let replica = signed(&[("x-amz-bucket-replication-status", "REPLICA")]);
        assert_eq!(
            check(Method::PUT, "/akr-evidence/a", &replica, false),
            Err(Rule::StoreInternalHeader)
        );

        let mut unsigned_copy = signed(&[]);
        unsigned_copy.insert(
            "x-amz-copy-source",
            HeaderValue::from_static("/akr-evidence/other"),
        );
        assert_eq!(
            check(Method::PUT, "/akr-evidence/a", &unsigned_copy, false),
            Err(Rule::UnsignedAmzHeader)
        );

        for (method, target) in [
            (Method::GET, "/minio/admin/v3/info"),
            (Method::POST, "/minio/admin/v3/update"),
            (Method::GET, "/minio/health/live"),
            (Method::GET, "/minio/v2/metrics/cluster"),
            (Method::POST, "/minio/storage/data/v63/readall"),
            (Method::GET, "/minio"),
            (Method::GET, "/%6Dinio/admin/v3/info"),
            (Method::POST, "/"),
            (
                Method::POST,
                "/?Action=AssumeRoleWithWebIdentity&Version=2011-06-15",
            ),
            (Method::GET, "/akr-evidence/a?action=AssumeRole"),
        ] {
            assert_eq!(
                app(method.clone(), target),
                Err(Rule::AdminPath),
                "{method} {target}"
            );
        }
    }

    #[test]
    fn an_anonymous_request_with_an_amz_header_names_no_signed_list() {
        let mut headers = HeaderMap::new();
        headers.insert("host", HeaderValue::from_static("s3guard:9000"));
        headers.insert("x-amz-date", HeaderValue::from_static("20261011T080000Z"));
        assert_eq!(
            check(Method::GET, "/akr-exports/a", &headers, false),
            Err(Rule::UnsignedAmzHeader)
        );

        let presigned = "/akr-exports/r.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=c\
            &X-Amz-Date=d&X-Amz-Expires=120&X-Amz-SignedHeaders=host&X-Amz-Signature=ab";
        headers.remove("x-amz-date");
        headers.insert(
            "x-amz-copy-source",
            HeaderValue::from_static("/akr-evidence/other"),
        );
        assert_eq!(
            check(Method::GET, presigned, &headers, false),
            Err(Rule::UnsignedAmzHeader)
        );
    }

    #[test]
    fn a_delete_reaches_the_quarantine_bucket_only() {
        for target in [
            "/akr-evidence/a",
            "/akr-evidence/a?versionId=v1",
            "/akr-anchors/2026/a",
            "/akr-models/m",
            "/akr-exports/r.pdf",
            "/akr-quarantine-2/a",
        ] {
            assert_eq!(
                check(Method::DELETE, target, &signed(&[]), true),
                Err(Rule::DeleteOutsideQuarantine),
                "{target}"
            );
        }
    }

    #[test]
    fn bucket_writes_pass_from_the_installer_only() {
        for (method, target, operation) in [
            (Method::PUT, "/akr-evidence", Operation::CreateBucket),
            (
                Method::PUT,
                "/akr-evidence?object-lock=",
                Operation::PutObjectLockConfiguration,
            ),
            (
                Method::GET,
                "/akr-evidence?object-lock=",
                Operation::GetObjectLockConfiguration,
            ),
        ] {
            assert_eq!(
                check(method.clone(), target, &signed(&[]), true),
                Ok(operation),
                "{method} {target}"
            );
            assert_eq!(
                check(method.clone(), target, &signed(&[]), false),
                Err(Rule::BucketWriteOutsideInstaller),
                "{method} {target}"
            );
        }
    }

    #[test]
    fn what_the_product_does_not_use_is_refused_for_everyone() {
        for (method, target) in [
            (Method::GET, "/"),
            (Method::DELETE, "/akr-evidence"),
            (Method::GET, "/akr-evidence"),
            (Method::GET, "/akr-evidence?list-type=2"),
            (Method::GET, "/akr-evidence?policy="),
            (Method::PUT, "/akr-evidence?policy="),
            (Method::PUT, "/akr-evidence?versioning="),
            (Method::PUT, "/akr-evidence?lifecycle="),
            (Method::PUT, "/akr-evidence?replication="),
            (Method::PUT, "/akr-evidence?notification="),
            (Method::POST, "/akr-evidence?delete="),
            (Method::POST, "/akr-evidence"),
            (Method::GET, "/akr-evidence/a?tagging="),
            (Method::GET, "/akr-evidence/a?acl="),
            (Method::GET, "/akr-evidence/a?uploadId=u"),
            (Method::GET, "/akr-evidence/a?attributes="),
            (Method::PUT, "/akr-evidence/a?tagging="),
            (Method::PUT, "/akr-evidence/a?acl="),
            (Method::POST, "/akr-evidence/a?restore="),
            (Method::POST, "/akr-evidence/a"),
            (Method::DELETE, "/akr-quarantine/a?tagging="),
            (Method::OPTIONS, "/akr-evidence/a"),
            (Method::PATCH, "/akr-evidence/a"),
            (Method::GET, "/Akr-Evidence/a"),
            (Method::GET, "/ab/a"),
        ] {
            assert_eq!(
                check(method.clone(), target, &signed(&[]), true),
                Err(Rule::OperationNotAllowed),
                "{method} {target}"
            );
        }
    }

    #[test]
    fn a_path_the_store_could_read_another_way_is_refused() {
        for target in [
            "/akr-quarantine/../akr-evidence/a",
            "/akr-quarantine/%2e%2e/akr-evidence/a",
            "/akr-quarantine/./a",
            "/akr-quarantine//a",
            "/akr-quarantine/a/",
            "/akr-quarantine%2Fa",
            "/akr-quarantine/a%2fb",
            "/akr-quarantine/a%5Cb",
            "/akr-quarantine/a\\b",
            "/akr-quarantine/a%00",
            "/akr-quarantine/a%0Ab",
            "/akr-quarantine/a%zz",
            "/akr-quarantine/a%",
            "/akr-quarantine/%ff%fe",
            "akr-quarantine/a",
        ] {
            assert_eq!(
                app(Method::DELETE, target),
                Err(Rule::OperationNotAllowed),
                "{target}"
            );
        }
        assert_eq!(
            app(Method::GET, "/akr-evidence/a?versionId=%zz"),
            Ok(Operation::GetObject)
        );
        assert_eq!(
            app(Method::GET, "/akr-evidence/a?version%zzId=1"),
            Err(Rule::OperationNotAllowed)
        );
    }

    #[test]
    fn a_request_signed_another_way_or_twice_is_refused() {
        let mut v2 = signed(&[]);
        v2.insert(
            "authorization",
            HeaderValue::from_static("AWS akr-writer:c2lnbmF0dXJl"),
        );
        assert_eq!(
            check(Method::GET, "/akr-evidence/a", &v2, false),
            Err(Rule::OperationNotAllowed)
        );

        let both = "/akr-evidence/a?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Signature=ab&X-Amz-SignedHeaders=host";
        assert_eq!(
            check(Method::GET, both, &signed(&[]), false),
            Err(Rule::OperationNotAllowed)
        );

        let mut twice = signed(&[]);
        twice.append(
            "authorization",
            HeaderValue::from_static("AWS4-HMAC-SHA256 x"),
        );
        assert_eq!(
            check(Method::GET, "/akr-evidence/a", &twice, false),
            Err(Rule::OperationNotAllowed)
        );
    }

    #[test]
    fn a_body_must_be_a_single_signed_part_of_known_digest() {
        let mut streaming = signed(&[]);
        streaming.insert(
            "x-amz-content-sha256",
            HeaderValue::from_static("STREAMING-AWS4-HMAC-SHA256-PAYLOAD"),
        );
        assert_eq!(
            check(Method::PUT, "/akr-evidence/a", &streaming, false),
            Err(Rule::OperationNotAllowed)
        );

        let mut unsigned = signed(&[]);
        unsigned.insert(
            "x-amz-content-sha256",
            HeaderValue::from_static("UNSIGNED-PAYLOAD"),
        );
        assert_eq!(
            check(Method::PUT, "/akr-evidence/a", &unsigned, false),
            Err(Rule::OperationNotAllowed)
        );

        let mut chunked = signed(&[]);
        chunked.insert("transfer-encoding", HeaderValue::from_static("chunked"));
        assert_eq!(
            check(Method::PUT, "/akr-evidence/a", &chunked, false),
            Err(Rule::OperationNotAllowed)
        );

        let mut no_digest = HeaderMap::new();
        no_digest.insert("host", HeaderValue::from_static("s3guard:9000"));
        no_digest.insert(
            "authorization",
            HeaderValue::from_static(
                "AWS4-HMAC-SHA256 Credential=c, SignedHeaders=host, Signature=ab",
            ),
        );
        assert_eq!(
            check(Method::PUT, "/akr-evidence/a", &no_digest, false),
            Err(Rule::OperationNotAllowed)
        );
        assert_eq!(
            check(Method::GET, "/akr-evidence/a", &no_digest, false),
            Ok(Operation::GetObject)
        );
    }
}
