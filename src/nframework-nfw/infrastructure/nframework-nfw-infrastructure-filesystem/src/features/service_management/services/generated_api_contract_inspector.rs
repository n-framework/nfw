use std::fs;
use std::path::Path;

use nframework_nfw_application::features::service_management::services::abstraction::generated_api_contract_inspector::GeneratedApiContractInspector;

#[derive(Debug, Default, Clone, Copy)]
pub struct FileSystemGeneratedApiContractInspector;

impl FileSystemGeneratedApiContractInspector {
    pub fn new() -> Self {
        Self
    }
}

impl GeneratedApiContractInspector for FileSystemGeneratedApiContractInspector {
    fn assert_health_endpoints(&self, service_root: &Path) -> Result<(), String> {
        let api_roots = find_api_roots(service_root)?;
        if api_roots.is_empty() {
            return Err(format!(
                "API layer directory '{}' was not generated",
                service_root.join("Api").display()
            ));
        }

        let mut has_live = false;
        let mut has_ready = false;

        for api_root in &api_roots {
            for file in find_source_files(api_root)? {
                let content = fs::read_to_string(&file).map_err(|error| {
                    format!(
                        "failed to read generated API file '{}': {error}",
                        file.display()
                    )
                })?;

                if contains_live_health_mapping(&content) {
                    has_live = true;
                }

                if contains_ready_health_mapping(&content) {
                    has_ready = true;
                }

                if has_live && has_ready {
                    return Ok(());
                }
            }
        }

        if !has_live && !has_ready {
            return Err("missing '/health/live' and '/health/ready' endpoints".to_owned());
        }

        if !has_live {
            return Err("missing '/health/live' endpoint".to_owned());
        }

        Err("missing '/health/ready' endpoint".to_owned())
    }
}

fn find_api_roots(root: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut api_roots = Vec::new();
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

            if !path.is_dir() {
                continue;
            }

            let directory_name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default();

            if directory_name == "Api" || directory_name.ends_with(".WebApi") {
                api_roots.push(path.clone());
            }

            directories.push(path);
        }
    }

    Ok(api_roots)
}

fn find_source_files(root: &Path) -> Result<Vec<std::path::PathBuf>, String> {
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

            let is_csharp = path
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|value| value.eq_ignore_ascii_case("cs"));
            if is_csharp {
                files.push(path);
            }
        }
    }

    Ok(files)
}

fn contains_live_health_mapping(content: &str) -> bool {
    content.contains("/health/live")
        || (content.contains("/health") && content.contains("/live"))
        || content.contains("MapHealthCheckEndpoints()")
}

fn contains_ready_health_mapping(content: &str) -> bool {
    content.contains("/health/ready")
        || (content.contains("/health") && content.contains("/ready"))
        || content.contains("MapHealthCheckEndpoints()")
}
