//! The OCI image layout under `images/`, and the cosign signatures it carries.
//!
//! An image is an entry of `index.json` named `repository:tag`. Its signature is the entry named
//! `repository:sha256-<hex>.sig`, where `<hex>` is the image's digest: the tag cosign gives a signature in a
//! registry. The signature manifest has one layer per signature; the layer's blob is the signed payload
//! (it names the image's digest) and the layer's annotation holds the signature.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::signing::{DevSigner, VendorKey};
use super::{ImageEntry, SIGNATURE_SUFFIX, read, sha256_hex, write};
use crate::error::CtlError;

pub const REF_NAME: &str = "org.opencontainers.image.ref.name";
pub const COSIGN_SIGNATURE: &str = "dev.cosignproject.cosign/signature";
pub const SIMPLE_SIGNING: &str = "application/vnd.dev.cosign.simplesigning.v1+json";
pub const IMAGE_MANIFEST: &str = "application/vnd.oci.image.manifest.v1+json";
pub const IMAGE_INDEX: &str = "application/vnd.oci.image.index.v1+json";
pub const IMAGE_CONFIG: &str = "application/vnd.oci.image.config.v1+json";
const COSIGN_TYPE: &str = "cosign container image signature";
const LAYOUT_FILE: &str = "oci-layout";
const LAYOUT_BODY: &str = "{\"imageLayoutVersion\":\"1.0.0\"}";
const INDEX_FILE: &str = "index.json";
const DIGEST_PREFIX: &str = "sha256:";
const MAX_TAG_LEN: usize = 128;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Descriptor {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub digest: String,
    pub size: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// `platform`, `artifactType` and whatever else a tool wrote, kept as found.
    #[serde(flatten)]
    pub other: BTreeMap<String, Value>,
}

impl Descriptor {
    pub fn annotation(&self, key: &str) -> Option<&str> {
        self.annotations.as_ref()?.get(key).map(String::as_str)
    }

    pub fn ref_name(&self) -> Option<&str> {
        self.annotation(REF_NAME)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Index {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(default)]
    pub manifests: Vec<Descriptor>,
    #[serde(flatten)]
    pub other: BTreeMap<String, Value>,
}

impl Index {
    pub fn named(&self, name: &str) -> Option<&Descriptor> {
        self.manifests.iter().find(|d| d.ref_name() == Some(name))
    }

    /// The images: every named entry that is not a signature. Sorted by name.
    pub fn images(&self) -> Vec<ImageEntry> {
        let mut images: Vec<ImageEntry> = self
            .manifests
            .iter()
            .filter_map(|d| {
                let name = d.ref_name()?;
                (!name.ends_with(SIGNATURE_SUFFIX)).then(|| ImageEntry {
                    name: name.to_owned(),
                    digest: d.digest.clone(),
                })
            })
            .collect();
        images.sort_by(|a, b| a.name.cmp(&b.name));
        images
    }
}

/// What a manifest blob points at: the configuration and layers of an image, or the manifests of an index.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ManifestBody {
    #[serde(default)]
    pub config: Option<Descriptor>,
    #[serde(default)]
    pub layers: Vec<Descriptor>,
    #[serde(default)]
    pub manifests: Vec<Descriptor>,
}

/// The hex of a `sha256:<64 lowercase hex>` digest.
pub fn hex_of(digest: &str) -> Option<&str> {
    let hex = digest.strip_prefix(DIGEST_PREFIX)?;
    (hex.len() == 64
        && hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
    .then_some(hex)
}

/// Splits `repository:tag`. The repository is lowercase path parts with no registry host; the bank's
/// registry is named at import.
pub fn split_ref(name: &str) -> Option<(&str, &str)> {
    let (repository, tag) = name.rsplit_once(':')?;
    let part_ok = |part: &str| {
        !part.is_empty()
            && part.bytes().all(|b| {
                b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-')
            })
            && part
                .bytes()
                .next()
                .is_some_and(|b| b.is_ascii_alphanumeric())
    };
    let tag_ok = !tag.is_empty()
        && tag.len() <= MAX_TAG_LEN
        && tag
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
        && !tag.starts_with(['.', '-']);
    (repository.split('/').all(part_ok) && tag_ok).then_some((repository, tag))
}

/// The name of the signature of the image `name` at `digest`.
pub fn signature_ref(name: &str, digest: &str) -> Option<String> {
    let (repository, _) = split_ref(name)?;
    Some(format!(
        "{repository}:sha256-{}{SIGNATURE_SUFFIX}",
        hex_of(digest)?
    ))
}

pub struct Layout {
    root: PathBuf,
}

impl Layout {
    pub fn open(root: &Path) -> Result<Self, CtlError> {
        for file in [LAYOUT_FILE, INDEX_FILE] {
            if !root.join(file).is_file() {
                return Err(CtlError::usage(format!(
                    "{} is not an OCI image layout: `{file}` is missing",
                    root.display()
                )));
            }
        }
        Ok(Self {
            root: root.to_path_buf(),
        })
    }

    /// An empty layout.
    pub fn create(root: &Path) -> Result<Self, CtlError> {
        write(&root.join(LAYOUT_FILE), LAYOUT_BODY.as_bytes())?;
        let layout = Self {
            root: root.to_path_buf(),
        };
        layout.write_index(&Index {
            schema_version: 2,
            media_type: Some(IMAGE_INDEX.to_owned()),
            manifests: Vec::new(),
            other: BTreeMap::new(),
        })?;
        Ok(layout)
    }

    pub fn index(&self) -> Result<Index, CtlError> {
        let path = self.root.join(INDEX_FILE);
        serde_json::from_slice(&read(&path)?)
            .map_err(|e| CtlError::usage(format!("{}: {e}", path.display())))
    }

    fn write_index(&self, index: &Index) -> Result<(), CtlError> {
        let bytes = serde_json::to_vec(index)
            .map_err(|e| CtlError::usage(format!("the image index cannot be written: {e}")))?;
        write(&self.root.join(INDEX_FILE), &bytes)
    }

    pub fn blob_path(&self, digest: &str) -> Result<PathBuf, CtlError> {
        let hex = hex_of(digest)
            .ok_or_else(|| CtlError::usage(format!("`{digest}` is not a sha256 digest")))?;
        Ok(self.root.join("blobs").join("sha256").join(hex))
    }

    /// Reads a blob and checks it against its digest.
    pub fn blob(&self, digest: &str) -> Result<Vec<u8>, CtlError> {
        let path = self.blob_path(digest)?;
        let bytes = read(&path)?;
        if hex_of(digest) != Some(sha256_hex(&bytes).as_str()) {
            return Err(CtlError::usage(format!(
                "{} does not hold the bytes its name promises",
                path.display()
            )));
        }
        Ok(bytes)
    }

    pub fn manifest_body(&self, digest: &str) -> Result<ManifestBody, CtlError> {
        serde_json::from_slice(&self.blob(digest)?)
            .map_err(|e| CtlError::usage(format!("{digest} is not a manifest: {e}")))
    }

    /// Stores `bytes` and returns their digest.
    pub fn put_blob(&self, bytes: &[u8]) -> Result<String, CtlError> {
        let digest = format!("{DIGEST_PREFIX}{}", sha256_hex(bytes));
        write(&self.blob_path(&digest)?, bytes)?;
        Ok(digest)
    }

    /// Stores a manifest and names it in the index, in place of an entry of the same name.
    pub fn tag(
        &self,
        name: &str,
        media_type: &str,
        manifest: &[u8],
    ) -> Result<Descriptor, CtlError> {
        if split_ref(name).is_none() {
            return Err(CtlError::usage(format!("`{name}` is not `repository:tag`")));
        }
        let descriptor = Descriptor {
            media_type: media_type.to_owned(),
            digest: self.put_blob(manifest)?,
            size: manifest.len() as u64,
            annotations: Some(BTreeMap::from([(REF_NAME.to_owned(), name.to_owned())])),
            other: BTreeMap::new(),
        };
        let mut index = self.index()?;
        index.manifests.retain(|d| d.ref_name() != Some(name));
        index.manifests.push(descriptor.clone());
        self.write_index(&index)?;
        Ok(descriptor)
    }

    /// Writes the cosign signature of `image` with a key from a file.
    pub fn sign_image(&self, image: &ImageEntry, signer: &DevSigner) -> Result<(), CtlError> {
        let not_a_ref = || {
            CtlError::usage(format!(
                "`{}` cannot be signed: not `repository:tag`",
                image.name
            ))
        };
        let (repository, _) = split_ref(&image.name).ok_or_else(not_a_ref)?;
        let name = signature_ref(&image.name, &image.digest).ok_or_else(not_a_ref)?;

        let payload = serde_json::to_vec(&json!({
            "critical": {
                "identity": { "docker-reference": repository },
                "image": { "docker-manifest-digest": image.digest },
                "type": COSIGN_TYPE,
            },
            "optional": null,
        }))
        .map_err(|e| CtlError::usage(e.to_string()))?;
        let config = b"{}";
        let manifest = serde_json::to_vec(&json!({
            "schemaVersion": 2,
            "mediaType": IMAGE_MANIFEST,
            "config": {
                "mediaType": IMAGE_CONFIG,
                "digest": self.put_blob(config)?,
                "size": config.len(),
            },
            "layers": [{
                "mediaType": SIMPLE_SIGNING,
                "digest": self.put_blob(&payload)?,
                "size": payload.len(),
                "annotations": { COSIGN_SIGNATURE: signer.sign(&payload) },
            }],
        }))
        .map_err(|e| CtlError::usage(e.to_string()))?;
        self.tag(&name, IMAGE_MANIFEST, &manifest)?;
        Ok(())
    }

    /// `Ok` when the layout holds `image` at its digest and a signature of that digest by `key`. `Err`
    /// says what is wrong.
    pub fn verify_image(
        &self,
        index: &Index,
        image: &ImageEntry,
        key: &VendorKey,
    ) -> Result<(), String> {
        match index.named(&image.name) {
            Some(found) if found.digest == image.digest => {}
            Some(found) => return Err(format!("the layout holds it at {}", found.digest)),
            None => return Err("the layout does not hold it".to_owned()),
        }
        let name = signature_ref(&image.name, &image.digest)
            .ok_or_else(|| "its name is not `repository:tag` with a sha256 digest".to_owned())?;
        let signature = index
            .named(&name)
            .ok_or_else(|| "it has no signature".to_owned())?;
        let body = self
            .manifest_body(&signature.digest)
            .map_err(|e| e.to_string())?;

        let signed = body.layers.iter().any(|layer| {
            let (Some(signature), Ok(payload)) =
                (layer.annotation(COSIGN_SIGNATURE), self.blob(&layer.digest))
            else {
                return false;
            };
            layer.media_type == SIMPLE_SIGNING
                && key.verifies(&payload, signature)
                && payload_names(&payload, &image.digest)
        });
        if signed {
            Ok(())
        } else {
            Err("no signature by the vendor key names its digest".to_owned())
        }
    }
}

/// The signed payload must be a cosign image signature of exactly this digest: a valid signature of another
/// image proves nothing about this one.
fn payload_names(payload: &[u8], digest: &str) -> bool {
    let Ok(value) = serde_json::from_slice::<Value>(payload) else {
        return false;
    };
    let critical = &value["critical"];
    critical["type"] == COSIGN_TYPE && critical["image"]["docker-manifest-digest"] == digest
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("akr-ctl-oci-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    fn image(layout: &Layout, name: &str, body: &[u8]) -> ImageEntry {
        let descriptor = layout.tag(name, IMAGE_MANIFEST, body).unwrap();
        ImageEntry {
            name: name.to_owned(),
            digest: descriptor.digest,
        }
    }

    #[test]
    fn a_reference_is_a_lowercase_repository_and_a_tag() {
        assert_eq!(
            split_ref("akr/akr-api:1.0.0"),
            Some(("akr/akr-api", "1.0.0"))
        );
        assert_eq!(split_ref("akr-api:latest"), Some(("akr-api", "latest")));
        for bad in [
            "akr-api",
            "akr-api:",
            ":1.0.0",
            "AKR/api:1",
            "akr//api:1",
            "akr/../api:1",
            "harbor.bank.example:8443/akr/api:1",
            "akr/api:-1",
        ] {
            assert_eq!(split_ref(bad), None, "{bad}");
        }
    }

    #[test]
    fn a_signature_is_named_after_the_digest_it_signs() {
        let digest = format!("sha256:{}", "ab".repeat(32));
        assert_eq!(
            signature_ref("akr/akr-api:1.0.0", &digest).unwrap(),
            format!("akr/akr-api:sha256-{}.sig", "ab".repeat(32))
        );
        assert_eq!(signature_ref("akr/akr-api:1.0.0", "sha256:AB"), None);
        assert_eq!(hex_of(&format!("sha512:{}", "ab".repeat(32))), None);
    }

    #[test]
    fn an_image_signed_by_the_vendor_verifies_and_one_signed_by_another_key_does_not() {
        let layout = Layout::create(&scratch("signed")).unwrap();
        let api = image(&layout, "akr/akr-api:1.0.0", b"{\"layers\":[]}");
        let signer = DevSigner::from_seed(&[3; 32]);
        layout.sign_image(&api, &signer).unwrap();
        let index = layout.index().unwrap();

        assert_eq!(index.images(), vec![api.clone()]);
        assert_eq!(
            layout.verify_image(&index, &api, &signer.vendor_key()),
            Ok(())
        );
        let other = DevSigner::from_seed(&[4; 32]).vendor_key();
        assert!(layout.verify_image(&index, &api, &other).is_err());
    }

    #[test]
    fn a_signature_of_another_digest_does_not_cover_the_image() {
        let layout = Layout::create(&scratch("moved")).unwrap();
        let signer = DevSigner::from_seed(&[3; 32]);
        let first = image(&layout, "akr/akr-api:1.0.0", b"{\"layers\":[]}");
        layout.sign_image(&first, &signer).unwrap();

        let replaced = image(&layout, "akr/akr-api:1.0.0", b"{\"layers\":[],\"x\":1}");
        let index = layout.index().unwrap();
        assert_eq!(
            layout.verify_image(&index, &replaced, &signer.vendor_key()),
            Err("it has no signature".to_owned())
        );
        assert!(
            layout
                .verify_image(&index, &first, &signer.vendor_key())
                .unwrap_err()
                .starts_with("the layout holds it at")
        );
    }

    #[test]
    fn a_blob_whose_bytes_changed_is_not_read() {
        let layout = Layout::create(&scratch("blob")).unwrap();
        let digest = layout.put_blob(b"payload").unwrap();
        assert_eq!(layout.blob(&digest).unwrap(), b"payload");

        std::fs::write(layout.blob_path(&digest).unwrap(), b"paYload").unwrap();
        assert!(layout.blob(&digest).is_err());
        assert!(layout.blob_path("sha256:../../etc").is_err());
    }
}
