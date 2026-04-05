use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

use nframework_nfw_application::features::service_management::models::errors::add_service_error::AddServiceError;
use nframework_nfw_application::features::service_management::models::service_generation_plan::ServiceGenerationPlan;
use nframework_nfw_application::features::service_management::services::abstractions::service_template_renderer::ServiceTemplateRenderer;

use crate::features::service_management::services::service_generation_cleanup::ServiceGenerationCleanup;

#[derive(Debug, Clone)]
pub struct FileSystemServiceTemplateRenderer {
    cleanup: ServiceGenerationCleanup,
}

struct RenderContext<'a> {
    content_root: &'a Path,
    output_root: &'a Path,
    placeholder_values: &'a BTreeMap<String, String>,
}

impl FileSystemServiceTemplateRenderer {
    pub fn new(cleanup: ServiceGenerationCleanup) -> Self {
        Self { cleanup }
    }

    fn render_template_content(
        &self,
        current_path: &Path,
        context: &RenderContext<'_>,
    ) -> Result<(), AddServiceError> {
        for entry in fs::read_dir(current_path).map_err(|error| {
            AddServiceError::RenderFailed(format!(
                "failed to read template content '{}': {error}",
                current_path.display()
            ))
        })? {
            let entry = entry.map_err(|error| {
                AddServiceError::RenderFailed(format!(
                    "failed to read an entry under '{}': {error}",
                    current_path.display()
                ))
            })?;

            let source_path = entry.path();
            let relative_path = source_path
                .strip_prefix(context.content_root)
                .map_err(|error| {
                    AddServiceError::RenderFailed(format!(
                        "failed to compute template-relative path for '{}': {error}",
                        source_path.display()
                    ))
                })?;

            let rendered_relative_path = render_path(relative_path, context.placeholder_values);
            ensure_safe_relative_path(&rendered_relative_path)?;
            let destination_path = context.output_root.join(rendered_relative_path);

            if source_path.is_dir() {
                fs::create_dir_all(&destination_path).map_err(|error| {
                    AddServiceError::RenderFailed(format!(
                        "failed to create directory '{}': {error}",
                        destination_path.display()
                    ))
                })?;

                self.render_template_content(&source_path, context)?;
                continue;
            }

            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent).map_err(|error| {
                    AddServiceError::RenderFailed(format!(
                        "failed to create parent directory '{}': {error}",
                        parent.display()
                    ))
                })?;
            }

            let bytes = fs::read(&source_path).map_err(|error| {
                AddServiceError::RenderFailed(format!(
                    "failed to read template file '{}': {error}",
                    source_path.display()
                ))
            })?;
            let rendered_bytes = render_bytes(&bytes, context.placeholder_values);
            fs::write(&destination_path, rendered_bytes).map_err(|error| {
                AddServiceError::RenderFailed(format!(
                    "failed to write generated file '{}': {error}",
                    destination_path.display()
                ))
            })?;
        }

        Ok(())
    }
}

impl Default for FileSystemServiceTemplateRenderer {
    fn default() -> Self {
        Self::new(ServiceGenerationCleanup::new())
    }
}

impl ServiceTemplateRenderer for FileSystemServiceTemplateRenderer {
    fn render_service(&self, plan: &ServiceGenerationPlan) -> Result<(), AddServiceError> {
        let content_root = plan.template_cache_path.join("content");
        if !content_root.is_dir() {
            return Err(AddServiceError::RenderFailed(format!(
                "template '{}' is missing required 'content/' directory",
                plan.template_cache_path.display()
            )));
        }

        fs::create_dir_all(&plan.output_root).map_err(|error| {
            AddServiceError::RenderFailed(format!(
                "failed to create output directory '{}': {error}",
                plan.output_root.display()
            ))
        })?;

        let context = RenderContext {
            content_root: &content_root,
            output_root: &plan.output_root,
            placeholder_values: &plan.placeholder_values,
        };

        match self.render_template_content(&content_root, &context) {
            Ok(()) => Ok(()),
            Err(error) => self
                .cleanup
                .cleanup_output(&plan.output_root)
                .map_err(|cleanup_error| merge_cleanup_error(cleanup_error, &error))
                .and(Err(error)),
        }
    }

    fn cleanup_partial_output(&self, output_root: &Path) -> Result<(), AddServiceError> {
        self.cleanup
            .cleanup_output(output_root)
            .map_err(AddServiceError::CleanupFailed)
    }
}

fn render_path(relative_path: &Path, placeholders: &BTreeMap<String, String>) -> PathBuf {
    let mut rendered_path = PathBuf::new();

    for component in relative_path.components() {
        let text = component.as_os_str().to_string_lossy();
        rendered_path.push(render_text(&text, placeholders));
    }

    rendered_path
}

fn render_bytes(bytes: &[u8], placeholders: &BTreeMap<String, String>) -> Vec<u8> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => render_text(&text, placeholders).into_bytes(),
        Err(_) => bytes.to_vec(),
    }
}

fn render_text(value: &str, placeholders: &BTreeMap<String, String>) -> String {
    let mut rendered = value.to_owned();
    for (placeholder, replacement) in placeholders {
        rendered = rendered.replace(placeholder, replacement);
    }
    rendered
}

fn ensure_safe_relative_path(relative_path: &Path) -> Result<(), AddServiceError> {
    if relative_path.is_absolute() {
        return Err(AddServiceError::RenderFailed(format!(
            "unsafe rendered path '{}': absolute paths are not allowed",
            relative_path.display()
        )));
    }

    for component in relative_path.components() {
        match component {
            Component::Prefix(_)
            | Component::RootDir
            | Component::ParentDir
            | Component::CurDir => {
                return Err(AddServiceError::RenderFailed(format!(
                    "unsafe rendered path '{}': traversal or root components are not allowed",
                    relative_path.display()
                )));
            }
            Component::Normal(_) => {}
        }
    }

    Ok(())
}

fn merge_cleanup_error(cleanup_error: String, original_error: &AddServiceError) -> AddServiceError {
    AddServiceError::CleanupFailed(format!(
        "{cleanup_error}; original error: {original_error}"
    ))
}
