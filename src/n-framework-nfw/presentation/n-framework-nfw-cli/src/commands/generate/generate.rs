use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use serde_yaml::Value as YamlValue;

const WORKSPACE_METADATA_FILE: &str = "nfw.yaml";

#[derive(Debug, Clone)]
pub struct GenerateCliCommand<E> {
    engine: E,
}

impl<E: TemplateEngine> GenerateCliCommand<E> {
    pub fn new(engine: E) -> Self {
        Self { engine }
    }

    pub fn execute(
        &self,
        generator_type: &str,
        name: &str,
        feature: Option<&str>,
        params: Option<&str>,
    ) -> Result<(), String> {
        let current_dir =
            std::env::current_dir().map_err(|e| format!("failed to get current directory: {e}"))?;
        let workspace_root = resolve_workspace_root(&current_dir)?;
        let nfw_yaml_path = workspace_root.join(WORKSPACE_METADATA_FILE);

        let nfw_yaml_content = fs::read_to_string(&nfw_yaml_path)
            .map_err(|e| format!("failed to read nfw.yaml: {e}"))?;
        let nfw_yaml: YamlValue =
            serde_yaml::from_str(&nfw_yaml_content).map_err(|e| format!("invalid nfw.yaml: {e}"))?;

        let template_id = nfw_yaml
            .get("generators")
            .and_then(|g| g.get(generator_type))
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                format!(
                    "generator type '{}' not found in nfw.yaml. Add generators.{}: <template-id> to your nfw.yaml.",
                    generator_type, generator_type
                )
            })?;

        let template_root = resolve_template_root(&nfw_yaml, template_id, &workspace_root)?;

        let template_config_path = template_root.join("template.yaml");
        let template_config_content = fs::read_to_string(&template_config_path).map_err(|e| {
            format!(
                "failed to read template.yaml at {}: {e}",
                template_config_path.display()
            )
        })?;
        let config: TemplateConfig = serde_yaml::from_str(&template_config_content)
            .map_err(|e| format!("failed to parse template.yaml: {e}"))?;

        let feature_name = feature.unwrap_or(name);
        let mut placeholders = BTreeMap::new();
        placeholders.insert("Name".to_string(), name.to_string());
        placeholders.insert("Feature".to_string(), feature_name.to_string());

        if let Some(params_str) = params {
            for param_pair in params_str.split(',') {
                if let Some((key, value)) = param_pair.split_once('=') {
                    placeholders.insert(key.trim().to_string(), value.trim().to_string());
                } else {
                    return Err(format!(
                        "invalid parameter format '{}'. expected Key=Value",
                        param_pair
                    ));
                }
            }
        }

        self.engine
            .execute(&config, &template_root, &workspace_root, &placeholders)
            .map_err(|e| format!("{e}"))?;

        println!("Generated '{generator_type}' '{name}' successfully.");
        Ok(())
    }
}

fn resolve_workspace_root(start_directory: &Path) -> Result<PathBuf, String> {
    let mut candidate = start_directory.to_path_buf();
    loop {
        let workspace_file = candidate.join(WORKSPACE_METADATA_FILE);
        if workspace_file.is_file() {
            return Ok(candidate);
        }
        let Some(parent) = candidate.parent() else {
            break;
        };
        candidate = parent.to_path_buf();
    }
    Err("could not find nfw.yaml in current directory or parent directories".to_string())
}

fn resolve_template_root(
    nfw_yaml: &YamlValue,
    template_id: &str,
    workspace_root: &Path,
) -> Result<PathBuf, String> {
    // Check if there is a template_sources.local entry mapping to a local directory
    if let Some(sources) = nfw_yaml.get("template_sources") {
        if let Some(local) = sources.get("local") {
            if let Some(local_path) = local.as_str() {
                let candidate = workspace_root.join(local_path).join(template_id);
                if candidate.is_dir() {
                    return Ok(candidate);
                }
            }
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
