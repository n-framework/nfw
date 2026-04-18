use std::fs;
use std::path::Path;
use n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;
use n_framework_nfw_core_application::features::service_management::models::service_generation_plan::ServiceGenerationPlan;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_template_renderer::ServiceTemplateRenderer;

use crate::features::service_management::services::service_generation_cleanup::ServiceGenerationCleanup;

use crate::features::template_management::template_engine::FileSystemTemplateEngine;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_domain::features::template_management::template_config::{
    TemplateConfig, TemplateStep,
};

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
        let template_root = &plan.template_cache_path;
        let root_config_path = template_root.join("template.yaml");
        let subfolder_config_path = template_root.join("add-service").join("template.yaml");

        // Determine which configuration and template root to use
        let (config, final_template_root) = if subfolder_config_path.exists() {
            // Implementation is in add-service/
            let yaml = fs::read_to_string(&subfolder_config_path).map_err(|e| {
                AddServiceError::RenderFailed(format!(
                    "failed to read subfolder template.yaml: {e}"
                ))
            })?;
            let mut config = serde_yaml::from_str::<TemplateConfig>(&yaml).map_err(|e| {
                AddServiceError::RenderFailed(format!(
                    "failed to parse subfolder template.yaml: {e}"
                ))
            })?;
            config
                .validate()
                .map_err(|e| AddServiceError::RenderFailed(e.to_string()))?;
            // Set ID if missing in subfolder config
            if config.id().is_none() {
                let _ = config.set_id(plan.template_id.clone());
            }
            (config, template_root.join("add-service"))
        } else if root_config_path.exists() {
            // Check if root config has steps
            let yaml = fs::read_to_string(&root_config_path).map_err(|e| {
                AddServiceError::RenderFailed(format!("failed to read root template.yaml: {e}"))
            })?;
            let config = serde_yaml::from_str::<TemplateConfig>(&yaml).map_err(|e| {
                AddServiceError::RenderFailed(format!("failed to parse root template.yaml: {e}"))
            })?;
            // We do not call config.validate() here because older templates
            // may legitimately have an empty root template.yaml and use content/.
            // Instead we check config.steps().is_empty() below.

            if !config.steps().is_empty() {
                config
                    .validate()
                    .map_err(|e| AddServiceError::RenderFailed(e.to_string()))?;
                (config, template_root.to_path_buf())
            } else {
                // Root config exists but has no steps, and no subfolder config found.
                // Fallback to legacy if content/ exists, otherwise error or empty
                let content_path = template_root.join("content");
                if content_path.exists() {
                    let config = TemplateConfig::new(
                        Some(plan.template_id.clone()),
                        vec![TemplateStep::RenderFolder {
                            source: "content".to_string(),
                            destination: "".to_string(),
                        }],
                    )
                    .map_err(|e| AddServiceError::RenderFailed(e.to_string()))?;
                    (config, template_root.to_path_buf())
                } else {
                    return Err(AddServiceError::RenderFailed("no template steps found in template.yaml or add-service/template.yaml, and no legacy content folder present".to_string()));
                }
            }
        } else {
            // Legacy fallback: Create a virtual config that renders the 'content/' folder
            let config = TemplateConfig::new(
                Some(plan.template_id.clone()),
                vec![TemplateStep::RenderFolder {
                    source: "content".to_string(),
                    destination: "".to_string(),
                }],
            )
            .map_err(|e| AddServiceError::RenderFailed(e.to_string()))?;
            (config, template_root.to_path_buf())
        };

        if let Err(engine_error) = self.engine.execute(
            &config,
            &final_template_root,
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
