use std::path::{Path, PathBuf};

use crate::features::service_management::models::errors::add_service_error::AddServiceError;

const WORKSPACE_METADATA_FILE: &str = "nfw.yaml";

#[derive(Debug, Default, Clone, Copy)]
pub struct AddServiceWorkspaceContextGuard;

impl AddServiceWorkspaceContextGuard {
    pub fn new() -> Self {
        Self
    }

    pub fn ensure_workspace_root(
        &self,
        current_directory: &Path,
    ) -> Result<PathBuf, AddServiceError> {
        let mut candidate = current_directory.to_path_buf();

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

        Err(AddServiceError::InvalidWorkspaceContext(
            "could not find nfw.yaml in current directory or parent directories".to_owned(),
        ))
    }
}
