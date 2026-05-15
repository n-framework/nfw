use std::path::{Path, PathBuf};

pub trait GeneratorCatalogSource {
    fn discover_generator_directories(&self, source_root: &Path) -> Result<Vec<PathBuf>, String>;
    fn read_generator_metadata(&self, generator_directory: &Path) -> Result<String, String>;
}
