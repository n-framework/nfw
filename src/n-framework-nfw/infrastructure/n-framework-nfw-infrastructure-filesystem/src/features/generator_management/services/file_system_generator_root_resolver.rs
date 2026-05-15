use std::path::{Path, PathBuf};
use serde_yaml::Value as YamlValue;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;

#[derive(Debug, Clone, Default)]
pub struct FileSystemGeneratorRootResolver;

impl FileSystemGeneratorRootResolver {
    pub fn new() -> Self {
        Self
    }
}

impl GeneratorRootResolver for FileSystemGeneratorRootResolver {
    fn resolve(
        &self,
        nfw_yaml: &YamlValue,
        generator_id: &str,
        workspace_root: &Path,
    ) -> Result<PathBuf, String> {
        // 1. Check local candidates configured in nfw.yaml
        if let Some(sources) = nfw_yaml.get("generator_sources")
            && let Some(local) = sources.get("local")
            && let Some(local_path) = local.as_str()
        {
            let local_root = workspace_root.join(local_path);

            // Try exact match
            let candidate = local_root.join(generator_id);
            if candidate.is_dir() {
                return Ok(candidate);
            }

            // Try stripping source namespace (e.g. official/dotnet-service -> dotnet-service)
            // and also checking src/ subfolder (common in official repo structure)
            if let Some((_ns, rest)) = generator_id.split_once('/') {
                let candidate = local_root.join(rest);
                if candidate.is_dir() {
                    return Ok(candidate);
                }

                let candidate = local_root.join("src").join(rest);
                if candidate.is_dir() {
                    return Ok(candidate);
                }
            }
        }

        // 2. Fallback: check ~/.nfw/generators (the primary cache/config dir)
        let nfw_home = dirs::home_dir()
            .map(|p| p.join(".nfw"))
            .ok_or_else(|| "could not determine .nfw directory".to_string())?;

        let generators_root = nfw_home.join("generators");

        // Try exact match in generators_root
        let candidate = generators_root.join(generator_id);
        if candidate.is_dir() {
            return Ok(candidate);
        }

        // Try standard source/src structure: e.g. official/dotnet-service -> official/src/dotnet-service
        if let Some((ns, rest)) = generator_id.split_once('/') {
            let candidate = generators_root.join(ns).join("src").join(rest);
            if candidate.is_dir() {
                return Ok(candidate);
            }

            let candidate = generators_root.join(ns).join(rest);
            if candidate.is_dir() {
                return Ok(candidate);
            }
        }

        // 3. Last fallback: look in the legacy nfw cache directory (~/.cache/nfw/generators)
        if let Some(cache) = dirs::cache_dir() {
            let cache_root = cache.join("nfw").join("generators");
            let candidate = cache_root.join(generator_id);
            if candidate.is_dir() {
                return Ok(candidate);
            }
        }

        Err(format!(
            "generator '{}' not found locally or in cache",
            generator_id
        ))
    }
}
