use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewCommandResolution {
    pub workspace_name: String,
    pub generator_id: String,
    pub generator_cache_path: PathBuf,
    pub namespace_base: String,
    pub output_path: PathBuf,
}
