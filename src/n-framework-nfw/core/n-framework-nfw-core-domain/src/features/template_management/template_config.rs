use crate::features::template_management::errors::TemplateConfigError;
use serde::{Deserialize, Serialize};

/// Configuration for a template, defining its identity and the steps required to render it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "TemplateConfigShadow")]
pub struct TemplateConfig {
    /// Optional unique identifier for the template.
    id: Option<String>,
    /// The sequence of rendering steps to perform.
    #[serde(default)]
    steps: Vec<TemplateStep>,
    /// The input parameters accepted by this template.
    #[serde(default)]
    inputs: Vec<TemplateInput>,
}

#[derive(Deserialize)]
struct TemplateConfigShadow {
    id: Option<String>,
    #[serde(default)]
    steps: Vec<TemplateStep>,
    #[serde(default)]
    inputs: Vec<TemplateInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TemplateInputType {
    Text,
    Password,
    Confirm,
    Select,
    Multiselect,
    Object,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemplateInput {
    /// Unique identifier for the input variable, not strictly required for list elements.
    pub id: Option<String>,
    /// Input type (e.g., text, confirm, select).
    #[serde(rename = "type")]
    pub input_type: TemplateInputType,
    /// Message displayed to the user when prompted.
    pub prompt: String,
    /// Optional default value.
    pub default: Option<serde_json::Value>,
    /// Valid choices for select/multiselect types.
    pub options: Option<Vec<String>>,
    /// Nested properties for 'object' type inputs.
    pub properties: Option<Vec<TemplateInput>>,
    /// Element schema for 'list' type inputs.
    pub items: Option<Box<TemplateInput>>,
}

impl TryFrom<TemplateConfigShadow> for TemplateConfig {
    type Error = TemplateConfigError;

    fn try_from(shadow: TemplateConfigShadow) -> Result<Self, Self::Error> {
        let config = Self {
            id: shadow.id,
            steps: shadow.steps,
            inputs: shadow.inputs,
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
    pub fn new(
        id: Option<String>,
        steps: Vec<TemplateStep>,
        inputs: Vec<TemplateInput>,
    ) -> Result<Self, TemplateConfigError> {
        let config = Self { id, steps, inputs };
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
                            message: "render_folder destination cannot be just whitespace"
                                .to_string(),
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
        for (i, input) in self.inputs.iter().enumerate() {
            input.validate(i, None)?;
        }
        Ok(())
    }
}

impl TemplateInput {
    fn validate(&self, index: usize, parent_id: Option<&str>) -> Result<(), TemplateConfigError> {
        let input_id = self.id.as_deref().unwrap_or("unknown");
        let context = if let Some(parent) = parent_id {
            format!("property '{}' of object '{}'", input_id, parent)
        } else {
            format!("input '{}' at index {}", input_id, index)
        };

        if self.id.is_none() && parent_id.is_some() {
            return Err(TemplateConfigError::InvalidFormat {
                field: "id".to_string(),
                message: format!("missing id for {}", context),
            });
        }

        match self.input_type {
            TemplateInputType::Select | TemplateInputType::Multiselect => {
                let opts =
                    self.options
                        .as_ref()
                        .ok_or_else(|| TemplateConfigError::InvalidFormat {
                            field: "options".to_string(),
                            message: format!("{} must have options defined", context),
                        })?;
                if opts.is_empty() {
                    return Err(TemplateConfigError::InvalidFormat {
                        field: "options".to_string(),
                        message: format!("{} has an empty options list", context),
                    });
                }
            }
            TemplateInputType::Object => {
                let props =
                    self.properties
                        .as_ref()
                        .ok_or_else(|| TemplateConfigError::InvalidFormat {
                            field: "properties".to_string(),
                            message: format!("{} must have properties defined", context),
                        })?;
                if props.is_empty() {
                    return Err(TemplateConfigError::InvalidFormat {
                        field: "properties".to_string(),
                        message: format!("{} has an empty properties list", context),
                    });
                }
                for (j, prop) in props.iter().enumerate() {
                    prop.validate(j, Some(input_id))?;
                }
            }
            TemplateInputType::List => {
                if self.items.is_none() {
                    return Err(TemplateConfigError::InvalidFormat {
                        field: "items".to_string(),
                        message: format!("{} must have an items schema defined", context),
                    });
                }
                // Recursively validate list items
                self.items.as_ref().unwrap().validate(0, Some(input_id))?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl TemplateConfig {
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

    /// Returns the list of template inputs (parameters).
    pub fn inputs(&self) -> &[TemplateInput] {
        &self.inputs
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
