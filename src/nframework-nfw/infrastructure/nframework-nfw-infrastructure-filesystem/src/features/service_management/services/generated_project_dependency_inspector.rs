use std::fs;
use std::path::Path;

use nframework_nfw_application::features::service_management::services::abstraction::generated_project_dependency_inspector::GeneratedProjectDependencyInspector;

#[derive(Debug, Default, Clone, Copy)]
pub struct FileSystemGeneratedProjectDependencyInspector;

impl FileSystemGeneratedProjectDependencyInspector {
    pub fn new() -> Self {
        Self
    }
}

impl GeneratedProjectDependencyInspector for FileSystemGeneratedProjectDependencyInspector {
    fn inspect_dependencies(&self, service_root: &Path) -> Result<Vec<(String, String)>, String> {
        let mut edges = Vec::<(String, String)>::new();
        for project_file in find_files_by_extension(service_root, "csproj")? {
            let Some(source_layer) = detect_layer_from_path(&project_file) else {
                continue;
            };

            let content = fs::read_to_string(&project_file).map_err(|error| {
                format!(
                    "failed to read generated project file '{}': {error}",
                    project_file.display()
                )
            })?;

            for include_path in extract_project_reference_includes(&content) {
                let Some(target_layer) = detect_layer_from_reference(&include_path) else {
                    continue;
                };

                edges.push((source_layer.clone(), target_layer));
            }
        }

        Ok(edges)
    }
}

fn detect_layer_from_path(path: &Path) -> Option<String> {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .find(|segment| {
            matches!(
                segment.as_str(),
                "Domain" | "Application" | "Infrastructure" | "Api"
            )
        })
}

fn detect_layer_from_reference(value: &str) -> Option<String> {
    if value.contains(".Domain") || value.contains("/Domain/") || value.contains("\\Domain\\") {
        return Some("Domain".to_owned());
    }

    if value.contains(".Application")
        || value.contains("/Application/")
        || value.contains("\\Application\\")
    {
        return Some("Application".to_owned());
    }

    if value.contains(".Infrastructure")
        || value.contains("/Infrastructure/")
        || value.contains("\\Infrastructure\\")
    {
        return Some("Infrastructure".to_owned());
    }

    if value.contains(".Api") || value.contains("/Api/") || value.contains("\\Api\\") {
        return Some("Api".to_owned());
    }

    None
}

fn extract_project_reference_includes(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let include_key = "Include=\"";
            let start_index = line.find(include_key)? + include_key.len();
            let remaining = &line[start_index..];
            let end_index = remaining.find('"')?;
            Some(remaining[..end_index].to_owned())
        })
        .collect()
}

fn find_files_by_extension(
    root: &Path,
    extension: &str,
) -> Result<Vec<std::path::PathBuf>, String> {
    let mut files = Vec::new();
    let mut directories = vec![root.to_path_buf()];

    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(&directory).map_err(|error| {
            format!(
                "failed to read directory '{}': {error}",
                directory.display()
            )
        })? {
            let path = entry
                .map_err(|error| {
                    format!(
                        "failed to read entry in directory '{}': {error}",
                        directory.display()
                    )
                })?
                .path();

            if path.is_dir() {
                directories.push(path);
                continue;
            }

            if path
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|value| value.eq_ignore_ascii_case(extension))
            {
                files.push(path);
            }
        }
    }

    Ok(files)
}
