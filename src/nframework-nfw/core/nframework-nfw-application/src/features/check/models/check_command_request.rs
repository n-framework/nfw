use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckCommandRequest {
    pub start_directory: PathBuf,
}

impl CheckCommandRequest {
    /// Creates a new CheckCommandRequest with the provided start directory.
    /// Returns an error if the path does not exist or is not a directory.
    pub fn new(start_directory: PathBuf) -> Result<Self, String> {
        if !start_directory.exists() {
            return Err(format!(
                "start directory does not exist: '{}'",
                start_directory.display()
            ));
        }
        if !start_directory.is_dir() {
            return Err(format!(
                "start directory must be a directory, not a file: '{}'",
                start_directory.display()
            ));
        }

        Ok(Self { start_directory })
    }

    /// Creates a new CheckCommandRequest using the current directory.
    pub fn current_dir() -> Result<Self, String> {
        let current = std::env::current_dir()
            .map_err(|error| format!("failed to get current directory: {error}"))?;
        Self::new(current)
    }
}
