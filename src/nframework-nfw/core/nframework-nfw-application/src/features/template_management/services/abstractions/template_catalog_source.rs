use std::path::{Path, PathBuf};

pub trait TemplateCatalogSource {
    fn discover_template_directories(&self, source_root: &Path) -> Result<Vec<PathBuf>, String>;
    fn read_template_metadata(&self, template_directory: &Path) -> Result<String, String>;
}
