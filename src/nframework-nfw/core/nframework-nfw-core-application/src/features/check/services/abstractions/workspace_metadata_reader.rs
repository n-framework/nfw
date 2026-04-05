use std::path::{Path, PathBuf};

use crate::features::check::models::errors::CheckError;

/// Trait for reading workspace metadata (nfw.yaml).
/// This abstraction allows the application layer to remain pure while
/// implementations handle file I/O and YAML parsing in the infrastructure layer.
pub trait WorkspaceMetadataReader: Send + Sync {
    /// Resolves the workspace root by searching for nfw.yaml in the current
    /// directory and parent directories.
    fn resolve_workspace_root(&self, start_directory: &Path) -> Result<PathBuf, CheckError>;

    /// Reads service root paths from the workspace metadata file.
    /// Returns a vector of absolute paths to service directories.
    fn resolve_service_roots(&self, workspace_root: &Path) -> Result<Vec<PathBuf>, String>;

    /// Reads the content of a project manifest file (go.mod, Cargo.toml, etc.).
    fn read_manifest_content(&self, path: &Path) -> Result<String, String>;

    /// Reads the content of a source file.
    fn read_source_file(&self, path: &Path) -> Result<String, String>;
}
