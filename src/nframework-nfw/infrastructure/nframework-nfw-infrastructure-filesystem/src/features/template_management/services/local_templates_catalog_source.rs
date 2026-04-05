use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use glob::Pattern;
use nframework_nfw_core_application::features::template_management::constants::{source, template};
use nframework_nfw_core_application::features::template_management::services::abstractions::template_catalog_source::TemplateCatalogSource;

use crate::features::template_management::services::placeholder_detector::PlaceholderDetector;

#[derive(Debug, Default, Clone, Copy)]
pub struct LocalTemplatesCatalogSource {
    placeholder_detector: PlaceholderDetector,
}

impl LocalTemplatesCatalogSource {
    pub fn new(placeholder_detector: PlaceholderDetector) -> Self {
        Self {
            placeholder_detector,
        }
    }

    pub fn collect_content_entries(
        &self,
        template_directory: &Path,
    ) -> Result<Vec<PathBuf>, String> {
        let content_directory = template_directory.join(template::CONTENT_DIR);
        if !content_directory.is_dir() {
            return Err(format!(
                "template directory '{}' is missing required 'content/' directory",
                template_directory.display()
            ));
        }

        let ignore_patterns = load_ignore_patterns(template_directory)?;
        let mut entries =
            collect_entries_recursive(&content_directory, &content_directory, &ignore_patterns)?;

        if entries.is_empty() {
            return Err(format!(
                "template directory '{}' has an empty 'content/' directory",
                template_directory.display()
            ));
        }

        entries.sort();
        Ok(entries)
    }

    pub fn detect_placeholders(&self, template_directory: &Path) -> Result<Vec<String>, String> {
        let content_directory = template_directory.join(template::CONTENT_DIR);
        let content_entries = self.collect_content_entries(template_directory)?;
        let mut placeholders = BTreeSet::new();

        for entry in content_entries {
            let relative_path = entry
                .strip_prefix(&content_directory)
                .map_err(|error| {
                    format!(
                        "failed to resolve content-relative path for '{}': {error}",
                        entry.display()
                    )
                })?
                .to_path_buf();

            for placeholder in self.placeholder_detector.detect_in_path(&relative_path) {
                placeholders.insert(placeholder);
            }
        }

        Ok(placeholders.into_iter().collect())
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
                "directory '{}' is not a valid template directory; expected template.yaml and content/",
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

#[derive(Debug, Default)]
struct IgnorePatterns {
    directory_prefixes: Vec<PathBuf>,
    glob_patterns: Vec<Pattern>,
}

fn collect_entries_recursive(
    root_directory: &Path,
    current_directory: &Path,
    ignore_patterns: &IgnorePatterns,
) -> Result<Vec<PathBuf>, String> {
    let directory_entries = fs::read_dir(current_directory).map_err(|error| {
        format!(
            "failed to read content directory '{}': {error}",
            current_directory.display()
        )
    })?;

    let mut entries = Vec::new();

    for directory_entry in directory_entries {
        let entry_path = directory_entry
            .map_err(|error| {
                format!(
                    "failed to read an entry in content directory '{}': {error}",
                    current_directory.display()
                )
            })?
            .path();

        let relative_path = entry_path
            .strip_prefix(root_directory)
            .map_err(|error| {
                format!(
                    "failed to build content-relative path for '{}': {error}",
                    entry_path.display()
                )
            })?
            .to_path_buf();

        if should_ignore(&relative_path, ignore_patterns) {
            continue;
        }

        entries.push(entry_path.clone());

        if entry_path.is_dir() {
            let sub_entries =
                collect_entries_recursive(root_directory, &entry_path, ignore_patterns)?;
            entries.extend(sub_entries);
        }
    }

    Ok(entries)
}

fn should_ignore(relative_path: &Path, ignore_patterns: &IgnorePatterns) -> bool {
    if ignore_patterns
        .directory_prefixes
        .iter()
        .any(|directory_prefix| relative_path.starts_with(directory_prefix))
    {
        return true;
    }

    ignore_patterns
        .glob_patterns
        .iter()
        .any(|glob_pattern| glob_pattern.matches_path(relative_path))
}

fn load_ignore_patterns(template_directory: &Path) -> Result<IgnorePatterns, String> {
    let ignore_file_path = template_directory.join(template::IGNORE_FILE);
    if !ignore_file_path.is_file() {
        return Ok(IgnorePatterns::default());
    }

    let ignore_content = fs::read_to_string(&ignore_file_path).map_err(|error| {
        format!(
            "failed to read ignore file '{}': {error}",
            ignore_file_path.display()
        )
    })?;

    let mut patterns = IgnorePatterns::default();
    for line in ignore_content.lines() {
        let normalized_line = line.trim();
        if normalized_line.is_empty() || normalized_line.starts_with('#') {
            continue;
        }

        let normalized_line = normalized_line.trim_start_matches('/');
        if normalized_line.is_empty() {
            continue;
        }

        if normalized_line.ends_with('/') {
            let directory_prefix = normalized_line.trim_end_matches('/');
            if !directory_prefix.is_empty() {
                patterns
                    .directory_prefixes
                    .push(PathBuf::from(directory_prefix));
                patterns
                    .glob_patterns
                    .push(parse_glob_pattern(&format!("{directory_prefix}/**"))?);
                patterns
                    .glob_patterns
                    .push(parse_glob_pattern(&format!("**/{directory_prefix}/**"))?);
            }
            continue;
        }

        patterns
            .glob_patterns
            .push(parse_glob_pattern(normalized_line)?);
        if !normalized_line.contains('/') {
            patterns
                .glob_patterns
                .push(parse_glob_pattern(&format!("**/{normalized_line}"))?);
        }
    }

    Ok(patterns)
}

fn parse_glob_pattern(value: &str) -> Result<Pattern, String> {
    Pattern::new(value).map_err(|error| format!("invalid .nfwignore pattern '{value}': {error}"))
}

fn is_template_directory(path: &Path) -> bool {
    has_required_template_structure(path)
}

/// Checks if a directory has the required template structure (metadata file and content directory)
fn has_required_template_structure(path: &Path) -> bool {
    has_template_metadata(path) && has_content_directory(path)
}

/// Checks if a directory contains the required template metadata file
fn has_template_metadata(path: &Path) -> bool {
    path.join(template::METADATA_FILE).is_file()
}

/// Checks if a directory contains the required content directory
fn has_content_directory(path: &Path) -> bool {
    path.join(template::CONTENT_DIR).is_dir()
}
