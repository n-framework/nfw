use std::fs;
use std::path::{Path, PathBuf};

use serde_yaml::Value as YamlValue;

use crate::features::check::models::errors::CheckError;
use crate::features::check::services::abstractions::WorkspaceMetadataReader;

const WORKSPACE_METADATA_FILE: &str = "nfw.yaml";

/// File system implementation of WorkspaceMetadataReader.
pub struct FilesystemWorkspaceMetadataReader;

impl FilesystemWorkspaceMetadataReader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FilesystemWorkspaceMetadataReader {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceMetadataReader for FilesystemWorkspaceMetadataReader {
    fn resolve_workspace_root(&self, start_directory: &Path) -> Result<PathBuf, CheckError> {
        let mut candidate = start_directory.to_path_buf();

        loop {
            let workspace_file = candidate.join(WORKSPACE_METADATA_FILE);
            if workspace_file.is_file() {
                return Ok(candidate);
            }

            let Some(parent) = candidate.parent() else {
                break;
            };

            candidate = parent.to_path_buf();
        }

        Err(CheckError::InvalidWorkspaceContext(
            "could not find nfw.yaml in current directory or parent directories. Ensure nfw.yaml exists and contains valid workspace configuration.".to_owned(),
        ))
    }

    fn resolve_service_roots(&self, workspace_root: &Path) -> Result<Vec<PathBuf>, String> {
        let workspace_metadata_path = workspace_root.join(WORKSPACE_METADATA_FILE);
        let metadata_content = fs::read_to_string(&workspace_metadata_path).map_err(|error| {
            format!(
                "failed to read workspace metadata file '{path}': {error}",
                path = workspace_metadata_path.display()
            )
        })?;
        let metadata = serde_yaml::from_str::<YamlValue>(&metadata_content)
            .map_err(|error| format!("failed to parse workspace metadata file '{path}': {error}. Ensure the file contains valid YAML.", path = workspace_metadata_path.display()))?;

        let Some(services) = metadata
            .get("services")
            .and_then(|value| value.as_mapping())
        else {
            return Ok(Vec::new());
        };

        let mut service_roots = Vec::new();
        for service_definition in services.values() {
            let Some(service_mapping) = service_definition.as_mapping() else {
                continue;
            };
            let Some(path_value) = service_mapping
                .get(YamlValue::String("path".to_owned()))
                .and_then(|value| value.as_str())
            else {
                continue;
            };

            let normalized = path_value.trim();
            if normalized.is_empty() {
                continue;
            }

            service_roots.push(workspace_root.join(normalized));
        }

        service_roots.sort();
        service_roots.dedup();
        Ok(service_roots)
    }

    fn read_manifest_content(&self, path: &Path) -> Result<String, String> {
        fs::read_to_string(path).map_err(|error| {
            format!(
                "failed to read manifest file '{path}': {error}",
                path = path.display()
            )
        })
    }

    fn read_source_file(&self, path: &Path) -> Result<String, String> {
        fs::read_to_string(path).map_err(|error| {
            format!(
                "failed to read source file '{path}': {error}",
                path = path.display()
            )
        })
    }
}
