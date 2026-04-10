use std::fs;
use std::path::Path;

use crate::features::check::models::{FindingType, ValidationFinding};
use crate::features::check::services::abstractions::SourceFileCollector;

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

/// File system implementation of SourceFileCollector.
pub struct FilesystemSourceFileCollector;

impl FilesystemSourceFileCollector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FilesystemSourceFileCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceFileCollector for FilesystemSourceFileCollector {
    fn collect_source_files(
        &self,
        project_directory: &Path,
        errors: &mut Vec<ValidationFinding>,
    ) -> Vec<std::path::PathBuf> {
        let mut results = Vec::new();
        self.collect_recursive(project_directory, &mut results, errors);
        results.sort();
        results
    }
}

impl FilesystemSourceFileCollector {
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

            let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
                continue;
            };

            let supported = matches!(
                extension.to_ascii_lowercase().as_str(),
                "cs" | "rs" | "js" | "jsx" | "ts" | "tsx" | "py" | "go"
            );

            if supported {
                output.push(path);
            }
        }
    }
}
