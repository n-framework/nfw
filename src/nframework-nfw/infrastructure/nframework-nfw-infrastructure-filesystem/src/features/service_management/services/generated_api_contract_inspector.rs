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
        let api_root = service_root.join("Api");
        if !api_root.exists() {
            return Err(format!(
                "API layer directory '{}' was not generated",
                api_root.display()
            ));
        }

        let mut has_live = false;
        let mut has_ready = false;

        for file in find_text_files(&api_root)? {
            let content = fs::read_to_string(&file).map_err(|error| {
                format!(
                    "failed to read generated API file '{}': {error}",
                    file.display()
                )
            })?;

            if content.contains("/health/live") {
                has_live = true;
            }

            if content.contains("/health/ready") {
                has_ready = true;
            }

            if has_live && has_ready {
                return Ok(());
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

fn find_text_files(root: &Path) -> Result<Vec<std::path::PathBuf>, String> {
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

            files.push(path);
        }
    }

    Ok(files)
}
