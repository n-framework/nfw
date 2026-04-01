use std::path::PathBuf;

pub trait PathResolver {
    fn cache_dir(&self) -> Result<PathBuf, String>;
    fn config_dir(&self) -> Result<PathBuf, String>;
}
