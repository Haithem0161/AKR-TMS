//! Loads the images of a verified bundle into the bank's registry, with their signatures.
//!
//! The registry is spoken to over the OCI distribution API. A blob the registry holds already is not sent
//! again, so an import that was cut short is run again from the start.

use std::path::Path;
use std::time::Duration;

use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE, LOCATION};
use reqwest::redirect::Policy;
use reqwest::{Method, RequestBuilder, Response, StatusCode};
use url::{Host, Url};
use zeroize::Zeroizing;

use super::IMAGES_DIR;
use super::oci::{Descriptor, Layout, ManifestBody, split_ref};
use super::verify::Verified;
use crate::error::CtlError;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const READ_TIMEOUT: Duration = Duration::from_secs(120);
const CONTENT_DIGEST: &str = "docker-content-digest";
const OCTET_STREAM: &str = "application/octet-stream";

pub struct Registry {
    base: Url,
    http: reqwest::Client,
    basic: Option<(String, Zeroizing<String>)>,
}

fn is_loopback(url: &Url) -> bool {
    match url.host() {
        Some(Host::Ipv4(address)) => address.is_loopback(),
        Some(Host::Ipv6(address)) => address.is_loopback(),
        Some(Host::Domain(name)) => name == "localhost",
        None => false,
    }
}

impl Registry {
    /// `url` is the registry: `https://host[:port]`. Plain http is taken for a loopback address only.
    /// `cacert` is the only certificate authority trusted when it is given.
    pub fn new(
        url: &str,
        cacert: Option<&Path>,
        basic: Option<(String, Zeroizing<String>)>,
    ) -> Result<Self, CtlError> {
        let not_a_registry = |why: &str| CtlError::usage(format!("`{url}`: {why}"));
        let mut base = Url::parse(url).map_err(|e| not_a_registry(&e.to_string()))?;
        match base.scheme() {
            "https" => {}
            "http" if is_loopback(&base) => {}
            _ => return Err(not_a_registry("a registry is reached over https")),
        }
        if base.query().is_some() || base.fragment().is_some() || !base.username().is_empty() {
            return Err(not_a_registry(
                "the address holds a query, a fragment or a user name",
            ));
        }
        if !base.path().ends_with('/') {
            let path = format!("{}/", base.path());
            base.set_path(&path);
        }

        let mut http = reqwest::Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .read_timeout(READ_TIMEOUT)
            .redirect(Policy::none());
        if let Some(path) = cacert {
            let pem = std::fs::read(path).map_err(CtlError::file(path))?;
            let certificate = reqwest::Certificate::from_pem(&pem)
                .map_err(|e| CtlError::usage(format!("{}: {e}", path.display())))?;
            http = http.tls_certs_only([certificate]);
        }
        Ok(Self {
            base,
            http: http
                .build()
                .map_err(|e| CtlError::Registry(e.to_string()))?,
            basic,
        })
    }

    fn url(&self, repository: &str, rest: &str) -> Result<Url, CtlError> {
        self.base
            .join(&format!("v2/{repository}/{rest}"))
            .map_err(|e| CtlError::Registry(e.to_string()))
    }

    fn request(&self, method: Method, url: Url) -> RequestBuilder {
        let request = self.http.request(method, url);
        match &self.basic {
            Some((user, password)) => request.basic_auth(user, Some(password.as_str())),
            None => request,
        }
    }

    async fn send(
        &self,
        method: Method,
        url: Url,
        build: impl FnOnce(RequestBuilder) -> RequestBuilder,
    ) -> Result<Response, CtlError> {
        let what = format!("{method} {}", url.path());
        build(self.request(method, url))
            .send()
            .await
            .map_err(|e| CtlError::Registry(format!("{what}: {}", e.without_url())))
    }

    async fn has_blob(&self, repository: &str, digest: &str) -> Result<bool, CtlError> {
        let url = self.url(repository, &format!("blobs/{digest}"))?;
        let response = self.send(Method::HEAD, url, |request| request).await?;
        Ok(response.status() == StatusCode::OK)
    }

    /// `true` when the blob was sent, `false` when the registry held it already.
    async fn push_blob(
        &self,
        repository: &str,
        blob: &Descriptor,
        path: &Path,
    ) -> Result<bool, CtlError> {
        if self.has_blob(repository, &blob.digest).await? {
            return Ok(false);
        }
        let start = self.url(repository, "blobs/uploads/")?;
        let started = self.send(Method::POST, start, |request| request).await?;
        if started.status() != StatusCode::ACCEPTED {
            return Err(refused_by_registry(
                "POST",
                repository,
                "blobs/uploads/",
                &started,
            ));
        }
        let location = started
            .headers()
            .get(LOCATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|location| self.base.join(location).ok())
            .filter(|location| location.origin() == self.base.origin())
            .ok_or_else(|| {
                CtlError::Registry(format!(
                    "{repository}: the registry named no place on itself to send a blob to"
                ))
            })?;
        let mut target = location;
        target.query_pairs_mut().append_pair("digest", &blob.digest);

        let file = tokio::fs::File::open(path)
            .await
            .map_err(CtlError::file(path))?;
        let size = file.metadata().await.map_err(CtlError::file(path))?.len();
        if size != blob.size {
            return Err(CtlError::usage(format!(
                "{}: {size} bytes where its descriptor says {}",
                path.display(),
                blob.size
            )));
        }
        let sent = self
            .send(Method::PUT, target, |request| {
                request
                    .header(CONTENT_TYPE, OCTET_STREAM)
                    .header(CONTENT_LENGTH, size)
                    .body(reqwest::Body::from(file))
            })
            .await?;
        if sent.status() != StatusCode::CREATED {
            return Err(refused_by_registry("PUT", repository, &blob.digest, &sent));
        }
        Ok(true)
    }

    async fn put_manifest(&self, repository: &str, push: &ManifestPush) -> Result<(), CtlError> {
        let url = self.url(repository, &format!("manifests/{}", push.reference))?;
        let body = push.bytes.clone();
        let put = self
            .send(Method::PUT, url, |request| {
                request
                    .header(CONTENT_TYPE, push.media_type.as_str())
                    .body(body)
            })
            .await?;
        if put.status() != StatusCode::CREATED {
            let rest = format!("manifests/{}", push.reference);
            return Err(refused_by_registry("PUT", repository, &rest, &put));
        }
        let stored = put
            .headers()
            .get(CONTENT_DIGEST)
            .and_then(|value| value.to_str().ok());
        if stored.is_some_and(|stored| stored != push.digest) {
            return Err(CtlError::Registry(format!(
                "{repository}:{}: the registry stored another manifest than the one sent",
                push.reference
            )));
        }
        Ok(())
    }
}

fn refused_by_registry(
    method: &str,
    repository: &str,
    rest: &str,
    response: &Response,
) -> CtlError {
    CtlError::Registry(format!(
        "{method} /v2/{repository}/{rest}: the registry answered {}",
        response.status().as_u16()
    ))
}

struct ManifestPush {
    repository: String,
    /// A tag, or the digest for a manifest an index points at.
    reference: String,
    media_type: String,
    digest: String,
    bytes: Vec<u8>,
    blobs: Vec<Descriptor>,
}

/// Lists the manifests under `descriptor`, the ones an index points at before the index: a registry
/// refuses a manifest that names something it does not hold.
fn plan(
    layout: &Layout,
    repository: &str,
    reference: &str,
    descriptor: &Descriptor,
    pushes: &mut Vec<ManifestPush>,
) -> Result<(), CtlError> {
    let bytes = layout.blob(&descriptor.digest)?;
    let body: ManifestBody = serde_json::from_slice(&bytes)
        .map_err(|e| CtlError::usage(format!("{} is not a manifest: {e}", descriptor.digest)))?;
    for child in &body.manifests {
        plan(layout, repository, &child.digest, child, pushes)?;
    }
    pushes.push(ManifestPush {
        repository: repository.to_owned(),
        reference: reference.to_owned(),
        media_type: descriptor.media_type.clone(),
        digest: descriptor.digest.clone(),
        bytes,
        blobs: body.config.into_iter().chain(body.layers).collect(),
    });
    Ok(())
}

/// `verified` is the proof that `bundle verify` passed on this bundle: there is no other way to get one.
pub async fn import(
    bundle: &Path,
    verified: &Verified,
    registry: &Registry,
) -> Result<Vec<String>, CtlError> {
    let release = &verified.manifest.release;
    if verified.manifest.images.is_empty() {
        return Ok(vec![format!("bundle {release} holds no images")]);
    }
    let layout = Layout::open(&bundle.join(IMAGES_DIR))?;

    let mut pushes = Vec::new();
    let mut lines = vec![format!("bundle {release}: loading the images")];
    for entry in &layout.index()?.manifests {
        let Some(name) = entry.ref_name() else {
            continue;
        };
        let (repository, tag) = split_ref(name)
            .ok_or_else(|| CtlError::usage(format!("`{name}` is not `repository:tag`")))?;
        plan(&layout, repository, tag, entry, &mut pushes)?;
        lines.push(format!("  {name}  {}", entry.digest));
    }

    let (mut sent, mut held) = (0usize, 0usize);
    for push in &pushes {
        for blob in &push.blobs {
            let path = layout.blob_path(&blob.digest)?;
            if registry.push_blob(&push.repository, blob, &path).await? {
                sent += 1;
            } else {
                held += 1;
            }
        }
        registry.put_manifest(&push.repository, push).await?;
    }
    lines.push(format!(
        "  {} manifests written; {sent} blobs sent, {held} held by the registry already",
        pushes.len()
    ));
    Ok(lines)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn a_registry_is_reached_over_https_or_on_the_loopback() {
        assert!(Registry::new("https://harbor.bank.example", None, None).is_ok());
        assert!(Registry::new("http://127.0.0.1:5000", None, None).is_ok());
        assert!(Registry::new("http://localhost:5000", None, None).is_ok());
        for bad in [
            "http://harbor.bank.example",
            "harbor.bank.example",
            "ftp://127.0.0.1",
            "https://user@harbor.bank.example",
            "https://harbor.bank.example/?x=1",
        ] {
            assert!(Registry::new(bad, None, None).is_err(), "{bad}");
        }
    }

    #[test]
    fn requests_stay_under_the_address_given() {
        let registry = Registry::new("https://harbor.bank.example/registry", None, None).unwrap();
        assert_eq!(
            registry
                .url("akr/akr-api", "manifests/1.0.0")
                .unwrap()
                .as_str(),
            "https://harbor.bank.example/registry/v2/akr/akr-api/manifests/1.0.0"
        );
    }
}
