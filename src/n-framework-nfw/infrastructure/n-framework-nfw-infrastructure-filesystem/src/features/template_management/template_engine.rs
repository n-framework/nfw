use std::fs;
use std::path::{Path, PathBuf};

use n_framework_nfw_core_application::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_domain::features::template_management::template_config::{
    InjectionTarget, TemplateConfig, TemplateStep,
};
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;

use n_framework_core_template_abstractions::{FileGenerator, TemplateContext, TemplateRenderer};
use n_framework_core_template_tera::{TeraFileGenerator, TeraTemplateRenderer};

#[derive(Debug, Clone)]
pub struct FileSystemTemplateEngine {
    generator: TeraFileGenerator<TeraTemplateRenderer>,
    renderer: TeraTemplateRenderer,
}

impl Default for FileSystemTemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystemTemplateEngine {
    pub fn new() -> Self {
        Self {
            generator: TeraFileGenerator::default(),
            renderer: TeraTemplateRenderer,
        }
    }

    fn to_core_context(&self, parameters: &TemplateParameters) -> TemplateContext {
        let mut context = TemplateContext::empty();
        for (key, value) in parameters.as_map() {
            context.insert(key, value);
        }
        context
    }

    fn map_error(
        error: n_framework_core_template_abstractions::TemplateError,
        step_index: usize,
        template_id: Option<String>,
        file_path: Option<String>,
    ) -> TemplateError {
        TemplateError::TemplateRenderError {
            message: error.message(),
            step_index: Some(step_index),
            template_id,
            file_path,
            source: Some(Box::new(error)),
        }
    }

    fn ensure_safe_path(
        &self,
        dest_path: &Path,
        output_root: &Path,
        template_id: Option<String>,
    ) -> Result<(), TemplateError> {
        // Validation Strategy:
        // We prevent path traversal and unauthorized file access via:
        // 1. Component Analysis: We block '..' (ParentDir) to prevent traversing outside the root.
        // 2. Absolute Path Check: We ensure absolute paths start with the output root.
        // 3. Symlink Note: While we don't currently block symlinks at the resolution level,
        //    individual file operations use standard I/O which will follow existing symlinks
        //    if they exist in the target structure.
        // This ensures the template cannot overwrite arbitrary system files even if placeholders
        // contain malicious sequences.
        if dest_path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(TemplateError::TemplateRenderError {
                message: format!(
                    "destination path '{}' contains parent directory traversal",
                    dest_path.display()
                ),
                step_index: None,
                template_id,
                file_path: None,
                source: None,
            });
        }

        if dest_path.is_absolute() && !dest_path.starts_with(output_root) {
            return Err(TemplateError::TemplateRenderError {
                message: format!(
                    "destination path '{}' is an absolute path escaping output root",
                    dest_path.display()
                ),
                step_index: None,
                template_id,
                file_path: None,
                source: None,
            });
        }

        Ok(())
    }

    fn resolve_paths(
        &self,
        source: &str,
        destination: &str,
        ctx: &ExecutionContext,
        step_index: usize,
    ) -> Result<(PathBuf, PathBuf), TemplateError> {
        let source_path = ctx.template_root.join(source);
        let rendered_dest = self
            .renderer
            .render_content(destination, ctx.core_context)
            .map_err(|e| Self::map_error(e, step_index, ctx.template_id.clone(), None))?;
        let dest_path = ctx.output_root.join(rendered_dest);

        self.ensure_safe_path(&dest_path, ctx.output_root, ctx.template_id.clone())?;

        Ok((source_path, dest_path))
    }
}

struct ExecutionContext<'a> {
    template_root: &'a Path,
    output_root: &'a Path,
    core_context: &'a TemplateContext,
    template_id: Option<String>,
}

impl TemplateEngine for FileSystemTemplateEngine {
    fn execute(
        &self,
        config: &TemplateConfig,
        template_root: &Path,
        output_root: &Path,
        parameters: &TemplateParameters,
    ) -> Result<(), TemplateError> {
        let core_context = self.to_core_context(parameters);
        let template_id = config.id().map(String::from);
        let ctx = ExecutionContext {
            template_root,
            output_root,
            core_context: &core_context,
            template_id: template_id.clone(),
        };

        for (i, step) in config.steps().iter().enumerate() {
            match step {
                TemplateStep::Render {
                    source,
                    destination,
                } => {
                    let (source_path, dest_path) =
                        self.resolve_paths(source, destination, &ctx, i)?;

                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent).map_err(|e| {
                            TemplateError::io(
                                format!("failed to create parent directory: {e}"),
                                parent,
                            )
                        })?;
                    }

                    let content = fs::read_to_string(&source_path).map_err(|e| {
                        TemplateError::io(
                            format!("failed to read template source: {e}"),
                            source_path,
                        )
                    })?;
                    let rendered_content = self
                        .renderer
                        .render_content(&content, &core_context)
                        .map_err(|e| {
                            Self::map_error(
                                e,
                                i,
                                template_id.clone(),
                                Some(dest_path.display().to_string()),
                            )
                        })?;

                    fs::write(&dest_path, rendered_content).map_err(|e| {
                        TemplateError::io(format!("failed to write generated file: {e}"), dest_path)
                    })?;
                }
                TemplateStep::RenderFolder {
                    source,
                    destination,
                } => {
                    let (source_path, dest_path) =
                        self.resolve_paths(source, destination, &ctx, i)?;

                    self.generator
                        .generate(&source_path, &dest_path, &core_context)
                        .map_err(|e| {
                            Self::map_error(
                                e,
                                i,
                                template_id.clone(),
                                Some(dest_path.display().to_string()),
                            )
                        })?;
                }
                TemplateStep::Inject {
                    source,
                    destination,
                    injection_target,
                } => {
                    let (source_path, dest_path) =
                        self.resolve_paths(source, destination, &ctx, i)?;

                    let inject_content_raw = fs::read_to_string(&source_path).map_err(|e| {
                        TemplateError::io(
                            format!("failed to read injection source: {e}"),
                            source_path,
                        )
                    })?;
                    let rendered_inject_content = self
                        .renderer
                        .render_content(&inject_content_raw, &core_context)
                        .map_err(|e| {
                            Self::map_error(
                                e,
                                i,
                                template_id.clone(),
                                Some(dest_path.display().to_string()),
                            )
                        })?;

                    let mut file_content = fs::read_to_string(&dest_path).map_err(|e| {
                        TemplateError::io(
                            format!("failed to read target file for injection: {e}"),
                            &dest_path,
                        )
                    })?;

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
                                    // Insertion happens BEFORE the end marker, keeping region intact.
                                    let absolute_end_pos = start_pos + end_pos;
                                    file_content
                                        .insert_str(absolute_end_pos, &rendered_inject_content);
                                } else {
                                    return Err(TemplateError::TemplateInjectionError {
                                        message: format!(
                                            "region end marker '{}' not found in '{}'. Regions must follow the format: // region: {} ... // endregion: {}",
                                            end_marker,
                                            dest_path.display(),
                                            name,
                                            name
                                        ),
                                        file_path: Some(dest_path.display().to_string()),
                                        region: Some(name.clone()),
                                        template_id: template_id.clone(),
                                    });
                                }
                            } else {
                                return Err(TemplateError::TemplateInjectionError {
                                    message: format!(
                                        "region start marker '{}' not found in '{}'. Ensure the target file has the required injection marker.",
                                        start_marker,
                                        dest_path.display()
                                    ),
                                    file_path: Some(dest_path.display().to_string()),
                                    region: Some(name.clone()),
                                    template_id: template_id.clone(),
                                });
                            }
                        }
                    }

                    fs::write(&dest_path, file_content).map_err(|e| {
                        TemplateError::io(format!("failed to write injected file: {e}"), dest_path)
                    })?;
                }
            }
        }

        Ok(())
    }
}
