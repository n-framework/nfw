use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use n_framework_nfw_core_application::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_domain::features::template_management::template_config::{
    InjectionTarget, TemplateConfig, TemplateStep,
};

use n_framework_core_template_abstractions::{FileGenerator, TemplateContext, TemplateRenderer};
use n_framework_core_template_tera::{TeraFileGenerator, TeraTemplateRenderer};

#[derive(Debug, Clone)]
pub struct FileSystemTemplateEngine {
    generator: TeraFileGenerator<TeraTemplateRenderer>,
    renderer: TeraTemplateRenderer,
}

impl FileSystemTemplateEngine {
    pub fn new() -> Self {
        Self {
            generator: TeraFileGenerator::default(),
            renderer: TeraTemplateRenderer::default(),
        }
    }

    fn to_core_context(&self, placeholders: &BTreeMap<String, String>) -> TemplateContext {
        let mut context = TemplateContext::empty();
        for (key, value) in placeholders {
            context.insert(key, value);
        }
        context
    }

    fn map_error(&self, error: n_framework_core_template_abstractions::TemplateError) -> TemplateError {
        TemplateError::RenderError(error.message())
    }
}

impl TemplateEngine for FileSystemTemplateEngine {
    fn execute(
        &self,
        config: &TemplateConfig,
        template_root: &Path,
        output_root: &Path,
        placeholders: &BTreeMap<String, String>,
    ) -> Result<(), TemplateError> {
        let core_context = self.to_core_context(placeholders);

        for step in &config.steps {
            match step {
                TemplateStep::Render { source, destination } => {
                    let source_path = template_root.join(source);
                    let rendered_dest = self.renderer
                        .render_content(destination, &core_context)
                        .map_err(|e| self.map_error(e))?;
                    let dest_path = output_root.join(rendered_dest);

                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent).map_err(|e| TemplateError::FileSystemError(e.to_string()))?;
                    }

                    let content = fs::read_to_string(&source_path)
                        .map_err(|e| TemplateError::FileSystemError(e.to_string()))?;
                    let rendered_content = self.renderer
                        .render_content(&content, &core_context)
                        .map_err(|e| self.map_error(e))?;

                    fs::write(&dest_path, rendered_content)
                        .map_err(|e| TemplateError::FileSystemError(e.to_string()))?;
                }
                TemplateStep::RenderFolder { source, destination } => {
                    let source_path = template_root.join(source);
                    let rendered_dest = self.renderer
                        .render_content(destination, &core_context)
                        .map_err(|e| self.map_error(e))?;
                    let dest_path = output_root.join(rendered_dest);

                    self.generator
                        .generate(&source_path, &dest_path, &core_context)
                        .map_err(|e| self.map_error(e))?;
                }
                TemplateStep::Inject { source, destination, injection_target } => {
                    let source_path = template_root.join(source);
                    let rendered_dest = self.renderer
                        .render_content(destination, &core_context)
                        .map_err(|e| self.map_error(e))?;
                    let dest_path = output_root.join(rendered_dest);

                    let inject_content_raw = fs::read_to_string(&source_path)
                        .map_err(|e| TemplateError::FileSystemError(e.to_string()))?;
                    let rendered_inject_content = self.renderer
                        .render_content(&inject_content_raw, &core_context)
                        .map_err(|e| self.map_error(e))?;

                    let mut file_content = fs::read_to_string(&dest_path)
                        .map_err(|e| TemplateError::FileSystemError(e.to_string()))?;

                    match injection_target {
                        InjectionTarget::AtEnd => {
                            if !file_content.ends_with('\n') && !file_content.is_empty() {
                                file_content.push('\n');
                            }
                            file_content.push_str(&rendered_inject_content);
                        }
                        InjectionTarget::Region(name) => {
                            let start_marker = format!("// region: {}", name);
                            let end_marker = format!("// endregion: {}", name);

                            if let Some(start_pos) = file_content.find(&start_marker) {
                                if let Some(end_pos) = file_content[start_pos..].find(&end_marker) {
                                    let absolute_end_pos = start_pos + end_pos;
                                    file_content.insert_str(absolute_end_pos, &rendered_inject_content);
                                } else {
                                    return Err(TemplateError::InjectionError(format!(
                                        "region end marker not found for: {}",
                                        name
                                    )));
                                }
                            } else {
                                return Err(TemplateError::InjectionError(format!(
                                    "region start marker not found for: {}",
                                    name
                                )));
                            }
                        }
                        _ => {
                            // Fallback for unsupported targets
                            file_content.push_str(&rendered_inject_content);
                        }
                    }

                    fs::write(&dest_path, file_content)
                        .map_err(|e| TemplateError::FileSystemError(e.to_string()))?;
                }
            }
        }

        Ok(())
    }
}
