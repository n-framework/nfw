use crate::features::generator_management::errors::GeneratorConfigError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for a generator, defining its identity and the steps required to render it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "GeneratorConfigShadow")]
pub struct GeneratorConfig {
    /// Optional unique identifier for the generator.
    id: Option<String>,
    /// The sequence of rendering steps to perform.
    #[serde(default)]
    steps: Vec<GeneratorStepConfig>,
    /// The input parameters accepted by this generator.
    #[serde(default)]
    inputs: Vec<GeneratorInput>,
    /// Modules that must be present in the target service before this generator can execute.
    #[serde(default)]
    required_modules: Vec<String>,
    /// Explicit paths for nested generators.
    #[serde(default)]
    generators: Option<HashMap<String, String>>,
    /// Generator types whose step destinations are scanned when attaching this generator output to
    /// an existing mediator artifact (command or query). Each entry is a key in the parent
    /// generator's `generators` map (e.g. `"command"`, `"query"`).
    #[serde(default)]
    mediator_sources: Vec<String>,
}

#[derive(Deserialize)]
struct GeneratorConfigShadow {
    id: Option<String>,
    #[serde(default)]
    steps: Vec<GeneratorStepConfig>,
    #[serde(default)]
    inputs: Vec<GeneratorInput>,
    #[serde(default)]
    required_modules: Vec<String>,
    #[serde(default)]
    generators: Option<HashMap<String, String>>,
    #[serde(default)]
    mediator_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorInputType {
    Text,
    Password,
    Confirm,
    Select,
    Multiselect,
    Object,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "GeneratorInputShadow")]
pub struct GeneratorInput {
    /// Unique identifier for the input variable.
    id: String,
    /// Input type (e.g., text, confirm, select).
    #[serde(rename = "type")]
    input_type: GeneratorInputType,
    /// Message displayed to the user when prompted.
    prompt: String,
    /// Optional description displayed below the prompt.
    description: Option<String>,
    /// Optional default value.
    default: Option<serde_json::Value>,
    /// Valid choices for select/multiselect types.
    options: Option<Vec<String>>,
    /// Nested properties for 'object' type inputs.
    properties: Option<Vec<GeneratorInput>>,
    /// Element schema for 'list' type inputs.
    items: Option<Box<GeneratorInput>>,
}

#[derive(Deserialize)]
struct GeneratorInputShadow {
    id: Option<String>,
    #[serde(rename = "type")]
    input_type: GeneratorInputType,
    prompt: String,
    description: Option<String>,
    default: Option<serde_json::Value>,
    options: Option<Vec<String>>,
    properties: Option<Vec<GeneratorInput>>,
    items: Option<Box<GeneratorInput>>,
}

impl TryFrom<GeneratorConfigShadow> for GeneratorConfig {
    type Error = GeneratorConfigError;

    fn try_from(shadow: GeneratorConfigShadow) -> Result<Self, Self::Error> {
        let config = Self {
            id: shadow.id,
            steps: shadow.steps,
            inputs: shadow.inputs,
            required_modules: shadow.required_modules,
            generators: shadow.generators,
            mediator_sources: shadow.mediator_sources,
        };
        config.validate()?;
        Ok(config)
    }
}

impl TryFrom<GeneratorInputShadow> for GeneratorInput {
    type Error = GeneratorConfigError;

    fn try_from(shadow: GeneratorInputShadow) -> Result<Self, Self::Error> {
        let id = shadow
            .id
            .ok_or_else(|| GeneratorConfigError::InvalidInput {
                id: "unknown".to_string(),
                message: "missing 'id' for generator input".to_string(),
            })?;

        if id.trim().is_empty() {
            return Err(GeneratorConfigError::InvalidInput {
                id: "unknown".to_string(),
                message: "generator input 'id' cannot be empty".to_string(),
            });
        }

        let input = Self {
            id,
            input_type: shadow.input_type,
            prompt: shadow.prompt,
            description: shadow.description,
            default: shadow.default,
            options: shadow.options,
            properties: shadow.properties,
            items: shadow.items,
        };
        // Perform initial validation during deserialization
        input.validate(0, None)?;
        Ok(input)
    }
}

impl GeneratorConfig {
    /// Creates a new generator configuration.
    ///
    /// # Errors
    /// Returns an error if the configuration is invalid.
    pub fn new(
        id: Option<String>,
        steps: Vec<GeneratorStepConfig>,
        inputs: Vec<GeneratorInput>,
    ) -> Result<Self, GeneratorConfigError> {
        let config = Self {
            id,
            steps,
            inputs,
            required_modules: Vec::new(),
            generators: None,
            mediator_sources: Vec::new(),
        };
        config.validate()?;
        Ok(config)
    }

    /// Validates the configuration state.
    pub fn validate(&self) -> Result<(), GeneratorConfigError> {
        if let Some(id) = &self.id {
            validate_id_format(id)?;
        }

        // Allow steps to be empty if the generator defines generators (acting as a parent generator)
        if self.steps.is_empty() && self.generators.as_ref().is_none_or(|g| g.is_empty()) {
            return Err(GeneratorConfigError::InvalidStep {
                index: 0,
                message: "A generator must define at least one step or specify child generators"
                    .to_string(),
            });
        }

        for (i, step) in self.steps.iter().enumerate() {
            match &step.action {
                GeneratorStepAction::Render {
                    source,
                    destination,
                }
                | GeneratorStepAction::RenderIfAbsent {
                    source,
                    destination,
                } => {
                    if source.trim().is_empty() {
                        return Err(GeneratorConfigError::InvalidStep {
                            index: i,
                            message: "render source cannot be empty".to_string(),
                        });
                    }
                    if destination.trim().is_empty() {
                        return Err(GeneratorConfigError::InvalidStep {
                            index: i,
                            message: "render destination cannot be empty".to_string(),
                        });
                    }
                }
                GeneratorStepAction::RenderFolder {
                    source,
                    destination,
                } => {
                    if source.trim().is_empty() {
                        return Err(GeneratorConfigError::InvalidStep {
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
                        return Err(GeneratorConfigError::InvalidStep {
                            index: i,
                            message: "render_folder destination cannot be just whitespace"
                                .to_string(),
                        });
                    }
                }
                GeneratorStepAction::Inject {
                    source,
                    destination,
                    injection_target: _,
                } => {
                    if source.trim().is_empty() {
                        return Err(GeneratorConfigError::InvalidStep {
                            index: i,
                            message: "inject source cannot be empty".to_string(),
                        });
                    }
                    if destination.trim().is_empty() {
                        return Err(GeneratorConfigError::InvalidStep {
                            index: i,
                            message: "inject destination cannot be empty".to_string(),
                        });
                    }
                }
                GeneratorStepAction::RunCommand { command, .. } => {
                    if command.trim().is_empty() {
                        return Err(GeneratorConfigError::InvalidStep {
                            index: i,
                            message: "run_command command cannot be empty".to_string(),
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

impl GeneratorInput {
    /// Creates a new generator input.
    pub fn new(id: String, input_type: GeneratorInputType, prompt: String) -> Self {
        Self {
            id,
            input_type,
            prompt,
            description: None,
            default: None,
            options: None,
            properties: None,
            items: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_default(mut self, default: serde_json::Value) -> Self {
        self.default = Some(default);
        self
    }

    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = Some(options);
        self
    }

    pub fn with_properties(mut self, properties: Vec<GeneratorInput>) -> Self {
        self.properties = Some(properties);
        self
    }

    pub fn with_items(mut self, items: GeneratorInput) -> Self {
        self.items = Some(Box::new(items));
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn input_type(&self) -> &GeneratorInputType {
        &self.input_type
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn default(&self) -> Option<&serde_json::Value> {
        self.default.as_ref()
    }

    pub fn options(&self) -> Option<&[String]> {
        self.options.as_deref()
    }

    pub fn properties(&self) -> Option<&[GeneratorInput]> {
        self.properties.as_deref()
    }

    pub fn items(&self) -> Option<&GeneratorInput> {
        self.items.as_ref().map(|i| i.as_ref())
    }

    fn validate(&self, index: usize, parent_id: Option<&str>) -> Result<(), GeneratorConfigError> {
        let input_id = self.id.as_str();
        let context = if let Some(parent) = parent_id {
            format!("property '{}' of object '{}'", input_id, parent)
        } else {
            format!("input '{}' at index {}", input_id, index)
        };

        match self.input_type {
            GeneratorInputType::Select | GeneratorInputType::Multiselect => {
                let opts =
                    self.options
                        .as_ref()
                        .ok_or_else(|| GeneratorConfigError::InvalidFormat {
                            field: "options".to_string(),
                            message: format!("{} must have options defined", context),
                        })?;
                if opts.is_empty() {
                    return Err(GeneratorConfigError::InvalidFormat {
                        field: "options".to_string(),
                        message: format!("{} has an empty options list", context),
                    });
                }
            }
            GeneratorInputType::Object => {
                let props = self.properties.as_ref().ok_or_else(|| {
                    GeneratorConfigError::InvalidFormat {
                        field: "properties".to_string(),
                        message: format!("{} must have properties defined", context),
                    }
                })?;
                if props.is_empty() {
                    return Err(GeneratorConfigError::InvalidFormat {
                        field: "properties".to_string(),
                        message: format!("{} has an empty properties list", context),
                    });
                }
                for (j, prop) in props.iter().enumerate() {
                    prop.validate(j, Some(input_id))?;
                }
            }
            GeneratorInputType::List => {
                if self.items.is_none() {
                    return Err(GeneratorConfigError::InvalidFormat {
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

impl GeneratorConfig {
    /// Returns the generator ID if set.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Sets the generator ID and validates it.
    pub fn set_id(&mut self, id: String) -> Result<(), GeneratorConfigError> {
        validate_id_format(&id)?;
        self.id = Some(id);
        Ok(())
    }

    /// Returns the list of rendering steps.
    pub fn steps(&self) -> &[GeneratorStepConfig] {
        &self.steps
    }

    /// Returns the list of generator inputs (parameters).
    pub fn inputs(&self) -> &[GeneratorInput] {
        &self.inputs
    }

    /// Returns the list of required modules for this generator.
    pub fn required_modules(&self) -> &[String] {
        &self.required_modules
    }

    /// Returns the optional nested generator paths configuration.
    ///
    /// This property provides a mapping of dynamic generator types (e.g. `persistence`, `mediator`)
    /// to their relative sub-folder paths within the root generator directory. It allows
    /// complex generators to encompass multiple artifacts within the same generator bundle.
    pub fn generators(&self) -> Option<&HashMap<String, String>> {
        self.generators.as_ref()
    }

    /// Returns the generator types whose step destinations are scanned when attaching this
    /// generator's output to an existing mediator artifact.
    ///
    /// Each entry corresponds to a key in the parent generator's `generators` map
    /// (e.g. `"command"`, `"query"`). An empty slice means no mediator attachment is supported.
    pub fn mediator_sources(&self) -> &[String] {
        &self.mediator_sources
    }
}

fn validate_id_format(id: &str) -> Result<(), GeneratorConfigError> {
    use std::sync::OnceLock;
    static ID_REGEX: OnceLock<regex::Regex> = OnceLock::new();
    let re = ID_REGEX.get_or_init(|| {
        regex::Regex::new("^[a-zA-Z0-9_\\-./]+$").expect("invalid generator id regex")
    });

    if !re.is_match(id) {
        return Err(GeneratorConfigError::InvalidFormat {
            field: "id".to_string(),
            message: format!(
                "invalid generator id '{}'. Only alphanumeric characters, hyphens, underscores, dots, and slashes are allowed.",
                id
            ),
        });
    }
    Ok(())
}

/// A single step in the generator rendering process.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GeneratorStepConfig {
    /// Optional condition (Tera expression). If it evaluates to false, the step is skipped.
    #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// The action to perform for this step.
    #[serde(flatten)]
    pub action: GeneratorStepAction,
}

/// The action of a generator step.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum GeneratorStepAction {
    /// Renders a single generator file.
    Render {
        /// Path to the source generator file (relative to generator root).
        source: String,
        /// Path to the destination file (relative to output root).
        destination: String,
    },
    /// Renders a single generator file only if the destination does not already exist.
    /// Use this for hand-editable scaffolding files that should be created once but never
    /// overwritten by subsequent generator runs (e.g. partial class declarations,
    /// feature grouping registration stubs).
    RenderIfAbsent {
        /// Path to the source generator file (relative to generator root).
        source: String,
        /// Path to the destination file (relative to output root).
        destination: String,
    },
    /// Renders an entire folder of generators.
    RenderFolder {
        /// Path to the source folder (relative to generator root).
        source: String,
        /// Path to the destination folder (relative to output root).
        destination: String,
    },
    /// Injects content into an existing file.
    Inject {
        /// Path to the source generator file for the injected content.
        source: String,
        /// Path to the target file where content will be injected.
        destination: String,
        /// Where in the target file the content should be injected.
        injection_target: InjectionTarget,
    },
    /// Executes a shell command.
    RunCommand {
        /// The command string to execute (supports Tera placeholders).
        command: String,
        /// Optional working directory relative to output root.
        #[serde(default)]
        working_directory: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum InjectionTarget {
    /// Append the content to the very end of the file.
    AtEnd,
    /// Insert the content into a specific named region (e.g. // region: name).
    Region(String),
}

#[cfg(test)]
#[path = "generator_config.tests.rs"]
mod tests;
