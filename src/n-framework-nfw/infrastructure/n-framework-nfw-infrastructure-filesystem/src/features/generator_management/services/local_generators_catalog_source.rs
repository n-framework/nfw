use std::fs;
use std::path::{Path, PathBuf};

use n_framework_nfw_core_application::features::generator_management::constants::{source, generator};
use n_framework_nfw_core_application::features::generator_management::services::abstractions::generator_catalog_source::GeneratorCatalogSource;

#[derive(Debug, Default, Clone, Copy)]
pub struct LocalGeneratorsCatalogSource;

impl LocalGeneratorsCatalogSource {
    pub fn new() -> Self {
        Self
    }
}

impl GeneratorCatalogSource for LocalGeneratorsCatalogSource {
    fn discover_generator_directories(&self, source_root: &Path) -> Result<Vec<PathBuf>, String> {
        if !source_root.is_dir() {
            return Err(format!(
                "generator source root '{}' does not exist or is not a directory",
                source_root.display()
            ));
        }

        let mut scan_roots = vec![source_root.to_path_buf()];
        let source_generators_root = source_root.join(source::GENERATORS_ROOT_DIR);
        if source_generators_root.is_dir() {
            scan_roots.push(source_generators_root);
        }

        let mut directories = Vec::new();
        for scan_root in scan_roots {
            let mut discovered = discover_generators_in_scan_root(&scan_root)?;
            directories.append(&mut discovered);
        }

        directories.sort();
        directories.dedup();
        Ok(directories)
    }

    fn read_generator_metadata(&self, generator_directory: &Path) -> Result<String, String> {
        if !is_generator_directory(generator_directory) {
            return Err(format!(
                "directory '{}' is not a valid generator directory; expected generator.yaml",
                generator_directory.display()
            ));
        }

        let metadata_path = generator_directory.join(generator::METADATA_FILE);
        fs::read_to_string(&metadata_path).map_err(|error| {
            format!(
                "failed to read generator metadata '{}': {error}",
                metadata_path.display()
            )
        })
    }
}

fn discover_generators_in_scan_root(scan_root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut directories = Vec::new();

    if is_generator_directory(scan_root) {
        directories.push(scan_root.to_path_buf());
    }

    let entries = fs::read_dir(scan_root).map_err(|error| {
        format!(
            "failed to read generator source root '{}': {error}",
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

        if path.is_dir() && is_generator_directory(&path) {
            directories.push(path);
        }
    }

    Ok(directories)
}

fn is_generator_directory(path: &Path) -> bool {
    has_required_generator_structure(path)
}

/// Checks if a directory has the required generator structure (metadata file)
fn has_required_generator_structure(path: &Path) -> bool {
    has_generator_metadata(path)
}

/// Checks if a directory contains the required generator metadata file
fn has_generator_metadata(path: &Path) -> bool {
    path.join(generator::METADATA_FILE).is_file()
}
