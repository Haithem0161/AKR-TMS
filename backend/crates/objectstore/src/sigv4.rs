//! AWS Signature Version 4 for S3: the `Authorization` header and the presigned query.
//!
//! Pure functions over text. The client decides what a request is; this module signs it.

use chrono::{DateTime, Utc};
use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

use crate::ObjectStoreError;

const ALGORITHM: &str = "AWS4-HMAC-SHA256";
const SERVICE: &str = "s3";
const TERMINATOR: &str = "aws4_request";

/// What a presigned request signs in place of a body digest.
pub const UNSIGNED_PAYLOAD: &str = "UNSIGNED-PAYLOAD";

/// The SHA-256 of an empty body.
pub const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

/// Percent-encodes as SigV4 requires: every byte but `A-Z a-z 0-9 - . _ ~`, in upper-case hex. A path
/// keeps its slashes; a query name or value does not.
pub fn uri_encode(text: &str, keep_slash: bool) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(byte as char);
            }
            b'/' if keep_slash => out.push('/'),
            other => {
                out.push('%');
                out.push_str(&format!("{other:02X}"));
            }
        }
    }
    out
}

/// The query as it is signed and as it goes on the wire: encoded pairs, sorted by name then value.
pub fn canonical_query(pairs: &[(String, String)]) -> String {
    let mut encoded: Vec<(String, String)> = pairs
        .iter()
        .map(|(name, value)| (uri_encode(name, false), uri_encode(value, false)))
        .collect();
    encoded.sort();
    encoded
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join("&")
}

pub fn hex_sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

/// `20130524T000000Z`.
pub fn amz_date(at: DateTime<Utc>) -> String {
    at.format("%Y%m%dT%H%M%SZ").to_string()
}

fn short_date(at: DateTime<Utc>) -> String {
    at.format("%Y%m%d").to_string()
}

fn hmac(key: &[u8], data: &str) -> Result<Zeroizing<[u8; 32]>, ObjectStoreError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)
        .map_err(|_| ObjectStoreError::Config("the signing key has an unusable length".into()))?;
    mac.update(data.as_bytes());
    Ok(Zeroizing::new(mac.finalize().into_bytes().into()))
}

/// One request, as the signature sees it.
#[derive(Debug, Clone, Copy)]
pub struct Canonical<'a> {
    pub method: &'a str,
    /// The path as it goes on the wire, already percent-encoded.
    pub path: &'a str,
    /// Raw names and values.
    pub query: &'a [(String, String)],
    /// Names in lower case. Every header given is signed.
    pub headers: &'a [(String, String)],
    pub payload_hash: &'a str,
}

impl Canonical<'_> {
    fn sorted_headers(&self) -> Vec<(&str, String)> {
        let mut headers: Vec<(&str, String)> = self
            .headers
            .iter()
            .map(|(name, value)| {
                (
                    name.as_str(),
                    value.split_whitespace().collect::<Vec<_>>().join(" "),
                )
            })
            .collect();
        headers.sort();
        headers
    }

    pub fn signed_headers(&self) -> String {
        self.sorted_headers()
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
            .join(";")
    }

    fn text(&self) -> String {
        let headers: String = self
            .sorted_headers()
            .iter()
            .map(|(name, value)| format!("{name}:{value}\n"))
            .collect();
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            self.method,
            self.path,
            canonical_query(self.query),
            headers,
            self.signed_headers(),
            self.payload_hash
        )
    }
}

#[derive(Clone, Copy)]
pub struct Signer<'a> {
    pub access_key_id: &'a str,
    pub secret_access_key: &'a str,
    pub region: &'a str,
}

impl Signer<'_> {
    fn scope(&self, at: DateTime<Utc>) -> String {
        format!("{}/{}/{SERVICE}/{TERMINATOR}", short_date(at), self.region)
    }

    fn signature(
        &self,
        at: DateTime<Utc>,
        request: &Canonical<'_>,
    ) -> Result<String, ObjectStoreError> {
        let string_to_sign = format!(
            "{ALGORITHM}\n{}\n{}\n{}",
            amz_date(at),
            self.scope(at),
            hex_sha256(request.text().as_bytes())
        );
        let secret = Zeroizing::new(format!("AWS4{}", self.secret_access_key));
        let date_key = hmac(secret.as_bytes(), &short_date(at))?;
        let region_key = hmac(date_key.as_slice(), self.region)?;
        let service_key = hmac(region_key.as_slice(), SERVICE)?;
        let signing_key = hmac(service_key.as_slice(), TERMINATOR)?;
        Ok(hex::encode(
            hmac(signing_key.as_slice(), &string_to_sign)?.as_slice(),
        ))
    }

    /// The value of the `Authorization` header. `request.headers` must hold `host` and `x-amz-date`.
    pub fn authorization(
        &self,
        at: DateTime<Utc>,
        request: &Canonical<'_>,
    ) -> Result<String, ObjectStoreError> {
        Ok(format!(
            "{ALGORITHM} Credential={}/{}, SignedHeaders={}, Signature={}",
            self.access_key_id,
            self.scope(at),
            request.signed_headers(),
            self.signature(at, request)?
        ))
    }

    /// The whole query of a presigned request: `extra` plus the signature parameters. Only `host` is
    /// signed, and the body is not.
    pub fn presigned_query(
        &self,
        at: DateTime<Utc>,
        method: &str,
        path: &str,
        host: &str,
        expires_s: u64,
        extra: &[(String, String)],
    ) -> Result<String, ObjectStoreError> {
        let mut query = extra.to_vec();
        query.extend([
            ("X-Amz-Algorithm".to_owned(), ALGORITHM.to_owned()),
            (
                "X-Amz-Credential".to_owned(),
                format!("{}/{}", self.access_key_id, self.scope(at)),
            ),
            ("X-Amz-Date".to_owned(), amz_date(at)),
            ("X-Amz-Expires".to_owned(), expires_s.to_string()),
            ("X-Amz-SignedHeaders".to_owned(), "host".to_owned()),
        ]);
        let headers = [("host".to_owned(), host.to_owned())];
        let signature = self.signature(
            at,
            &Canonical {
                method,
                path,
                query: &query,
                headers: &headers,
                payload_hash: UNSIGNED_PAYLOAD,
            },
        )?;
        Ok(format!(
            "{}&X-Amz-Signature={signature}",
            canonical_query(&query)
        ))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    // The worked examples of the S3 API reference ("Signature Calculations for the Authorization
    // Header" and "Authenticating Requests: Using Query Parameters"). The key pair is the documented
    // example pair, not a credential.
    const SIGNER: Signer<'static> = Signer {
        access_key_id: "AKIAIOSFODNN7EXAMPLE",
        secret_access_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
        region: "us-east-1",
    };
    const HOST: &str = "examplebucket.s3.amazonaws.com";

    fn at() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2013, 5, 24, 0, 0, 0).unwrap()
    }

    fn pairs(items: &[(&str, &str)]) -> Vec<(String, String)> {
        items
            .iter()
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect()
    }

    fn signature_of(authorization: &str) -> &str {
        authorization.rsplit_once("Signature=").unwrap().1
    }

    #[test]
    fn get_object_with_a_range() {
        let headers = pairs(&[
            ("host", HOST),
            ("range", "bytes=0-9"),
            ("x-amz-content-sha256", EMPTY_SHA256),
            ("x-amz-date", "20130524T000000Z"),
        ]);
        let authorization = SIGNER
            .authorization(
                at(),
                &Canonical {
                    method: "GET",
                    path: "/test.txt",
                    query: &[],
                    headers: &headers,
                    payload_hash: EMPTY_SHA256,
                },
            )
            .unwrap();
        assert_eq!(
            authorization,
            "AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request, \
             SignedHeaders=host;range;x-amz-content-sha256;x-amz-date, \
             Signature=f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41"
        );
    }

    #[test]
    fn put_object_with_a_key_that_needs_encoding() {
        let body = b"Welcome to Amazon S3.";
        let payload_hash = hex_sha256(body);
        assert_eq!(
            payload_hash,
            "44ce7dd67c959e0d3524ffac1771dfbba87d2b6b4b4e99e42034a8b803f8b072"
        );
        let headers = pairs(&[
            ("date", "Fri, 24 May 2013 00:00:00 GMT"),
            ("host", HOST),
            ("x-amz-content-sha256", &payload_hash),
            ("x-amz-date", "20130524T000000Z"),
            ("x-amz-storage-class", "REDUCED_REDUNDANCY"),
        ]);
        let path = uri_encode("/test$file.text", true);
        assert_eq!(path, "/test%24file.text");
        let authorization = SIGNER
            .authorization(
                at(),
                &Canonical {
                    method: "PUT",
                    path: &path,
                    query: &[],
                    headers: &headers,
                    payload_hash: &payload_hash,
                },
            )
            .unwrap();
        assert_eq!(
            signature_of(&authorization),
            "98ad721746da40c64f1a55b78f14c238d841ea1380cd77a1b5971af0ece108bd"
        );
    }

    #[test]
    fn a_query_is_sorted_and_a_bare_name_gets_an_equals_sign() {
        let headers = pairs(&[
            ("host", HOST),
            ("x-amz-content-sha256", EMPTY_SHA256),
            ("x-amz-date", "20130524T000000Z"),
        ]);
        let sign = |query: &[(String, String)]| {
            let authorization = SIGNER
                .authorization(
                    at(),
                    &Canonical {
                        method: "GET",
                        path: "/",
                        query,
                        headers: &headers,
                        payload_hash: EMPTY_SHA256,
                    },
                )
                .unwrap();
            signature_of(&authorization).to_owned()
        };

        assert_eq!(canonical_query(&pairs(&[("lifecycle", "")])), "lifecycle=");
        assert_eq!(
            sign(&pairs(&[("lifecycle", "")])),
            "fea454ca298b7da1c68078a5d1bdbfbbe0d65c699e0f91ac7a200a0136783543"
        );

        let listing = pairs(&[("prefix", "J"), ("max-keys", "2")]);
        assert_eq!(canonical_query(&listing), "max-keys=2&prefix=J");
        assert_eq!(
            sign(&listing),
            "34b48302e7b5fa45bde8084f4b7868a86f0a534bc59db6670ed5711ef69dc6f7"
        );
    }

    #[test]
    fn a_presigned_get() {
        let query = SIGNER
            .presigned_query(at(), "GET", "/test.txt", HOST, 86400, &[])
            .unwrap();
        assert_eq!(
            query,
            "X-Amz-Algorithm=AWS4-HMAC-SHA256\
             &X-Amz-Credential=AKIAIOSFODNN7EXAMPLE%2F20130524%2Fus-east-1%2Fs3%2Faws4_request\
             &X-Amz-Date=20130524T000000Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host\
             &X-Amz-Signature=aeeed9bbccd4d02ee5c0109b86d86835f995330da4c265957d157751f604d404"
        );
    }

    #[test]
    fn encoding_keeps_the_unreserved_set_and_nothing_else() {
        assert_eq!(uri_encode("AZaz09-._~", false), "AZaz09-._~");
        assert_eq!(uri_encode("a b+c/d", false), "a%20b%2Bc%2Fd");
        assert_eq!(
            uri_encode("evidence/2026/a b.pdf", true),
            "evidence/2026/a%20b.pdf"
        );
        assert_eq!(uri_encode("ملف", false), "%D9%85%D9%84%D9%81");
    }

    #[test]
    fn a_header_value_is_trimmed_and_its_inner_spaces_collapse() {
        let headers = pairs(&[("host", "  example.test "), ("x-amz-meta-note", "a   b")]);
        let canonical = Canonical {
            method: "GET",
            path: "/",
            query: &[],
            headers: &headers,
            payload_hash: EMPTY_SHA256,
        };
        assert!(
            canonical
                .text()
                .contains("host:example.test\nx-amz-meta-note:a b\n\nhost;x-amz-meta-note\n")
        );
    }
}
