use std::path::PathBuf;

/// Abstracts working directory access to follow clean architecture.
/// Application layer depends on this abstraction; infrastructure provides the implementation.
pub trait WorkingDirectoryProvider {
    /// Returns the current working directory.
    fn current_dir(&self) -> Result<PathBuf, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test implementation that returns a fixed path for testing.
    #[derive(Debug, Clone)]
    pub struct TestWorkingDirectoryProvider {
        pub path: PathBuf,
    }

    impl TestWorkingDirectoryProvider {
        pub fn new(path: impl Into<PathBuf>) -> Self {
            Self {
                path: path.into(),
            }
        }
    }

    impl WorkingDirectoryProvider for TestWorkingDirectoryProvider {
        fn current_dir(&self) -> Result<PathBuf, String> {
            Ok(self.path.clone())
        }
    }
}
