use std::fs;
use std::path::Path;

use crate::features::check::models::{FindingType, ValidationFinding};
use crate::features::check::services::abstractions::ProjectManifestCollector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectManifestKind {
    CSharp,
    Rust,
    Node,
}

fn detect_manifest_kind(path: &Path) -> Option<ProjectManifestKind> {
    let file_name = path.file_name().and_then(|value| value.to_str())?;

    if file_name.ends_with(".csproj") {
        return Some(ProjectManifestKind::CSharp);
    }

    if file_name.eq_ignore_ascii_case("Cargo.toml") {
        return Some(ProjectManifestKind::Rust);
    }

    if file_name.eq_ignore_ascii_case("package.json") {
        return Some(ProjectManifestKind::Node);
    }

    None
}

fn should_skip_path(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };

    matches!(
        file_name,
        ".git"
            | "bin"
            | "obj"
            | "target"
            | "node_modules"
            | ".idea"
            | ".vscode"
            | ".venv"
            | "venv"
            | "__pycache__"
    )
}

/// File system implementation of ProjectManifestCollector.
pub struct FilesystemProjectManifestCollector;

impl FilesystemProjectManifestCollector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FilesystemProjectManifestCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectManifestCollector for FilesystemProjectManifestCollector {
    fn collect_manifests(
        &self,
        root: &Path,
        errors: &mut Vec<ValidationFinding>,
    ) -> Vec<std::path::PathBuf> {
        let mut manifests = Vec::new();
        self.collect_recursive(root, &mut manifests, errors);
        manifests.sort();
        manifests
    }
}

impl FilesystemProjectManifestCollector {
    fn collect_recursive(
        &self,
        root: &Path,
        output: &mut Vec<std::path::PathBuf>,
        errors: &mut Vec<ValidationFinding>,
    ) {
        let entries = match fs::read_dir(root) {
            Ok(entries) => entries,
            Err(error) => {
                errors.push(ValidationFinding {
                    finding_type: FindingType::UnreadableArtifact,
                    location: root.to_path_buf(),
                    offending_value: format!("failed to read directory: {error}"),
                    violated_rule_id: Some("filesystem:directory-read".to_owned()),
                    remediation_hint: "check directory permissions and ensure path exists"
                        .to_owned(),
                });
                return;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if should_skip_path(&path) {
                continue;
            }

            if path.is_dir() {
                self.collect_recursive(&path, output, errors);
                continue;
            }

            if detect_manifest_kind(&path).is_some() {
                output.push(path);
            }
        }
    }
}
