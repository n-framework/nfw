use std::path::{Path, PathBuf};
use serde_yaml::Value as YamlValue;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;

#[derive(Debug, Clone, Default)]
pub struct FileSystemTemplateRootResolver;

impl FileSystemTemplateRootResolver {
    pub fn new() -> Self {
        Self
    }
}

impl TemplateRootResolver for FileSystemTemplateRootResolver {
    fn resolve(
        &self,
        nfw_yaml: &YamlValue,
        template_id: &str,
        workspace_root: &Path,
    ) -> Result<PathBuf, String> {
        // Check if there is a 'template_sources.local' workspace setting mapping to a local directory
        if let Some(sources) = nfw_yaml.get("template_sources")
            && let Some(local) = sources.get("local")
            && let Some(local_path) = local.as_str()
        {
            let candidate = workspace_root.join(local_path).join(template_id);
            if candidate.is_dir() {
                return Ok(candidate);
            }
        }

        // Fallback: look in the nfw cache directory
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| "could not determine cache directory".to_string())?
            .join("nfw")
            .join("templates")
            .join(template_id);

        if cache_dir.is_dir() {
            return Ok(cache_dir);
        }

        Err(format!(
            "template '{}' not found locally or in cache",
            template_id
        ))
    }
}
