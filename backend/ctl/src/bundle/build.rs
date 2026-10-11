//! Builds a bundle from a staged tree: copies it, adds the image layout, signs what a key in a file may sign,
//! and writes the manifest.
//!
//! A release is signed by cosign with the key in the vendor's KMS. A key in a file signs development and test
//! bundles only; without one the bundle is written unsigned and the pipeline signs it.

use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};

use super::oci::Layout;
use super::signing::DevSigner;
use super::verify::{charts, missing_content};
use super::{
    FORMAT, FileEntry, IMAGES_DIR, MANIFEST, MANIFEST_SIGNATURE, Manifest, SIGNATURE_SUFFIX, read,
    sha256_file, sha256_hex, walk, write,
};
use crate::error::CtlError;

const MAX_RELEASE_LEN: usize = 64;

#[derive(Debug, Clone)]
pub struct BuildOptions {
    /// The staged tree: everything of the bundle except the manifest, its signature and, when
    /// `images_layout` is given, the images.
    pub source: PathBuf,
    /// Where the bundle is written. Absent or empty.
    pub out: PathBuf,
    pub release: String,
    /// An OCI image layout to copy to `images/`.
    pub images_layout: Option<PathBuf>,
}

fn is_release(release: &str) -> bool {
    !release.is_empty()
        && release.len() <= MAX_RELEASE_LEN
        && release
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_' | b'+'))
}

fn copy_tree(from: &Path, to: &Path) -> Result<usize, CtlError> {
    let files = walk(from)?;
    for file in &files {
        let target = to.join(file);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(CtlError::file(parent))?;
        }
        std::fs::copy(from.join(file), &target).map_err(CtlError::file(&target))?;
    }
    Ok(files.len())
}

fn check_places(options: &BuildOptions) -> Result<(), CtlError> {
    let source = std::fs::canonicalize(&options.source).map_err(CtlError::file(&options.source))?;
    if !source.is_dir() {
        return Err(CtlError::usage(format!(
            "{} is not a directory",
            source.display()
        )));
    }
    for reserved in [MANIFEST, MANIFEST_SIGNATURE] {
        if source.join(reserved).exists() {
            return Err(CtlError::usage(format!(
                "{} already holds `{reserved}`: it is a bundle, not a tree to build one from",
                source.display()
            )));
        }
    }
    if options.images_layout.is_some() && source.join(IMAGES_DIR).exists() {
        return Err(CtlError::usage(format!(
            "{} holds `{IMAGES_DIR}/` and an image layout was given too",
            source.display()
        )));
    }

    let out = std::path::absolute(&options.out).map_err(CtlError::file(&options.out))?;
    if out.starts_with(&source) {
        return Err(CtlError::usage(
            "the bundle cannot be written inside the tree it is built from",
        ));
    }
    match std::fs::read_dir(&out) {
        Ok(mut entries) => {
            if entries.next().is_some() {
                return Err(CtlError::usage(format!("{} is not empty", out.display())));
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(CtlError::file(&out)(e)),
    }
    Ok(())
}

fn sign(out: &Path, signer: &DevSigner) -> Result<Vec<String>, CtlError> {
    let key = signer.vendor_key();
    let mut lines = Vec::new();

    let images = out.join(IMAGES_DIR);
    if images.is_dir() {
        let layout = Layout::open(&images)?;
        let mut signed = 0;
        for image in layout.index()?.images() {
            if layout.verify_image(&layout.index()?, &image, &key).is_err() {
                layout.sign_image(&image, signer)?;
                signed += 1;
            }
        }
        lines.push(format!("  signed {signed} images"));
    }

    let mut signed = 0;
    for chart in charts(out)? {
        let bytes = read(&out.join(&chart))?;
        let signature_path = out.join(format!("{chart}{SIGNATURE_SUFFIX}"));
        let has_signature = std::fs::read_to_string(&signature_path)
            .is_ok_and(|signature| key.verifies(&bytes, &signature));
        if !has_signature {
            write(&signature_path, signer.sign(&bytes).as_bytes())?;
            signed += 1;
        }
    }
    if signed > 0 {
        lines.push(format!("  signed {signed} charts"));
    }
    Ok(lines)
}

pub fn build(
    options: &BuildOptions,
    signer: Option<&DevSigner>,
    created_at: DateTime<Utc>,
) -> Result<Vec<String>, CtlError> {
    if !is_release(&options.release) {
        return Err(CtlError::usage(format!(
            "`{}` is not a release name: letters, digits, `.`, `-`, `_` and `+`, at most {MAX_RELEASE_LEN}",
            options.release
        )));
    }
    check_places(options)?;
    let out = options.out.as_path();
    std::fs::create_dir_all(out).map_err(CtlError::file(out))?;

    copy_tree(&options.source, out)?;
    if let Some(layout) = &options.images_layout {
        Layout::open(layout)?;
        copy_tree(layout, &out.join(IMAGES_DIR))?;
    }

    let mut lines = vec![format!(
        "bundle {} written to {}",
        options.release,
        out.display()
    )];
    if let Some(signer) = signer {
        lines.extend(sign(out, signer)?);
    }
    if let Some((code, detail)) = missing_content(out)? {
        return Err(CtlError::refused(code, detail));
    }

    lines.extend(seal(out, &options.release, signer, created_at)?);
    Ok(lines)
}

/// Writes the manifest of the files under `out` as they are now and, with a key, its signature. The last step
/// of a build; the pipeline runs it again after it has signed the images and the charts with the release key.
pub fn seal(
    out: &Path,
    release: &str,
    signer: Option<&DevSigner>,
    created_at: DateTime<Utc>,
) -> Result<Vec<String>, CtlError> {
    let mut files = Vec::new();
    for path in walk(out)? {
        if path == MANIFEST || path == MANIFEST_SIGNATURE {
            continue;
        }
        let (sha256, size) = sha256_file(&out.join(&path))?;
        files.push(FileEntry { path, sha256, size });
    }
    let images_dir = out.join(IMAGES_DIR);
    let images = if images_dir.is_dir() {
        Layout::open(&images_dir)?.index()?.images()
    } else {
        Vec::new()
    };
    let manifest = Manifest {
        format: FORMAT,
        release: release.to_owned(),
        created_at,
        files,
        images,
    };
    let mut bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|e| CtlError::usage(format!("the manifest cannot be written: {e}")))?;
    bytes.push(b'\n');
    write(&out.join(MANIFEST), &bytes)?;

    let mut lines = vec![
        format!(
            "  {} files, {} images",
            manifest.files.len(),
            manifest.images.len()
        ),
        format!("  manifest sha256         {}", sha256_hex(&bytes)),
    ];
    match signer {
        Some(signer) => {
            write(
                &out.join(MANIFEST_SIGNATURE),
                signer.sign(&bytes).as_bytes(),
            )?;
            lines.push(format!(
                "  signed with the key in the file; vendor key fingerprint {}",
                signer.vendor_key().fingerprint()
            ));
        }
        None => {
            let stale = out.join(MANIFEST_SIGNATURE);
            if stale.exists() {
                std::fs::remove_file(&stale).map_err(CtlError::file(&stale))?;
            }
            lines.push(format!(
                "  NOT signed: sign {MANIFEST}, every image and every chart with the release key before it ships"
            ));
        }
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_release_name_is_short_and_plain() {
        for good in ["1.0.0", "1.4.2-rc.1", "2026.10_1+build7"] {
            assert!(is_release(good), "{good}");
        }
        for bad in ["", "1.0 0", "../1", "1/0", &"1".repeat(65)] {
            assert!(!is_release(bad), "{bad}");
        }
    }
}
