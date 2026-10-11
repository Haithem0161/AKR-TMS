//! The bills of materials of a bundle, and what changed between two releases.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde_json::Value;

use super::{SBOM_DIR, VEX, read, walk};
use crate::error::CtlError;

const CYCLONEDX: &str = "CycloneDX";

/// Component name to the versions of it a release ships.
pub type Components = BTreeMap<String, BTreeSet<String>>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SbomDiff {
    /// `name version`.
    pub added: Vec<String>,
    pub removed: Vec<String>,
    /// `name old -> new`.
    pub changed: Vec<String>,
}

impl SbomDiff {
    pub fn lines(&self) -> Vec<String> {
        let mut lines = vec![format!(
            "SBOM diff against the installed release: {} added, {} removed, {} changed",
            self.added.len(),
            self.removed.len(),
            self.changed.len()
        )];
        for (mark, names) in [
            ("+", &self.added),
            ("-", &self.removed),
            ("~", &self.changed),
        ] {
            lines.extend(names.iter().map(|name| format!("  {mark} {name}")));
        }
        lines
    }
}

fn is_cyclonedx(document: &Value) -> bool {
    document["bomFormat"] == CYCLONEDX
}

fn collect(components: &Value, into: &mut Components) {
    for component in components.as_array().into_iter().flatten() {
        if let Some(name) = component["name"].as_str() {
            let version = component["version"].as_str().unwrap_or("").to_owned();
            into.entry(name.to_owned()).or_default().insert(version);
        }
        collect(&component["components"], into);
    }
}

/// The SBOM files of a bundle: every CycloneDX document under `sbom/` that lists components. The VEX
/// statement lists vulnerabilities, not components, and is not one of them.
pub fn sbom_files(bundle: &Path) -> Result<Vec<String>, CtlError> {
    let dir = bundle.join(SBOM_DIR);
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut found = Vec::new();
    for file in walk(&dir)? {
        let relative = format!("{SBOM_DIR}/{file}");
        if relative == VEX || !file.ends_with(".json") {
            continue;
        }
        let document: Value =
            serde_json::from_slice(&read(&bundle.join(&relative))?).unwrap_or(Value::Null);
        if is_cyclonedx(&document) && document["components"].is_array() {
            found.push(relative);
        }
    }
    Ok(found)
}

/// `true` when the bundle holds a CycloneDX VEX statement.
pub fn has_vex(bundle: &Path) -> bool {
    std::fs::read(bundle.join(VEX))
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
        .is_some_and(|document| is_cyclonedx(&document) && document["vulnerabilities"].is_array())
}

pub fn components(bundle: &Path) -> Result<Components, CtlError> {
    let mut all = Components::new();
    for file in sbom_files(bundle)? {
        let document: Value = serde_json::from_slice(&read(&bundle.join(&file))?)
            .map_err(|e| CtlError::usage(format!("{file}: {e}")))?;
        collect(&document["components"], &mut all);
    }
    Ok(all)
}

fn joined(versions: &BTreeSet<String>) -> String {
    versions
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn diff(installed: &Components, new: &Components) -> SbomDiff {
    let mut out = SbomDiff::default();
    for (name, versions) in new {
        match installed.get(name) {
            None => out.added.push(format!("{name} {}", joined(versions))),
            Some(before) if before != versions => {
                out.changed
                    .push(format!("{name} {} -> {}", joined(before), joined(versions)));
            }
            Some(_) => {}
        }
    }
    for (name, versions) in installed {
        if !new.contains_key(name) {
            out.removed.push(format!("{name} {}", joined(versions)));
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::*;

    fn bundle(name: &str, components: Value) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("akr-ctl-sbom-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(SBOM_DIR)).unwrap();
        let sbom =
            json!({ "bomFormat": "CycloneDX", "specVersion": "1.6", "components": components });
        std::fs::write(dir.join("sbom/akr-api.cdx.json"), sbom.to_string()).unwrap();
        dir
    }

    #[test]
    fn components_are_read_from_every_sbom_and_not_from_the_vex_statement() {
        let dir = bundle(
            "read",
            json!([
                { "name": "serde", "version": "1.0.229" },
                { "name": "axum", "version": "0.8.9", "components": [{ "name": "matchit", "version": "0.8.4" }] }
            ]),
        );
        assert!(!has_vex(&dir));
        let vex = json!({ "bomFormat": "CycloneDX", "vulnerabilities": [], "components": [{ "name": "ghost" }] });
        std::fs::write(dir.join(VEX), vex.to_string()).unwrap();
        std::fs::write(dir.join("sbom/readme.txt"), "not an sbom").unwrap();

        assert!(has_vex(&dir));
        assert_eq!(sbom_files(&dir).unwrap(), ["sbom/akr-api.cdx.json"]);
        let found = components(&dir).unwrap();
        assert_eq!(
            found.keys().collect::<Vec<_>>(),
            ["axum", "matchit", "serde"]
        );
    }

    #[test]
    fn a_diff_names_what_was_added_removed_and_changed() {
        let installed = components(&bundle(
            "old",
            json!([{ "name": "serde", "version": "1.0.228" }, { "name": "gone", "version": "0.1.0" }, { "name": "same", "version": "2" }]),
        ))
        .unwrap();
        let new = components(&bundle(
            "new",
            json!([{ "name": "serde", "version": "1.0.229" }, { "name": "fresh", "version": "3.1.0" }, { "name": "same", "version": "2" }]),
        ))
        .unwrap();

        let found = diff(&installed, &new);
        assert_eq!(found.added, ["fresh 3.1.0"]);
        assert_eq!(found.removed, ["gone 0.1.0"]);
        assert_eq!(found.changed, ["serde 1.0.228 -> 1.0.229"]);
        assert_eq!(found.lines().len(), 4);
        assert_eq!(diff(&new, &new), SbomDiff::default());
    }
}
