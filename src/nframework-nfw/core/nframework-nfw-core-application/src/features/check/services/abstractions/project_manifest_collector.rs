use std::path::Path;

use crate::features::check::models::ValidationFinding;

/// Trait for collecting project manifests from a workspace.
/// This abstraction allows the application layer to remain pure while
/// implementations handle file system I/O in the infrastructure layer.
pub trait ProjectManifestCollector: Send + Sync {
    /// Collects all project manifests from the given root directory.
    /// Returns a vector of manifest paths that can be validated.
    ///
    /// Any errors encountered during collection are reported as ValidationFindings
    /// rather than failing the entire operation, allowing validation to continue
    /// with whatever manifests were successfully discovered.
    fn collect_manifests(
        &self,
        root: &Path,
        errors: &mut Vec<ValidationFinding>,
    ) -> Vec<std::path::PathBuf>;
}
