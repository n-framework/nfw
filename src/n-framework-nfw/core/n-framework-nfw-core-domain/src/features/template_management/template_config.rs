use crate::features::template_management::errors::TemplateConfigError;
use serde::{Deserialize, Serialize};

/// Configuration for a template, defining its identity and the steps required to render it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "TemplateConfigShadow")]
pub struct TemplateConfig {
    /// Optional unique identifier for the template.
    id: Option<String>,
    /// The sequence of rendering steps to perform.
    steps: Vec<TemplateStep>,
}

#[derive(Deserialize)]
struct TemplateConfigShadow {
    id: Option<String>,
    #[serde(default)]
    steps: Vec<TemplateStep>,
}

impl TryFrom<TemplateConfigShadow> for TemplateConfig {
    type Error = TemplateConfigError;

    fn try_from(shadow: TemplateConfigShadow) -> Result<Self, Self::Error> {
        let config = Self {
            id: shadow.id,
            steps: shadow.steps,
        };
        config.validate()?;
        Ok(config)
    }
}

impl TemplateConfig {
    /// Creates a new template configuration.
    ///
    /// # Errors
    /// Returns an error if the configuration is invalid.
    pub fn new(id: Option<String>, steps: Vec<TemplateStep>) -> Result<Self, TemplateConfigError> {
        let config = Self { id, steps };
        config.validate()?;
        Ok(config)
    }

    /// Validates the configuration state.
    ///
    /// Note: Allows empty steps for backward compatibility with legacy templates
    /// that might use an empty template.yaml and rely on default content/ rendering.
    pub fn validate(&self) -> Result<(), TemplateConfigError> {
        if let Some(id) = &self.id {
            validate_id_format(id)?;
        }

        for (i, step) in self.steps.iter().enumerate() {
            match step {
                TemplateStep::Render {
                    source,
                    destination,
                } => {
                    if source.trim().is_empty() {
                        return Err(TemplateConfigError::InvalidStep {
                            index: i,
                            message: "render source cannot be empty".to_string(),
                        });
                    }
                    if destination.trim().is_empty() {
                        return Err(TemplateConfigError::InvalidStep {
                            index: i,
                            message: "render destination cannot be empty".to_string(),
                        });
                    }
                }
                TemplateStep::RenderFolder {
                    source,
                    destination,
                } => {
                    if source.trim().is_empty() {
                        return Err(TemplateConfigError::InvalidStep {
                            index: i,
                            message: "render_folder source cannot be empty".to_string(),
                        });
                    }
                    // Even if we render to root, we want an explicit path (e.g. "." or "")
                    // but the validator should ensure it's not JUST whitespace if it was provided.
                    // Actually, let's keep it consistent with Render/Inject.
                    // We allow empty destination for the root of the output directory.
                    // If it is just whitespace but not empty, we still error.
                    if !destination.is_empty() && destination.trim().is_empty() {
                        return Err(TemplateConfigError::InvalidStep {
                            index: i,
                            message: "render_folder destination cannot be just whitespace".to_string(),
                        });
                    }
                }
                TemplateStep::Inject {
                    source,
                    destination,
                    ..
                } => {
                    if source.trim().is_empty() {
                        return Err(TemplateConfigError::InvalidStep {
                            index: i,
                            message: "inject source cannot be empty".to_string(),
                        });
                    }
                    if destination.trim().is_empty() {
                        return Err(TemplateConfigError::InvalidStep {
                            index: i,
                            message: "inject destination cannot be empty".to_string(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Returns the template ID if set.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Sets the template ID and validates it.
    pub fn set_id(&mut self, id: String) -> Result<(), TemplateConfigError> {
        validate_id_format(&id)?;
        self.id = Some(id);
        Ok(())
    }

    /// Returns the list of rendering steps.
    pub fn steps(&self) -> &[TemplateStep] {
        &self.steps
    }
}

fn validate_id_format(id: &str) -> Result<(), TemplateConfigError> {
    use std::sync::OnceLock;
    static ID_REGEX: OnceLock<regex::Regex> = OnceLock::new();
    let re = ID_REGEX.get_or_init(|| {
        regex::Regex::new("^[a-zA-Z0-9_\\-./]+$").expect("invalid template id regex")
    });

    if !re.is_match(id) {
        return Err(TemplateConfigError::InvalidFormat {
            field: "id".to_string(),
            message: format!(
                "invalid template id '{}'. Only alphanumeric characters, hyphens, underscores, dots, and slashes are allowed.",
                id
            ),
        });
    }
    Ok(())
}


/// A single step in the template rendering process.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum TemplateStep {
    /// Renders a single template file.
    Render {
        /// Path to the source template file (relative to template root).
        source: String,
        /// Path to the destination file (relative to output root).
        destination: String,
    },
    /// Renders an entire folder of templates.
    RenderFolder {
        /// Path to the source folder (relative to template root).
        source: String,
        /// Path to the destination folder (relative to output root).
        destination: String,
    },
    /// Injects content into an existing file.
    Inject {
        /// Path to the source template file for the injected content.
        source: String,
        /// Path to the target file where content will be injected.
        destination: String,
        /// Where in the target file the content should be injected.
        injection_target: InjectionTarget,
    },
}

/// Defines where content should be injected into a target file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InjectionTarget {
    /// Append the content to the very end of the file.
    AtEnd,
    /// Insert the content into a specific named region (e.g. // region: name).
    Region(String),
}
