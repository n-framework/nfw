use std::fs;
use std::path::{Path, PathBuf};

use n_framework_nfw_core_application::features::template_management::constants::{source, template};
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_catalog_source::TemplateCatalogSource;

#[derive(Debug, Default, Clone, Copy)]
pub struct LocalTemplatesCatalogSource;

impl LocalTemplatesCatalogSource {
    pub fn new() -> Self {
        Self
    }
}

impl TemplateCatalogSource for LocalTemplatesCatalogSource {
    fn discover_template_directories(&self, source_root: &Path) -> Result<Vec<PathBuf>, String> {
        if !source_root.is_dir() {
            return Err(format!(
                "template source root '{}' does not exist or is not a directory",
                source_root.display()
            ));
        }

        let mut scan_roots = vec![source_root.to_path_buf()];
        let source_templates_root = source_root.join(source::TEMPLATES_ROOT_DIR);
        if source_templates_root.is_dir() {
            scan_roots.push(source_templates_root);
        }

        let mut directories = Vec::new();
        for scan_root in scan_roots {
            let mut discovered = discover_templates_in_scan_root(&scan_root)?;
            directories.append(&mut discovered);
        }

        directories.sort();
        directories.dedup();
        Ok(directories)
    }

    fn read_template_metadata(&self, template_directory: &Path) -> Result<String, String> {
        if !is_template_directory(template_directory) {
            return Err(format!(
                "directory '{}' is not a valid template directory; expected template.yaml",
                template_directory.display()
            ));
        }

        let metadata_path = template_directory.join(template::METADATA_FILE);
        fs::read_to_string(&metadata_path).map_err(|error| {
            format!(
                "failed to read template metadata '{}': {error}",
                metadata_path.display()
            )
        })
    }
}

fn discover_templates_in_scan_root(scan_root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut directories = Vec::new();

    if is_template_directory(scan_root) {
        directories.push(scan_root.to_path_buf());
    }

    let entries = fs::read_dir(scan_root).map_err(|error| {
        format!(
            "failed to read template source root '{}': {error}",
            scan_root.display()
        )
    })?;

    for entry in entries {
        let path = entry
            .map_err(|error| {
                format!(
                    "failed to read an entry from source root '{}': {error}",
                    scan_root.display()
                )
            })?
            .path();

        if path.is_dir() && is_template_directory(&path) {
            directories.push(path);
        }
    }

    Ok(directories)
}

fn is_template_directory(path: &Path) -> bool {
    has_required_template_structure(path)
}

/// Checks if a directory has the required template structure (metadata file)
fn has_required_template_structure(path: &Path) -> bool {
    has_template_metadata(path)
}

/// Checks if a directory contains the required template metadata file
fn has_template_metadata(path: &Path) -> bool {
    path.join(template::METADATA_FILE).is_file()
}
