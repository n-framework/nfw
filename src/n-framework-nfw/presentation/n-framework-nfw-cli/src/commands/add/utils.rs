use std::path::Path;

use crate::cli_error::CliError;

pub fn find_presentation_layers(
    workspace_root: &Path,
    service_path: &str,
    service_name: &str,
) -> Result<Vec<String>, CliError> {
    let presentation_dir = workspace_root.join(service_path).join("src/presentation");

    let mut layers = Vec::new();
    if presentation_dir.exists() {
        let entries = std::fs::read_dir(&presentation_dir).map_err(|e| {
            CliError::internal(format!(
                "Failed to read presentation directory at {}: {}. Check permissions.",
                presentation_dir.display(),
                e
            ))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                CliError::internal(format!("Error reading presentation layer entry: {}", e))
            })?;

            let file_type = entry.file_type().map_err(|e| {
                CliError::internal(format!(
                    "Failed to check file type for {}: {}",
                    entry.path().display(),
                    e
                ))
            })?;

            if file_type.is_dir() {
                let name = entry.file_name().into_string().map_err(|os_str| {
                    CliError::internal(format!("Invalid UTF-8 in directory name: {:?}", os_str))
                })?;
                let prefix = format!("{}.Presentation.", service_name);
                if name.starts_with(&prefix) {
                    let layer = name.replace(&prefix, "");
                    layers.push(layer);
                }
            }
        }
    }

    Ok(layers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_presentation_layers_empty() {
        let temp_dir = TempDir::new().unwrap();

        let layers =
            find_presentation_layers(temp_dir.path(), "services/MyService", "MyService").unwrap();
        assert!(layers.is_empty());
    }

    #[test]
    fn test_find_presentation_layers_with_layers() {
        let temp_dir = TempDir::new().unwrap();
        let presentation_dir = temp_dir.path().join("services/MyService/src/presentation");
        fs::create_dir_all(&presentation_dir).unwrap();

        fs::create_dir(presentation_dir.join("MyService.Presentation.WebApi")).unwrap();
        fs::create_dir(presentation_dir.join("MyService.Presentation.Admin")).unwrap();
        fs::create_dir(presentation_dir.join("OtherDir")).unwrap();

        let layers =
            find_presentation_layers(temp_dir.path(), "services/MyService", "MyService").unwrap();
        assert_eq!(layers.len(), 2);
        assert!(layers.contains(&"WebApi".to_string()));
        assert!(layers.contains(&"Admin".to_string()));
    }

    #[test]
    fn test_find_presentation_layers_permission_error() {
        let temp_dir = TempDir::new().unwrap();
        let presentation_dir = temp_dir.path().join("services/MyService/src/presentation");
        fs::create_dir_all(&presentation_dir).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&presentation_dir).unwrap().permissions();
            perms.set_mode(0o000);
            fs::set_permissions(&presentation_dir, perms.clone()).unwrap();

            let result =
                find_presentation_layers(temp_dir.path(), "services/MyService", "MyService");
            assert!(result.is_err());

            perms.set_mode(0o755);
            fs::set_permissions(&presentation_dir, perms).unwrap();
        }
    }
}
