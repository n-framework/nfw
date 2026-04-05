use std::fs;
use std::path::Path;

#[derive(Debug, Default, Clone, Copy)]
pub struct ServiceGenerationCleanup;

impl ServiceGenerationCleanup {
    pub fn new() -> Self {
        Self
    }

    pub fn cleanup_output(&self, output_root: &Path) -> Result<(), String> {
        if !output_root.exists() {
            return Ok(());
        }

        fs::remove_dir_all(output_root).map_err(|error| {
            format!(
                "failed to remove '{}' during cleanup: {error}",
                output_root.display()
            )
        })
    }
}
