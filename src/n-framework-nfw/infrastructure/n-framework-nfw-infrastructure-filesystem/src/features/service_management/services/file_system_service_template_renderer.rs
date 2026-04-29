use std::fs;
use std::path::Path;
use tracing;
use n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;
use n_framework_nfw_core_application::features::service_management::models::service_generation_plan::ServiceGenerationPlan;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_template_renderer::ServiceTemplateRenderer;

use crate::features::service_management::services::service_generation_cleanup::ServiceGenerationCleanup;

use crate::features::template_management::template_engine::FileSystemTemplateEngine;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;

#[derive(Debug, Clone)]
pub struct FileSystemServiceTemplateRenderer {
    cleanup: ServiceGenerationCleanup,
    engine: FileSystemTemplateEngine,
}

impl FileSystemServiceTemplateRenderer {
    pub fn new(cleanup: ServiceGenerationCleanup) -> Self {
        Self {
            cleanup,
            engine: FileSystemTemplateEngine::new(),
        }
    }
}

impl Default for FileSystemServiceTemplateRenderer {
    fn default() -> Self {
        Self::new(ServiceGenerationCleanup::new())
    }
}

impl ServiceTemplateRenderer for FileSystemServiceTemplateRenderer {
    fn render_service(&self, plan: &ServiceGenerationPlan) -> Result<(), AddServiceError> {
        let base_config_path = plan.template_cache_path.join("template.yaml");
        let mut service_folder = String::from("service");

        // Dynamic Sub-template Resolution Algorithm:
        // 1. Check if the root template cache contains a base `template.yaml`.
        // 2. If it exists, parse it and look up the `generators.service` key to find the exact sub-folder for the service template.
        // 3. Fall back to the default `service` folder if not specified or if parsing the base template fails.
        if base_config_path.exists() {
            match fs::read_to_string(&base_config_path) {
                Ok(yaml) => match serde_yaml::from_str::<TemplateConfig>(&yaml) {
                    Ok(base_config) => {
                        if let Some(generators) = base_config.generators()
                            && let Some(folder) = generators.get("service")
                        {
                            service_folder = folder.clone();
                        }
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Failed to parse base template.yaml at {}: {}; falling back to default 'service' folder",
                            base_config_path.display(),
                            e
                        );
                    }
                },
                Err(e) => {
                    tracing::warn!(
                        "Failed to read base template.yaml at {}: {}; falling back to default 'service' folder",
                        base_config_path.display(),
                        e
                    );
                }
            }
        }

        let service_root = plan.template_cache_path.join(&service_folder);
        let service_config_path = service_root.join("template.yaml");

        if !service_config_path.exists() {
            return Err(AddServiceError::RenderFailed(format!(
                "service/template.yaml not found at: {}",
                service_config_path.display()
            )));
        }

        let yaml = fs::read_to_string(&service_config_path).map_err(|e| {
            AddServiceError::RenderFailed(format!("failed to read service/template.yaml: {e}"))
        })?;

        let mut config = serde_yaml::from_str::<TemplateConfig>(&yaml).map_err(|e| {
            AddServiceError::RenderFailed(format!("failed to parse service/template.yaml: {e}"))
        })?;

        config
            .validate()
            .map_err(|e| AddServiceError::RenderFailed(e.to_string()))?;

        if config.id().is_none() {
            let _ = config.set_id(plan.template_id.clone());
        }

        if let Err(engine_error) = self.engine.execute(
            &config,
            &service_root,
            &plan.output_root,
            &plan.placeholder_values,
        ) {
            let error_msg = engine_error.to_string();
            let render_error = AddServiceError::RenderFailed(error_msg.clone());
            if let Err(cleanup_error) = self.cleanup.cleanup_output(&plan.output_root) {
                return Err(AddServiceError::CleanupFailed(format!(
                    "{cleanup_error}; original error: {error_msg}"
                )));
            }
            return Err(render_error);
        }

        Ok(())
    }

    fn cleanup_partial_output(&self, output_root: &Path) -> Result<(), AddServiceError> {
        self.cleanup
            .cleanup_output(output_root)
            .map_err(AddServiceError::CleanupFailed)
    }
}
