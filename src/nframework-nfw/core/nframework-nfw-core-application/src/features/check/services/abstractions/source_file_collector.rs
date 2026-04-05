use std::path::Path;

use crate::features::check::models::ValidationFinding;

/// Trait for collecting source files from a project directory.
/// This abstraction allows the application layer to remain pure while
/// implementations handle file system I/O in the infrastructure layer.
pub trait SourceFileCollector: Send + Sync {
    /// Collects all source files from the given project directory.
    /// Returns a vector of source file paths that can be validated for namespace usage.
    ///
    /// Any errors encountered during collection are reported as ValidationFindings
    /// rather than failing the entire operation, allowing validation to continue
    /// with whatever files were successfully discovered.
    fn collect_source_files(
        &self,
        project_directory: &Path,
        errors: &mut Vec<ValidationFinding>,
    ) -> Vec<std::path::PathBuf>;
}
