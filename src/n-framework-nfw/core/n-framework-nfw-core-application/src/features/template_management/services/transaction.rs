use std::fs;
use std::path::{Path, PathBuf};

use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;

pub struct FileTracker {
    baseline_files: Vec<PathBuf>,
    output_root: PathBuf,
}

impl FileTracker {
    pub fn new(output_root: &Path) -> Result<Self, std::io::Error> {
        let baseline_files = Self::scan_directory(output_root)?;
        Ok(Self {
            baseline_files,
            output_root: output_root.to_path_buf(),
        })
    }

    fn scan_directory(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut files = Vec::new();
        if dir.exists() {
            let entries = fs::read_dir(dir)?;
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    files.push(path);
                } else if path.is_dir() {
                    files.extend(Self::scan_directory(&path)?);
                }
            }
        }
        Ok(files)
    }

    pub fn get_created_files(&self) -> Vec<PathBuf> {
        let current_files = match Self::scan_directory(&self.output_root) {
            Ok(files) => files,
            Err(_) => return Vec::new(),
        };

        current_files
            .into_iter()
            .filter(|path| !self.baseline_files.contains(path))
            .collect()
    }

    pub fn cleanup_created_files(&self) -> Result<(), AddArtifactError> {
        let created_files = self.get_created_files();
        let mut leftover_files = Vec::new();

        for file in &created_files {
            if file.exists()
                && let Err(e) = fs::remove_file(file)
            {
                tracing::warn!(
                    "Failed to remove file during rollback: {}: {}",
                    file.display(),
                    e
                );
                leftover_files.push(file.display().to_string());
            }
        }

        if !leftover_files.is_empty() {
            return Err(AddArtifactError::WorkspaceError(format!(
                "Rollback partially failed. Manual cleanup required for: {:?}",
                leftover_files
            )));
        }

        Ok(())
    }
}

pub struct YamlBackup {
    original_content: String,
    yaml_path: PathBuf,
}

impl YamlBackup {
    pub fn create(yaml_path: &Path) -> Result<Self, AddArtifactError> {
        let original_content = fs::read_to_string(yaml_path).map_err(|e| {
            AddArtifactError::WorkspaceError(format!("Failed to backup nfw.yaml: {}", e))
        })?;

        Ok(Self {
            original_content,
            yaml_path: yaml_path.to_path_buf(),
        })
    }

    pub fn restore(&self) -> Result<(), AddArtifactError> {
        fs::write(&self.yaml_path, &self.original_content).map_err(|e| {
            AddArtifactError::WorkspaceError(format!("Failed to restore nfw.yaml backup: {}", e))
        })?;

        tracing::debug!("Restored nfw.yaml from backup");
        Ok(())
    }
}

#[cfg(test)]
#[path = "transaction.tests.rs"]
mod tests;
