//! The release bundle: a directory the vendor signs and the bank verifies before anything changes.
//!
//! ```text
//! manifest.json        the release, every file with its SHA-256 and size, every image with its digest
//! manifest.json.sig    the vendor's signature over the bytes of manifest.json (base64, Ed25519)
//! notes.json           the release notes, in Arabic and English
//! images/              an OCI image layout: the images and their cosign signatures
//! sbom/                CycloneDX bills of materials, and the VEX statement sbom/vex.cdx.json
//! notices/             third-party notices and the object store's Corresponding Source
//! rollback/plan.md     the rollback plan
//! chart/               Helm charts, each beside its detached signature <chart>.sig
//! migrations/ compose/ docs/ evidence/ k3s/    listed in the manifest like every other file
//! ```

use std::io::Read;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::CtlError;

pub mod build;
pub mod import;
pub mod oci;
pub mod sbom;
pub mod signing;
pub mod verify;

pub const MANIFEST: &str = "manifest.json";
pub const MANIFEST_SIGNATURE: &str = "manifest.json.sig";
pub const NOTES: &str = "notes.json";
pub const IMAGES_DIR: &str = "images";
pub const SBOM_DIR: &str = "sbom";
pub const VEX: &str = "sbom/vex.cdx.json";
pub const NOTICES_DIR: &str = "notices";
pub const ROLLBACK_PLAN: &str = "rollback/plan.md";
pub const CHART_DIR: &str = "chart";
pub const SIGNATURE_SUFFIX: &str = ".sig";
/// The layout this build writes and reads.
pub const FORMAT: u32 = 1;

/// Why a bundle is refused. The release record keeps the code (phase 25), so the names are fixed.
pub mod refusal {
    pub const FINGERPRINT_MISMATCH: &str = "fingerprint_mismatch";
    pub const BUNDLE_SIGNATURE_INVALID: &str = "bundle_signature_invalid";
    pub const IMAGE_SIGNATURE_INVALID: &str = "image_signature_invalid";
    pub const MANIFEST_DIGEST_MISMATCH: &str = "manifest_digest_mismatch";
    pub const SBOM_MISSING: &str = "sbom_missing";
    pub const NOTES_MISSING: &str = "notes_missing";
    pub const NOTICES_MISSING: &str = "notices_missing";
    pub const ROLLBACK_PLAN_MISSING: &str = "rollback_plan_missing";
    pub const CHART_SIGNATURE_INVALID: &str = "chart_signature_invalid";
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format: u32,
    pub release: String,
    pub created_at: DateTime<Utc>,
    /// Every file of the bundle except the manifest and its signature, sorted by path.
    pub files: Vec<FileEntry>,
    pub images: Vec<ImageEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileEntry {
    /// Relative to the bundle, with `/` between its parts.
    pub path: String,
    pub sha256: String,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageEntry {
    /// `repository:tag`, with no registry host: the bank's registry is named at import.
    pub name: String,
    /// `sha256:<hex>` of the image manifest.
    pub digest: String,
}

/// A path the manifest may name: relative, with no empty, `.` or `..` part and no backslash.
pub fn is_safe_relative(path: &str) -> bool {
    !path.is_empty()
        && !path.contains('\\')
        && path
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

/// Every regular file under `root` as a `/`-separated path relative to it, sorted. A bundle holds files and
/// directories only: a symbolic link, or anything else, is an error.
pub fn walk(root: &Path) -> Result<Vec<String>, CtlError> {
    let mut files = Vec::new();
    let mut pending = vec![(root.to_path_buf(), String::new())];
    while let Some((dir, prefix)) = pending.pop() {
        for entry in std::fs::read_dir(&dir).map_err(CtlError::file(&dir))? {
            let entry = entry.map_err(CtlError::file(&dir))?;
            let path = entry.path();
            let name = entry.file_name().into_string().map_err(|_| {
                CtlError::usage(format!("{}: the name is not UTF-8", path.display()))
            })?;
            let relative = if prefix.is_empty() {
                name
            } else {
                format!("{prefix}/{name}")
            };
            let kind = std::fs::symlink_metadata(&path)
                .map_err(CtlError::file(&path))?
                .file_type();
            if kind.is_dir() {
                pending.push((path, relative));
            } else if kind.is_file() {
                files.push(relative);
            } else {
                return Err(CtlError::usage(format!(
                    "{}: not a file or a directory",
                    path.display()
                )));
            }
        }
    }
    files.sort_unstable();
    Ok(files)
}

/// The SHA-256 of a file in lowercase hex, and its size.
pub fn sha256_file(path: &Path) -> Result<(String, u64), CtlError> {
    let mut file = std::fs::File::open(path).map_err(CtlError::file(path))?;
    let mut hash = Sha256::new();
    let mut buffer = vec![0u8; 1 << 16];
    let mut size = 0u64;
    loop {
        let read = file.read(&mut buffer).map_err(CtlError::file(path))?;
        if read == 0 {
            break;
        }
        hash.update(&buffer[..read]);
        size += read as u64;
    }
    Ok((hex::encode(hash.finalize()), size))
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

pub(crate) fn read(path: &Path) -> Result<Vec<u8>, CtlError> {
    std::fs::read(path).map_err(CtlError::file(path))
}

pub(crate) fn write(path: &Path, bytes: &[u8]) -> Result<(), CtlError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(CtlError::file(parent))?;
    }
    std::fs::write(path, bytes).map_err(CtlError::file(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_manifest_path_stays_inside_the_bundle() {
        for good in [
            "manifest.json",
            "sbom/api.cdx.json",
            "images/blobs/sha256/ab",
        ] {
            assert!(is_safe_relative(good), "{good}");
        }
        for bad in [
            "",
            "/etc/passwd",
            "../x",
            "a/../b",
            "a//b",
            "a/./b",
            "a\\b",
            "a/",
        ] {
            assert!(!is_safe_relative(bad), "{bad}");
        }
    }
}
