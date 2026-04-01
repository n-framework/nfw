use std::path::PathBuf;

use nframework_nfw_application::features::cli::configuration::abstraction::path_resolver::PathResolver;

#[derive(Debug, Default, Clone, Copy)]
pub struct DirsPathResolver;

impl DirsPathResolver {
    pub fn new() -> Self {
        Self
    }

    fn resolve_nfw_home_directory() -> Result<PathBuf, String> {
        let home_directory = dirs::home_dir()
            .ok_or_else(|| "failed to resolve user home directory for nfw paths".to_owned())?;
        Ok(home_directory.join(".nfw"))
    }
}

impl PathResolver for DirsPathResolver {
    fn cache_dir(&self) -> Result<PathBuf, String> {
        Self::resolve_nfw_home_directory()
    }

    fn config_dir(&self) -> Result<PathBuf, String> {
        Self::resolve_nfw_home_directory()
    }
}
