use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use n_framework_nfw_core_application::features::generator_management::models::generator_error::{
    CommandExecutionContext, GeneratorError,
};
use n_framework_nfw_core_application::features::generator_management::services::generator_engine::GeneratorEngine;
use n_framework_nfw_core_domain::features::generator_management::generator_config::{
    GeneratorConfig, GeneratorStepAction, InjectionTarget,
};
use n_framework_nfw_core_domain::features::generator_management::generator_parameters::GeneratorParameters;

use n_framework_core_template_abstractions::{FileGenerator, TemplateContext, TemplateRenderer};
use n_framework_core_template_tera::{TeraFileGenerator, TeraTemplateRenderer};

#[derive(Debug, Clone)]
pub struct FileSystemGeneratorEngine {
    generator: TeraFileGenerator<TeraTemplateRenderer>,
    renderer: TeraTemplateRenderer,
}

impl Default for FileSystemGeneratorEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystemGeneratorEngine {
    /// Creates a new instance of the file system generator engine.
    ///
    /// This initializes the underlying generator and renderer required
    /// to process generators, render contents securely, and prevent path traversal.
    pub fn new() -> Self {
        Self {
            generator: TeraFileGenerator::default(),
            renderer: TeraTemplateRenderer::new(),
        }
    }

    fn to_core_context(&self, parameters: &GeneratorParameters) -> TemplateContext {
        let mut context = TemplateContext::empty();
        for (key, value) in parameters.as_map() {
            context.insert_value(key, value.clone());
        }
        context
    }

    fn map_error(
        error: n_framework_core_template_abstractions::TemplateError,
        step_index: usize,
        generator_id: Option<String>,
        file_path: Option<String>,
    ) -> GeneratorError {
        GeneratorError::GeneratorRenderError {
            message: error.message(),
            step_index: Some(step_index),
            generator_id,
            file_path,
            source: Some(Box::new(error)),
        }
    }

    pub(crate) fn validate_rendered_command(
        command: &str,
        step_index: usize,
        generator_id: Option<String>,
    ) -> Result<(), GeneratorError> {
        let dangerous_patterns = [";", "&&", "||", "|", "`", "$(", "$( "];
        for pattern in dangerous_patterns {
            if command.contains(pattern) {
                return Err(GeneratorError::CommandExecutionError(Box::new(
                    CommandExecutionContext {
                        message: format!(
                            "security validation failed: command contains dangerous pattern '{}'",
                            pattern
                        ),
                        command: command.to_string(),
                        stdout: None,
                        working_directory: None,
                        exit_code: None,
                        step_index,
                        generator_id,
                    },
                )));
            }
        }
        Ok(())
    }

    fn ensure_safe_path(
        &self,
        dest_path: &Path,
        output_root: &Path,
        generator_id: Option<String>,
    ) -> Result<(), GeneratorError> {
        // Validation Strategy:
        // We prevent path traversal and unauthorized file access via:
        // 1. Component Analysis: We block '..' (ParentDir) to prevent traversing outside the root.
        // 2. Absolute Path Check: We ensure absolute paths start with the output root.
        // 3. Symlink Note: While we don't currently block symlinks at the resolution level,
        //    individual file operations use standard I/O which will follow existing symlinks
        //    if they exist in the target structure.
        // This ensures the generator cannot overwrite arbitrary system files even if placeholders
        // contain malicious sequences.
        if dest_path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(GeneratorError::GeneratorRenderError {
                message: format!(
                    "destination path '{}' contains parent directory traversal",
                    dest_path.display()
                ),
                step_index: None,
                generator_id,
                file_path: None,
                source: None,
            });
        }

        if dest_path.is_absolute() && !dest_path.starts_with(output_root) {
            return Err(GeneratorError::GeneratorRenderError {
                message: format!(
                    "destination path '{}' is an absolute path escaping output root",
                    dest_path.display()
                ),
                step_index: None,
                generator_id,
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
    ) -> Result<(PathBuf, PathBuf), GeneratorError> {
        let source_path = ctx.generator_root.join(source);
        let rendered_dest = self
            .renderer
            .render_content(destination, ctx.core_context)
            .map_err(|e| Self::map_error(e, step_index, ctx.generator_id.clone(), None))?;

        let dest_path = if rendered_dest.is_empty() || rendered_dest == "." {
            ctx.output_root.to_path_buf()
        } else {
            ctx.output_root.join(rendered_dest)
        };

        self.ensure_safe_path(&dest_path, ctx.output_root, ctx.generator_id.clone())?;

        Ok((source_path, dest_path))
    }
}

struct ExecutionContext<'a> {
    generator_root: &'a Path,
    output_root: &'a Path,
    core_context: &'a TemplateContext,
    generator_id: Option<String>,
}

impl GeneratorEngine for FileSystemGeneratorEngine {
    fn execute(
        &self,
        config: &GeneratorConfig,
        generator_root: &Path,
        output_root: &Path,
        parameters: &GeneratorParameters,
    ) -> Result<(), GeneratorError> {
        let core_context = self.to_core_context(parameters);
        let generator_id = config.id().map(String::from);
        let ctx = ExecutionContext {
            generator_root,
            output_root,
            core_context: &core_context,
            generator_id: generator_id.clone(),
        };

        for (i, step) in config.steps().iter().enumerate() {
            // Evaluate condition first if provided
            if let Some(condition_expr) = &step.condition {
                let eval_result = self
                    .renderer
                    .render_content(condition_expr, ctx.core_context)
                    .map_err(|e| GeneratorError::GeneratorRenderError {
                        message: format!(
                            "failed to evaluate condition '{}': {}",
                            condition_expr, e
                        ),
                        step_index: Some(i),
                        generator_id: ctx.generator_id.clone(),
                        file_path: None,
                        source: None,
                    })?;

                let is_true = match eval_result.trim().to_lowercase().as_str() {
                    "true" | "1" => true,
                    "false" | "0" | "" => false,
                    _ => {
                        return Err(GeneratorError::GeneratorRenderError {
                            message: format!(
                                "Invalid boolean evaluation for condition expression '{}'. Evaluated to '{}'",
                                condition_expr, eval_result
                            ),
                            step_index: Some(i),
                            generator_id: ctx.generator_id.clone(),
                            file_path: None,
                            source: None,
                        });
                    }
                };

                if !is_true {
                    tracing::debug!(
                        "Skipping step [action: {:?}] due to false condition: {}",
                        step.action,
                        condition_expr
                    );
                    continue;
                }
            }

            match &step.action {
                GeneratorStepAction::Render {
                    source,
                    destination,
                } => {
                    let (source_path, dest_path) =
                        self.resolve_paths(source, destination, &ctx, i)?;

                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent).map_err(|e| {
                            GeneratorError::io(
                                format!(
                                    "failed to create parent directory for {}: {e}",
                                    dest_path.display()
                                ),
                                parent,
                            )
                        })?;
                    }

                    let content = fs::read_to_string(&source_path).map_err(|e| {
                        GeneratorError::io(
                            format!("failed to read generator source: {e}"),
                            source_path,
                        )
                    })?;
                    let rendered_content = self
                        .renderer
                        .render_content(&content, ctx.core_context)
                        .map_err(|e| {
                            Self::map_error(
                                e,
                                i,
                                generator_id.clone(),
                                Some(dest_path.display().to_string()),
                            )
                        })?;

                    fs::write(&dest_path, rendered_content).map_err(|e| {
                        GeneratorError::io(
                            format!("failed to write generated file: {e}"),
                            dest_path,
                        )
                    })?;
                }
                GeneratorStepAction::RenderIfAbsent {
                    source,
                    destination,
                } => {
                    let (source_path, dest_path) =
                        self.resolve_paths(source, destination, &ctx, i)?;

                    // Skip if the destination already exists — this step is idempotent.
                    if dest_path.exists() {
                        continue;
                    }

                    if let Some(parent) = dest_path.parent() {
                        fs::create_dir_all(parent).map_err(|e| {
                            GeneratorError::io(
                                format!(
                                    "failed to create parent directory for {}: {e}",
                                    dest_path.display()
                                ),
                                parent,
                            )
                        })?;
                    }

                    let content = fs::read_to_string(&source_path).map_err(|e| {
                        GeneratorError::io(
                            format!("failed to read generator source: {e}"),
                            source_path,
                        )
                    })?;
                    let rendered_content = self
                        .renderer
                        .render_content(&content, ctx.core_context)
                        .map_err(|e| {
                            Self::map_error(
                                e,
                                i,
                                generator_id.clone(),
                                Some(dest_path.display().to_string()),
                            )
                        })?;

                    fs::write(&dest_path, rendered_content).map_err(|e| {
                        GeneratorError::io(
                            format!("failed to write generated file: {e}"),
                            dest_path,
                        )
                    })?;
                }
                GeneratorStepAction::RenderFolder {
                    source,
                    destination,
                } => {
                    let (source_path, dest_path) =
                        self.resolve_paths(source, destination, &ctx, i)?;

                    self.generator
                        .generate(&source_path, &dest_path, ctx.core_context)
                        .map_err(|e| {
                            Self::map_error(
                                e,
                                i,
                                generator_id.clone(),
                                Some(dest_path.display().to_string()),
                            )
                        })?;
                }
                GeneratorStepAction::Inject {
                    source,
                    destination,
                    injection_target,
                } => {
                    let (source_path, dest_path) =
                        self.resolve_paths(source, destination, &ctx, i)?;

                    let inject_content_raw = fs::read_to_string(&source_path).map_err(|e| {
                        GeneratorError::io(
                            format!("failed to read injection source: {e}"),
                            source_path,
                        )
                    })?;
                    let rendered_inject_content = self
                        .renderer
                        .render_content(&inject_content_raw, ctx.core_context)
                        .map_err(|e| {
                            Self::map_error(
                                e,
                                i,
                                generator_id.clone(),
                                Some(dest_path.display().to_string()),
                            )
                        })?;

                    let mut file_content = fs::read_to_string(&dest_path).map_err(|e| {
                        GeneratorError::io(
                            format!("failed to read target file for injection: {e}"),
                            &dest_path,
                        )
                    })?;

                    let (start_marker, end_marker, region_name) = match injection_target {
                        InjectionTarget::AtEnd => {
                            if !file_content.ends_with('\n') && !file_content.is_empty() {
                                file_content.push('\n');
                            }
                            file_content.push_str(&rendered_inject_content);
                            fs::write(&dest_path, file_content).map_err(|e| {
                                GeneratorError::io(
                                    format!("failed to write injected file: {e}"),
                                    dest_path,
                                )
                            })?;
                            continue;
                        }
                        InjectionTarget::Region(value) => {
                            let start_marker = format!("<nfw:{}:start>", value);
                            let end_marker = format!("<nfw:{}:end>", value);
                            (start_marker, end_marker, Some(value.clone()))
                        }
                    };

                    if let Some(start_pos) = file_content.find(&start_marker) {
                        if let Some(relative_end_pos) = file_content[start_pos..].find(&end_marker)
                        {
                            let absolute_end_pos = start_pos + relative_end_pos;
                            let insert_pos = file_content[..absolute_end_pos]
                                .rfind('\n')
                                .map(|pos| pos + 1)
                                .unwrap_or(absolute_end_pos);

                            let marker_line_start = file_content[..start_pos]
                                .rfind('\n')
                                .map(|pos| pos + 1)
                                .unwrap_or(0);
                            let indent: String = file_content[marker_line_start..]
                                .chars()
                                .take_while(|c| c.is_whitespace() && *c != '\n')
                                .collect();

                            let indented_content = indent_lines(&rendered_inject_content, &indent);

                            // Skip insert if the trimmed content already exists in the region.
                            let region_slice = &file_content[start_pos..absolute_end_pos];
                            let trimmed_new = rendered_inject_content.trim();
                            if !trimmed_new.is_empty()
                                && region_slice.lines().any(|l| l.trim() == trimmed_new)
                            {
                                // Content already present — no-op.
                            } else {
                                file_content.insert_str(insert_pos, &indented_content);
                            }
                        } else {
                            let snippet = get_snippet(&file_content, start_pos);
                            return Err(GeneratorError::GeneratorInjectionError {
                                message: format!(
                                    "region end marker '{}' not found in '{}' after start marker.\nContext around start marker:\n{}",
                                    end_marker,
                                    dest_path.display(),
                                    snippet
                                ),
                                file_path: Some(dest_path.display().to_string()),
                                region: region_name,
                                generator_id: generator_id.clone(),
                            });
                        }
                    } else {
                        return Err(GeneratorError::GeneratorInjectionError {
                            message: format!(
                                "region start marker '{}' not found in '{}'. Ensure the target file has the required injection marker.",
                                start_marker,
                                dest_path.display()
                            ),
                            file_path: Some(dest_path.display().to_string()),
                            region: region_name,
                            generator_id: generator_id.clone(),
                        });
                    }

                    fs::write(&dest_path, file_content).map_err(|e| {
                        GeneratorError::io(format!("failed to write injected file: {e}"), dest_path)
                    })?;
                }
                GeneratorStepAction::RunCommand {
                    command,
                    working_directory,
                } => {
                    let rendered_command = self
                        .renderer
                        .render_content(command, ctx.core_context)
                        .map_err(|e| Self::map_error(e, i, generator_id.clone(), None))?;

                    Self::validate_rendered_command(&rendered_command, i, generator_id.clone())?;

                    let work_dir = if let Some(wd) = working_directory {
                        let rendered_wd = self
                            .renderer
                            .render_content(wd, ctx.core_context)
                            .map_err(|e| Self::map_error(e, i, generator_id.clone(), None))?;
                        ctx.output_root.join(rendered_wd)
                    } else {
                        ctx.output_root.to_path_buf()
                    };

                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(&rendered_command)
                        .current_dir(&work_dir)
                        .output()
                        .map_err(|e| {
                            GeneratorError::CommandExecutionError(Box::new(
                                CommandExecutionContext {
                                    message: format!("failed to spawn command: {e}"),
                                    command: rendered_command.clone(),
                                    stdout: None,
                                    working_directory: Some(work_dir.clone()),
                                    exit_code: None,
                                    step_index: i,
                                    generator_id: generator_id.clone(),
                                },
                            ))
                        })?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        return Err(GeneratorError::CommandExecutionError(Box::new(
                            CommandExecutionContext {
                                message: format!(
                                    "command failed with exit code {}: {stderr}",
                                    output.status.code().unwrap_or(-1)
                                ),
                                command: rendered_command,
                                stdout: Some(stdout.to_string()),
                                working_directory: Some(work_dir),
                                exit_code: output.status.code(),
                                step_index: i,
                                generator_id: generator_id.clone(),
                            },
                        )));
                    }
                }
            }
        }

        Ok(())
    }
}

fn get_snippet(content: &str, pos: usize) -> String {
    let start = pos.saturating_sub(100);
    let end = (pos + 100).min(content.len());
    let mut snippet = content[start..end].to_string();
    if start > 0 {
        snippet.insert_str(0, "...");
    }
    if end < content.len() {
        snippet.push_str("...");
    }
    snippet
}

fn indent_lines(content: &str, indent: &str) -> String {
    if indent.is_empty() {
        return content.to_string();
    }
    content
        .lines()
        .map(|line| {
            if line.trim().is_empty() {
                line.to_string()
            } else {
                format!("{indent}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + if content.ends_with('\n') { "\n" } else { "" }
}

#[cfg(test)]
#[path = "generator_engine.tests.rs"]
mod tests;
