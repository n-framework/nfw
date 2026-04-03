use std::path::PathBuf;
use nframework_nfw_application::features::workspace_management::services::abstraction::working_directory_provider::WorkingDirectoryProvider;

/// Standard implementation of WorkingDirectoryProvider that uses std::env.
#[derive(Debug, Clone, Copy)]
pub struct StandardWorkingDirectoryProvider;

impl StandardWorkingDirectoryProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StandardWorkingDirectoryProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkingDirectoryProvider for StandardWorkingDirectoryProvider {
    fn current_dir(&self) -> Result<PathBuf, String> {
        std::env::current_dir()
            .map_err(|e| format!("failed to determine current directory: {e}"))
    }
}
