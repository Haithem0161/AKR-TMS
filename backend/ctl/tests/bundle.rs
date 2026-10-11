//! A bundle from build to the registry: what verifies, what each kind of damage is refused as, and what an
//! import sends.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use akr_ctl::CtlError;
use akr_ctl::bundle::build::{BuildOptions, build, seal};
use akr_ctl::bundle::import::{Registry, import};
use akr_ctl::bundle::oci::{IMAGE_CONFIG, IMAGE_MANIFEST, Layout, signature_ref};
use akr_ctl::bundle::signing::DevSigner;
use akr_ctl::bundle::verify::{Verified, sbom_diff, verify};
use akr_ctl::bundle::{ImageEntry, MANIFEST, MANIFEST_SIGNATURE, sha256_hex};
use axum::Router;
use axum::body::Bytes;
use axum::extract::{Path as UrlPath, Query, State};
use axum::http::{HeaderMap, Method, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use chrono::{DateTime, Utc};
use serde_json::json;

const RELEASE: &str = "1.0.0";
const IMAGE: &str = "akr/akr-api:1.0.0";
const CHART: &str = "chart/akr-1.0.0.tgz";
const MIGRATION: &str = "migrations/0001_init.sql";

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("akr-ctl-bundle-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn put(root: &Path, path: &str, bytes: &[u8]) {
    let file = root.join(path);
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(file, bytes).unwrap();
}

fn at() -> DateTime<Utc> {
    DateTime::parse_from_rfc3339("2026-10-11T12:00:00Z")
        .unwrap()
        .with_timezone(&Utc)
}

fn vendor() -> DevSigner {
    DevSigner::from_seed(&[7; 32])
}

fn sbom(axum_version: &str) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "bomFormat": "CycloneDX",
        "specVersion": "1.6",
        "components": [
            { "name": "axum", "version": axum_version },
            { "name": "tokio", "version": "1.53.2" },
        ],
    }))
    .unwrap()
}

/// A staged tree and an image layout with one image of one layer.
fn stage(root: &Path, axum_version: &str) -> (PathBuf, PathBuf) {
    let source = root.join("staged");
    put(&source, "sbom/akr-api.cdx.json", &sbom(axum_version));
    put(
        &source,
        "sbom/vex.cdx.json",
        br#"{"bomFormat":"CycloneDX","specVersion":"1.6","vulnerabilities":[]}"#,
    );
    put(
        &source,
        "notes.json",
        r#"{"en":"First release.","ar":"الإصدار الأول."}"#.as_bytes(),
    );
    put(&source, "notices/THIRD-PARTY.txt", b"axum: MIT\n");
    put(
        &source,
        "rollback/plan.md",
        b"# Rollback\n\nRestore 0.9.0.\n",
    );
    put(&source, CHART, b"not really a chart archive");
    put(&source, MIGRATION, b"SELECT 1;\n");

    let layout = Layout::create(&root.join("layout")).unwrap();
    let config = b"{\"architecture\":\"amd64\",\"os\":\"linux\"}";
    let layer = b"the bytes of one layer";
    let manifest = serde_json::to_vec(&json!({
        "schemaVersion": 2,
        "mediaType": IMAGE_MANIFEST,
        "config": {
            "mediaType": IMAGE_CONFIG,
            "digest": layout.put_blob(config).unwrap(),
            "size": config.len(),
        },
        "layers": [{
            "mediaType": "application/vnd.oci.image.layer.v1.tar+gzip",
            "digest": layout.put_blob(layer).unwrap(),
            "size": layer.len(),
        }],
    }))
    .unwrap();
    layout.tag(IMAGE, IMAGE_MANIFEST, &manifest).unwrap();
    (source, root.join("layout"))
}

struct Built {
    bundle: PathBuf,
    signer: DevSigner,
}

impl Built {
    fn new(name: &str) -> Self {
        Self::with_sbom(name, "0.8.9")
    }

    fn with_sbom(name: &str, axum_version: &str) -> Self {
        let root = scratch(name);
        let (source, layout) = stage(&root, axum_version);
        let signer = vendor();
        let options = BuildOptions {
            source,
            out: root.join("bundle"),
            release: RELEASE.to_owned(),
            images_layout: Some(layout),
        };
        build(&options, Some(&signer), at()).unwrap();
        Self {
            bundle: root.join("bundle"),
            signer,
        }
    }

    fn verify(&self) -> Result<Verified, CtlError> {
        let key = self.signer.vendor_key();
        verify(&self.bundle, &key.to_pem(), &key.fingerprint())
    }

    fn refusal(&self) -> &'static str {
        self.verify()
            .expect_err("the bundle verified")
            .code()
            .expect("the error is not a refusal")
    }

    /// Writes the manifest again and signs it, as a vendor who changed the bundle on purpose would.
    fn seal(&self) {
        seal(&self.bundle, RELEASE, Some(&self.signer), at()).unwrap();
    }

    fn image(&self) -> ImageEntry {
        self.verify_unchecked_images().remove(0)
    }

    fn verify_unchecked_images(&self) -> Vec<ImageEntry> {
        Layout::open(&self.bundle.join("images"))
            .unwrap()
            .index()
            .unwrap()
            .images()
    }

    fn flip_first_byte(&self, path: &str) {
        let file = self.bundle.join(path);
        let mut bytes = std::fs::read(&file).unwrap();
        bytes[0] ^= 0x01;
        std::fs::write(file, bytes).unwrap();
    }
}

#[test]
fn a_built_bundle_verifies() {
    let built = Built::new("verifies");
    let verified = built.verify().unwrap();

    assert_eq!(verified.manifest.release, RELEASE);
    assert_eq!(verified.images, 1);
    assert_eq!(verified.charts, 1);
    assert_eq!(
        verified.key_fingerprint,
        built.signer.vendor_key().fingerprint()
    );
    assert_eq!(
        verified.manifest_sha256,
        sha256_hex(&std::fs::read(built.bundle.join(MANIFEST)).unwrap())
    );
    let listed: Vec<&str> = verified
        .manifest
        .files
        .iter()
        .map(|file| file.path.as_str())
        .collect();
    assert!(listed.contains(&MIGRATION));
    assert!(listed.contains(&"images/index.json"));
    assert!(listed.contains(&"chart/akr-1.0.0.tgz.sig"));
    assert!(!listed.contains(&MANIFEST));
    assert!(listed.is_sorted());
}

#[test]
fn the_fingerprint_is_checked_before_anything_is_read() {
    let built = Built::new("fingerprint");
    let key = built.signer.vendor_key();

    let wrong = "0".repeat(64);
    let refused = verify(&built.bundle, &key.to_pem(), &wrong).unwrap_err();
    assert_eq!(refused.code(), Some("fingerprint_mismatch"));

    let spaced = key
        .fingerprint()
        .to_uppercase()
        .as_bytes()
        .chunks(2)
        .map(|pair| String::from_utf8_lossy(pair).into_owned())
        .collect::<Vec<_>>()
        .join(":");
    assert!(verify(&built.bundle, &key.to_pem(), &spaced).is_ok());

    let empty = scratch("fingerprint-empty");
    let refused = verify(&empty, &key.to_pem(), &wrong).unwrap_err();
    assert_eq!(refused.code(), Some("fingerprint_mismatch"));
}

#[test]
fn a_manifest_the_vendor_did_not_sign_is_refused() {
    let built = Built::new("signature");
    built.flip_first_byte(MANIFEST);
    assert_eq!(built.refusal(), "bundle_signature_invalid");

    let built = Built::new("signature-gone");
    std::fs::remove_file(built.bundle.join(MANIFEST_SIGNATURE)).unwrap();
    assert_eq!(built.refusal(), "bundle_signature_invalid");

    let built = Built::new("signature-other-key");
    let other = DevSigner::from_seed(&[8; 32]).vendor_key();
    let refused = verify(&built.bundle, &other.to_pem(), &other.fingerprint()).unwrap_err();
    assert_eq!(refused.code(), Some("bundle_signature_invalid"));
}

#[test]
fn one_changed_byte_in_any_file_is_refused() {
    let built = Built::new("byte");
    built.flip_first_byte(MIGRATION);
    assert_eq!(built.refusal(), "manifest_digest_mismatch");

    let built = Built::new("byte-layer");
    let layer = format!(
        "images/blobs/sha256/{}",
        sha256_hex(b"the bytes of one layer")
    );
    built.flip_first_byte(&layer);
    assert_eq!(built.refusal(), "manifest_digest_mismatch");

    let built = Built::new("extra");
    put(&built.bundle, "compose/extra.yaml", b"services: {}\n");
    assert_eq!(built.refusal(), "manifest_digest_mismatch");

    let built = Built::new("gone");
    std::fs::remove_file(built.bundle.join(MIGRATION)).unwrap();
    assert_eq!(built.refusal(), "manifest_digest_mismatch");
}

#[test]
fn a_blob_under_another_name_is_refused_even_when_the_manifest_lists_it() {
    let built = Built::new("blob-name");
    let stray = format!("images/blobs/sha256/{}", "a".repeat(64));
    put(&built.bundle, &stray, b"not the bytes of that name");
    built.seal();
    assert_eq!(built.refusal(), "manifest_digest_mismatch");
}

#[test]
fn an_image_the_vendor_did_not_sign_is_refused() {
    let built = Built::new("image");
    let layout = Layout::open(&built.bundle.join("images")).unwrap();
    layout
        .sign_image(&built.image(), &DevSigner::from_seed(&[8; 32]))
        .unwrap();
    built.seal();
    assert_eq!(built.refusal(), "image_signature_invalid");

    let built = Built::new("image-unsigned");
    let image = built.image();
    let signature = signature_ref(&image.name, &image.digest).unwrap();
    let index_path = built.bundle.join("images/index.json");
    let mut index: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&index_path).unwrap()).unwrap();
    index["manifests"].as_array_mut().unwrap().retain(|entry| {
        entry["annotations"]["org.opencontainers.image.ref.name"] != signature.as_str()
    });
    std::fs::write(&index_path, serde_json::to_vec(&index).unwrap()).unwrap();
    built.seal();
    assert_eq!(built.refusal(), "image_signature_invalid");
}

#[test]
fn a_changed_chart_is_refused_even_when_the_manifest_lists_it() {
    let built = Built::new("chart");
    built.flip_first_byte(CHART);
    assert_eq!(built.refusal(), "manifest_digest_mismatch");
    built.seal();
    assert_eq!(built.refusal(), "chart_signature_invalid");

    let built = Built::new("chart-unsigned");
    std::fs::remove_file(built.bundle.join(format!("{CHART}.sig"))).unwrap();
    built.seal();
    assert_eq!(built.refusal(), "chart_signature_invalid");
}

#[test]
fn a_release_without_what_every_release_carries_is_refused() {
    let cases: [(&str, &str, Option<&[u8]>, &str); 6] = [
        ("no-sbom", "sbom/akr-api.cdx.json", None, "sbom_missing"),
        ("no-vex", "sbom/vex.cdx.json", None, "sbom_missing"),
        ("no-notes", "notes.json", Some(b"{}"), "notes_missing"),
        (
            "no-notices",
            "notices/THIRD-PARTY.txt",
            None,
            "notices_missing",
        ),
        ("no-plan", "rollback/plan.md", None, "rollback_plan_missing"),
        (
            "empty-plan",
            "rollback/plan.md",
            Some(b""),
            "rollback_plan_missing",
        ),
    ];
    for (name, path, replacement, code) in cases {
        let built = Built::new(name);
        match replacement {
            Some(bytes) => std::fs::write(built.bundle.join(path), bytes).unwrap(),
            None => std::fs::remove_file(built.bundle.join(path)).unwrap(),
        }
        built.seal();
        assert_eq!(built.refusal(), code, "{name}");
    }
}

#[test]
fn a_tree_without_what_every_release_carries_is_not_built() {
    let root = scratch("build-refused");
    let (source, layout) = stage(&root, "0.8.9");
    std::fs::remove_dir_all(source.join("notices")).unwrap();
    let options = BuildOptions {
        source: source.clone(),
        out: root.join("bundle"),
        release: RELEASE.to_owned(),
        images_layout: Some(layout),
    };
    let refused = build(&options, Some(&vendor()), at()).unwrap_err();
    assert_eq!(refused.code(), Some("notices_missing"));

    let inside = BuildOptions {
        out: source.join("out"),
        ..options.clone()
    };
    assert!(matches!(
        build(&inside, None, at()),
        Err(CtlError::Usage(_))
    ));
}

#[test]
fn an_unsigned_build_is_written_and_does_not_verify() {
    let root = scratch("unsigned");
    let (source, layout) = stage(&root, "0.8.9");
    let options = BuildOptions {
        source,
        out: root.join("bundle"),
        release: RELEASE.to_owned(),
        images_layout: Some(layout),
    };
    let lines = build(&options, None, at()).unwrap();
    assert!(lines.iter().any(|line| line.contains("NOT signed")));
    assert!(!root.join("bundle").join(MANIFEST_SIGNATURE).exists());

    let key = vendor().vendor_key();
    let refused = verify(&root.join("bundle"), &key.to_pem(), &key.fingerprint()).unwrap_err();
    assert_eq!(refused.code(), Some("bundle_signature_invalid"));
}

#[test]
fn the_bill_of_materials_is_compared_with_the_installed_release() {
    let installed = Built::with_sbom("sbom-installed", "0.8.8");
    let new = Built::with_sbom("sbom-new", "0.8.9");
    let diff = sbom_diff(&new.bundle, &installed.bundle).unwrap();
    assert_eq!(diff.changed, ["axum 0.8.8 -> 0.8.9"]);
    assert!(diff.added.is_empty() && diff.removed.is_empty());

    assert!(sbom_diff(&new.bundle, &scratch("sbom-nothing")).is_err());
}

#[derive(Default)]
struct Held {
    blobs: HashMap<String, Vec<u8>>,
    /// `repository:reference` to the media type and the bytes.
    manifests: HashMap<String, (String, Vec<u8>)>,
    uploads: usize,
}

#[derive(Clone, Default)]
struct MockRegistry {
    held: Arc<Mutex<Held>>,
    /// Answers 401 to everything.
    locked: bool,
}

async fn registry(
    State(mock): State<MockRegistry>,
    method: Method,
    UrlPath(rest): UrlPath<String>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if mock.locked {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let mut held = mock.held.lock().unwrap();
    let rest = rest.trim_end_matches('/');

    if let Some((repository, digest)) = rest.split_once("/blobs/sha256:") {
        let _ = repository;
        return if method == Method::HEAD && held.blobs.contains_key(&format!("sha256:{digest}")) {
            StatusCode::OK.into_response()
        } else {
            StatusCode::NOT_FOUND.into_response()
        };
    }
    if let Some(repository) = rest.strip_suffix("/blobs/uploads") {
        if method != Method::POST {
            return StatusCode::METHOD_NOT_ALLOWED.into_response();
        }
        let location = format!("/v2/{repository}/blobs/uploads/session-1?state=opaque");
        return (StatusCode::ACCEPTED, [(header::LOCATION, location)]).into_response();
    }
    if rest.contains("/blobs/uploads/") {
        let Some(digest) = query.get("digest") else {
            return StatusCode::BAD_REQUEST.into_response();
        };
        if method != Method::PUT
            || query.get("state").map(String::as_str) != Some("opaque")
            || *digest != format!("sha256:{}", sha256_hex(&body))
        {
            return StatusCode::BAD_REQUEST.into_response();
        }
        held.blobs.insert(digest.clone(), body.to_vec());
        held.uploads += 1;
        return StatusCode::CREATED.into_response();
    }
    if let Some((repository, reference)) = rest.split_once("/manifests/") {
        if method != Method::PUT {
            return StatusCode::METHOD_NOT_ALLOWED.into_response();
        }
        let media_type = headers
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
            .to_owned();
        let digest = format!("sha256:{}", sha256_hex(&body));
        held.manifests.insert(
            format!("{repository}:{reference}"),
            (media_type, body.to_vec()),
        );
        return (StatusCode::CREATED, [("docker-content-digest", digest)]).into_response();
    }
    StatusCode::NOT_FOUND.into_response()
}

async fn serve(mock: MockRegistry) -> String {
    let app = Router::new()
        .route("/v2/{*rest}", any(registry))
        .with_state(mock);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    format!("http://{address}")
}

#[tokio::test]
async fn an_import_sends_every_image_with_its_signature_and_a_second_one_sends_nothing() {
    let built = Built::new("import");
    let verified = built.verify().unwrap();
    let mock = MockRegistry::default();
    let registry = Registry::new(&serve(mock.clone()).await, None, None).unwrap();

    import(&built.bundle, &verified, &registry).await.unwrap();

    let image = built.image();
    let signature = signature_ref(&image.name, &image.digest).unwrap();
    {
        let held = mock.held.lock().unwrap();
        let (media_type, bytes) = &held.manifests[IMAGE];
        assert_eq!(media_type, IMAGE_MANIFEST);
        assert_eq!(format!("sha256:{}", sha256_hex(bytes)), image.digest);
        assert!(held.manifests.contains_key(&signature));
        assert!(
            held.blobs
                .contains_key(&format!("sha256:{}", sha256_hex(b"the bytes of one layer")))
        );
        // The image's configuration and layer, the signature's configuration and payload.
        assert_eq!(held.uploads, 4);
    }

    let lines = import(&built.bundle, &verified, &registry).await.unwrap();
    assert_eq!(mock.held.lock().unwrap().uploads, 4);
    assert!(lines.last().unwrap().contains("0 blobs sent, 4 held"));
}

#[tokio::test]
async fn a_registry_that_refuses_stops_the_import() {
    let built = Built::new("import-refused");
    let verified = built.verify().unwrap();
    let mock = MockRegistry {
        locked: true,
        ..MockRegistry::default()
    };
    let registry = Registry::new(&serve(mock.clone()).await, None, None).unwrap();

    let refused = import(&built.bundle, &verified, &registry)
        .await
        .unwrap_err();
    assert!(matches!(&refused, CtlError::Registry(message) if message.contains("401")));
    assert!(mock.held.lock().unwrap().manifests.is_empty());
}
