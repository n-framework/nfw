use std::path::PathBuf;

/// Abstracts working directory access to follow clean architecture.
/// Application layer depends on this abstraction; infrastructure provides the implementation.
pub trait WorkingDirectoryProvider {
    /// Returns the current working directory.
    fn current_dir(&self) -> Result<PathBuf, String>;
}
