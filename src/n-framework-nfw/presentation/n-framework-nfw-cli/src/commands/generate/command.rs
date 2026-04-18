use n_framework_core_cli_abstractions::{PromptService, SelectOption};
use n_framework_nfw_core_application::features::template_management::commands::generate::generate_command::GenerateCommand;
use n_framework_nfw_core_application::features::template_management::commands::generate::generate_command_handler::GenerateCommandHandler;
pub use n_framework_nfw_core_application::features::template_management::models::errors::generate_error::GenerateError;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_domain::features::template_management::template_config::{TemplateInput, TemplateInputType};

/// CLI command implementation for the `generate` subcommand.
#[derive(Debug, Clone)]
pub struct GenerateCliCommand<W, R, E, P> {
    handler: GenerateCommandHandler<W, R, E>,
    prompt: P,
}

/// Request parameters for a generation operation.
#[derive(Debug, Clone)]
pub struct GenerateRequest<'a> {
    /// The type of component to generate (e.g. 'command', 'query').
    pub generator_type: &'a str,
    /// Optional name; when absent and interactive, user is prompted.
    pub name: Option<&'a str>,
    /// Optional feature name to associate the component with.
    pub feature: Option<&'a str>,
    /// Optional arbitrary parameters as 'Key=Value' pairs.
    pub params: Option<&'a str>,
    /// Whether interactive prompts are disabled.
    pub no_input: bool,
    /// Whether the current terminal is interactive.
    pub is_interactive_terminal: bool,
}

impl<W, R, E, P> GenerateCliCommand<W, R, E, P>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
    P: PromptService,
{
    pub fn new(handler: GenerateCommandHandler<W, R, E>, prompt: P) -> Self {
        Self { handler, prompt }
    }

    pub fn execute(&self, request: GenerateRequest) -> Result<(), GenerateError> {
        let name = self.resolve_name(&request)?;
        let config = self.handler.get_template_config(request.generator_type)?;

        let resolved_params = self.resolve_params(&request, config.inputs())?;

        let params_opt = if resolved_params.as_object().is_none_or(|o| o.is_empty()) {
            None
        } else {
            Some(resolved_params)
        };

        let command = GenerateCommand::new(
            request.generator_type,
            name.as_str(),
            request.feature.map(ToOwned::to_owned),
            params_opt,
        );

        self.handler.handle(&command)?;

        println!(
            "Generated '{}' '{}' successfully.",
            request.generator_type, name
        );

        Ok(())
    }

    fn resolve_name(&self, request: &GenerateRequest) -> Result<String, GenerateError> {
        if let Some(name) = request.name {
            return Ok(name.to_owned());
        }

        if request.no_input || !request.is_interactive_terminal {
            return Err(GenerateError::InvalidIdentifier(
                "name is required. Provide it as a positional argument or run interactively."
                    .to_string(),
            ));
        }

        self.prompt
            .text(
                &format!("Enter {} name (e.g. ApproveOrder):", request.generator_type),
                None,
            )
            .map_err(|e| GenerateError::WorkspaceError(e.to_string()))
    }

    fn resolve_params(
        &self,
        request: &GenerateRequest,
        inputs: &[TemplateInput],
    ) -> Result<serde_json::Value, GenerateError> {
        let mut map = serde_json::Map::new();

        if let Some(params_str) = request.params {
            for param_pair in params_str.split(',') {
                if let Some((key, value)) = param_pair.split_once('=') {
                    let key = key.trim();
                    let value = value.trim();
                    let json_value = match value.to_lowercase().as_str() {
                        "true" => serde_json::Value::Bool(true),
                        "false" => serde_json::Value::Bool(false),
                        _ => serde_json::Value::String(value.to_string()),
                    };
                    map.insert(key.to_string(), json_value);
                } else {
                    return Err(GenerateError::InvalidParameter(format!(
                        "invalid parameter format '{}'. expected Key=Value",
                        param_pair
                    )));
                }
            }
        }

        if !request.no_input && request.is_interactive_terminal {
            for input in inputs {
                let id = input.id.clone().unwrap_or_else(|| "Unknown".to_string());
                if !map.contains_key(&id) {
                    let value = self.prompt_for_input(input)?;
                    map.insert(id, value);
                }
            }
        }

        Ok(serde_json::Value::Object(map))
    }

    fn prompt_for_input(&self, input: &TemplateInput) -> Result<serde_json::Value, GenerateError> {
        match &input.input_type {
            TemplateInputType::Text | TemplateInputType::Password => {
                let default_str = input.default.as_ref().and_then(|v| v.as_str());
                let result = self
                    .prompt
                    .text(&input.prompt, default_str)
                    .map_err(|e| GenerateError::WorkspaceError(e.to_string()))?;
                Ok(serde_json::Value::String(result))
            }
            TemplateInputType::Confirm => {
                let default_bool = input
                    .default
                    .as_ref()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let result = self
                    .prompt
                    .confirm(&input.prompt, default_bool)
                    .map_err(|e| GenerateError::WorkspaceError(e.to_string()))?;
                Ok(serde_json::Value::Bool(result))
            }
            TemplateInputType::Select => {
                let options = input.options.as_deref().unwrap_or(&[]);
                let select_options: Vec<SelectOption> =
                    options.iter().map(|s| SelectOption::new(s, s)).collect();
                let default_idx = input
                    .default
                    .as_ref()
                    .and_then(|v| v.as_str())
                    .and_then(|d| options.iter().position(|o| o == d));
                let selected = self
                    .prompt
                    .select(&input.prompt, &select_options, default_idx)
                    .map_err(|e| GenerateError::WorkspaceError(e.to_string()))?;
                Ok(serde_json::Value::String(selected.value().to_string()))
            }
            TemplateInputType::Multiselect => {
                let options = input.options.as_deref().unwrap_or(&[]);
                let select_options: Vec<SelectOption> =
                    options.iter().map(|s| SelectOption::new(s, s)).collect();
                let selected = self
                    .prompt
                    .multiselect(&input.prompt, &select_options, &[])
                    .map_err(|e| GenerateError::WorkspaceError(e.to_string()))?;
                let selected_values: Vec<serde_json::Value> = selected
                    .iter()
                    .map(|s| serde_json::Value::String(s.value().to_string()))
                    .collect();
                Ok(serde_json::Value::Array(selected_values))
            }
            TemplateInputType::Object => {
                println!("{}", input.prompt);
                let mut obj_map = serde_json::Map::new();
                if let Some(props) = &input.properties {
                    for prop in props {
                        let id = prop.id.clone().unwrap_or_else(|| "Unknown".to_string());
                        let value = self.prompt_for_input(prop)?;
                        obj_map.insert(id, value);
                    }
                }
                Ok(serde_json::Value::Object(obj_map))
            }
            TemplateInputType::List => {
                let mut list = Vec::new();
                if let Some(item_schema) = &input.items {
                    loop {
                        let add_more = self
                            .prompt
                            .confirm(&format!("Add an item to {}?", input.prompt), false)
                            .map_err(|e| GenerateError::WorkspaceError(e.to_string()))?;
                        if !add_more {
                            break;
                        }
                        let item_value = self.prompt_for_input(item_schema)?;
                        list.push(item_value);
                    }
                }
                Ok(serde_json::Value::Array(list))
            }
        }
    }
}
