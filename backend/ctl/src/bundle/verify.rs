//! Verifies a bundle. Nothing is installed from a bundle that fails here.
//!
//! The checks run in a fixed order and the first one that fails refuses the bundle with its code: the key the
//! bank pinned, the signature over the manifest, the signature of every image, the digest of every file, the
//! content every release must carry, the signature of every chart.

use std::collections::BTreeSet;
use std::path::Path;

use serde_json::Value;

use super::oci::Layout;
use super::sbom::{self, SbomDiff};
use super::signing::{VendorKey, same_fingerprint};
use super::{
    CHART_DIR, FORMAT, IMAGES_DIR, MANIFEST, MANIFEST_SIGNATURE, Manifest, NOTES, NOTICES_DIR,
    ROLLBACK_PLAN, SBOM_DIR, SIGNATURE_SUFFIX, VEX, is_safe_relative, read, refusal, sha256_file,
    sha256_hex, walk,
};
use crate::error::CtlError;

const BLOB_PREFIX: &str = "images/blobs/sha256/";

/// A bundle that passed every check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verified {
    pub manifest: Manifest,
    pub manifest_sha256: String,
    pub key_fingerprint: String,
    pub images: usize,
    pub charts: usize,
}

pub fn verify(bundle: &Path, key_pem: &str, fingerprint: &str) -> Result<Verified, CtlError> {
    if !bundle.is_dir() {
        return Err(CtlError::usage(format!(
            "{} is not a directory",
            bundle.display()
        )));
    }

    let key = VendorKey::from_pem(key_pem)?;
    let key_fingerprint = key.fingerprint();
    if !same_fingerprint(fingerprint, &key_fingerprint) {
        return Err(CtlError::refused(
            refusal::FINGERPRINT_MISMATCH,
            format!(
                "the public key has the fingerprint {key_fingerprint}, not the one given; compare it with the \
                 fingerprint the vendor sent on the separate channel"
            ),
        ));
    }

    let manifest_bytes = signed_manifest(bundle, &key)?;
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|e| CtlError::usage(format!("{MANIFEST}: {e}")))?;
    if manifest.format != FORMAT {
        return Err(CtlError::usage(format!(
            "{MANIFEST} has the format {}; this akr-ctl reads the format {FORMAT}",
            manifest.format
        )));
    }

    image_signatures(bundle, &manifest, &key)?;
    file_digests(bundle, &manifest)?;
    if let Some((code, detail)) = missing_content(bundle)? {
        return Err(CtlError::refused(code, detail));
    }
    let charts = chart_signatures(bundle, &key)?;

    Ok(Verified {
        manifest_sha256: sha256_hex(&manifest_bytes),
        key_fingerprint,
        images: manifest.images.len(),
        charts,
        manifest,
    })
}

fn signed_manifest(bundle: &Path, key: &VendorKey) -> Result<Vec<u8>, CtlError> {
    let refuse = |detail: String| CtlError::refused(refusal::BUNDLE_SIGNATURE_INVALID, detail);
    let manifest = std::fs::read(bundle.join(MANIFEST))
        .map_err(|e| refuse(format!("{MANIFEST} cannot be read: {e}")))?;
    let signature = std::fs::read_to_string(bundle.join(MANIFEST_SIGNATURE))
        .map_err(|e| refuse(format!("{MANIFEST_SIGNATURE} cannot be read: {e}")))?;
    if !key.verifies(&manifest, &signature) {
        return Err(refuse(format!(
            "{MANIFEST_SIGNATURE} is not the vendor's signature of {MANIFEST}"
        )));
    }
    Ok(manifest)
}

fn image_signatures(bundle: &Path, manifest: &Manifest, key: &VendorKey) -> Result<(), CtlError> {
    if manifest.images.is_empty() {
        return Ok(());
    }
    let refuse = |detail: String| CtlError::refused(refusal::IMAGE_SIGNATURE_INVALID, detail);
    let layout = Layout::open(&bundle.join(IMAGES_DIR)).map_err(|e| refuse(e.to_string()))?;
    let index = layout.index().map_err(|e| refuse(e.to_string()))?;
    for image in &manifest.images {
        layout
            .verify_image(&index, image, key)
            .map_err(|why| refuse(format!("{}: {why}", image.name)))?;
    }
    Ok(())
}

fn file_digests(bundle: &Path, manifest: &Manifest) -> Result<(), CtlError> {
    let refuse = |detail: String| CtlError::refused(refusal::MANIFEST_DIGEST_MISMATCH, detail);

    let mut listed = BTreeSet::new();
    for file in &manifest.files {
        let path = file.path.as_str();
        if !is_safe_relative(path) || path == MANIFEST || path == MANIFEST_SIGNATURE {
            return Err(refuse(format!(
                "the manifest names `{path}`, which is not a file of the bundle"
            )));
        }
        if !listed.insert(path) {
            return Err(refuse(format!("the manifest names `{path}` twice")));
        }
    }

    let present = walk(bundle).map_err(|e| refuse(e.to_string()))?;
    for path in &present {
        if path != MANIFEST && path != MANIFEST_SIGNATURE && !listed.contains(path.as_str()) {
            return Err(refuse(format!("`{path}` is not in the manifest")));
        }
    }

    for file in &manifest.files {
        let on_disk = bundle.join(&file.path);
        if !on_disk.is_file() {
            return Err(refuse(format!("`{}` is missing", file.path)));
        }
        let (sha256, size) = sha256_file(&on_disk)?;
        if size != file.size || sha256 != file.sha256 {
            return Err(refuse(format!(
                "`{}` is not the file the manifest lists",
                file.path
            )));
        }
        if let Some(name) = file.path.strip_prefix(BLOB_PREFIX)
            && name != sha256
        {
            return Err(refuse(format!(
                "`{}` does not hold the bytes its name promises",
                file.path
            )));
        }
    }

    let images_dir = bundle.join(IMAGES_DIR);
    let in_layout = if images_dir.is_dir() {
        Layout::open(&images_dir)
            .and_then(|layout| layout.index())
            .map_err(|e| refuse(e.to_string()))?
            .images()
    } else {
        Vec::new()
    };
    let mut in_manifest = manifest.images.clone();
    in_manifest.sort_by(|a, b| a.name.cmp(&b.name));
    if in_layout != in_manifest {
        return Err(refuse(
            "the images under `images/` are not the images the manifest lists".to_owned(),
        ));
    }
    Ok(())
}

/// What every release must carry, checked on the files and not on the manifest: a bundle that lists a file
/// with the right digest and the wrong content is still refused. `bundle build` runs the same check, so a
/// bundle that would be refused for this is never written.
pub fn missing_content(bundle: &Path) -> Result<Option<(&'static str, String)>, CtlError> {
    if sbom::sbom_files(bundle)?.is_empty() {
        return Ok(Some((
            refusal::SBOM_MISSING,
            format!("`{SBOM_DIR}/` holds no CycloneDX bill of materials"),
        )));
    }
    if !sbom::has_vex(bundle) {
        return Ok(Some((
            refusal::SBOM_MISSING,
            format!("`{VEX}` is missing or is not a CycloneDX VEX statement"),
        )));
    }

    let notes = std::fs::read(bundle.join(NOTES))
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok());
    if !notes.is_some_and(|notes| notes.as_object().is_some_and(|fields| !fields.is_empty())) {
        return Ok(Some((
            refusal::NOTES_MISSING,
            format!("`{NOTES}` is missing or holds no release notes"),
        )));
    }

    let notices = bundle.join(NOTICES_DIR);
    if !notices.is_dir() || walk(&notices)?.is_empty() {
        return Ok(Some((
            refusal::NOTICES_MISSING,
            format!("`{NOTICES_DIR}/` holds no third-party notices"),
        )));
    }

    let plan = bundle.join(ROLLBACK_PLAN);
    if !std::fs::metadata(&plan).is_ok_and(|meta| meta.is_file() && meta.len() > 0) {
        return Ok(Some((
            refusal::ROLLBACK_PLAN_MISSING,
            format!("`{ROLLBACK_PLAN}` is missing or empty"),
        )));
    }
    Ok(None)
}

/// The charts of a bundle: every file under `chart/` that is not a signature.
pub fn charts(bundle: &Path) -> Result<Vec<String>, CtlError> {
    let dir = bundle.join(CHART_DIR);
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    Ok(walk(&dir)?
        .into_iter()
        .filter(|file| !file.ends_with(SIGNATURE_SUFFIX))
        .map(|file| format!("{CHART_DIR}/{file}"))
        .collect())
}

fn chart_signatures(bundle: &Path, key: &VendorKey) -> Result<usize, CtlError> {
    let charts = charts(bundle)?;
    for chart in &charts {
        let signature =
            std::fs::read_to_string(bundle.join(format!("{chart}{SIGNATURE_SUFFIX}"))).ok();
        let signed = signature.is_some_and(|signature| {
            read(&bundle.join(chart)).is_ok_and(|bytes| key.verifies(&bytes, &signature))
        });
        if !signed {
            return Err(CtlError::refused(
                refusal::CHART_SIGNATURE_INVALID,
                format!("`{chart}` has no signature by the vendor key"),
            ));
        }
    }
    Ok(charts.len())
}

pub fn report(bundle: &Path, verified: &Verified) -> Vec<String> {
    vec![
        format!(
            "bundle {} verified: {}",
            verified.manifest.release,
            bundle.display()
        ),
        format!("  vendor key fingerprint  {}", verified.key_fingerprint),
        format!("  manifest sha256         {}", verified.manifest_sha256),
        format!(
            "  {} files, {} images, {} charts, created {}",
            verified.manifest.files.len(),
            verified.images,
            verified.charts,
            verified.manifest.created_at.to_rfc3339()
        ),
    ]
}

/// What the new release adds, removes and changes against the bundle of the installed one.
pub fn sbom_diff(bundle: &Path, installed: &Path) -> Result<SbomDiff, CtlError> {
    if sbom::sbom_files(installed)?.is_empty() {
        return Err(CtlError::usage(format!(
            "{} holds no bill of materials to compare with",
            installed.display()
        )));
    }
    Ok(sbom::diff(
        &sbom::components(installed)?,
        &sbom::components(bundle)?,
    ))
}
