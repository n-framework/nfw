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
    /// Optional JSON parameters as a raw JSON string.
    pub param_json: Option<&'a str>,
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
        let context = self.handler.prepare_context(request.generator_type)?;
        let name = self.resolve_name(&request)?;

        let resolved_params = self.resolve_params(&request, context.config.inputs())?;

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

        self.handler.handle(&command, context)?;

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

        // 1. Resolve from --param (Key=Value, comma separated, supports quotes)
        if let Some(params_str) = request.params {
            for (key, value) in self.parse_param_pairs(params_str)? {
                map.insert(key, serde_json::Value::String(value));
            }
        }

        // 2. Resolve from --param-json (Raw JSON object)
        if let Some(json_str) = request.param_json {
            let json_val: serde_json::Value = serde_json::from_str(json_str).map_err(|e| {
                GenerateError::InvalidParameter(format!("invalid JSON in --param-json: {e}"))
            })?;

            if let serde_json::Value::Object(json_map) = json_val {
                for (k, v) in json_map {
                    if map.contains_key(&k) {
                        return Err(GenerateError::InvalidParameter(format!(
                            "parameter conflict: '{}' is defined in both --param and --param-json. Use one or the other for each key.",
                            k
                        )));
                    }
                    map.insert(k, v);
                }
            } else {
                return Err(GenerateError::InvalidParameter(
                    "--param-json must be a JSON object (e.g., --param-json '{\"key\": \"value\"}')"
                        .to_string(),
                ));
            }
        }

        if !request.no_input && request.is_interactive_terminal {
            for input in inputs {
                let id = input.id();
                if !map.contains_key(id) {
                    let value = self.prompt_for_input(input)?;
                    map.insert(id.to_string(), value);
                }
            }
        }

        // Final validation: Ensure all defined inputs have been resolved.
        // This catches missing parameters when in --no-input mode.
        for input in inputs {
            let id = input.id();
            if !map.contains_key(id) {
                return Err(GenerateError::InvalidParameter(format!(
                    "required parameter '{}' was not provided",
                    id
                )));
            }
        }

        Ok(serde_json::Value::Object(map))
    }

    pub fn prompt_for_input(
        &self,
        input: &TemplateInput,
    ) -> Result<serde_json::Value, GenerateError> {
        match input.input_type() {
            TemplateInputType::Text => self
                .prompt
                .text(input.prompt(), None)
                .map(serde_json::Value::String)
                .map_err(|e| GenerateError::WorkspaceError(e.to_string())),
            TemplateInputType::Password => self
                .prompt
                .password(input.prompt())
                .map(serde_json::Value::String)
                .map_err(|e| GenerateError::WorkspaceError(e.to_string())),
            TemplateInputType::Confirm => {
                let default_bool = input.default().and_then(|v| v.as_bool()).unwrap_or(false);
                let result = self
                    .prompt
                    .confirm(input.prompt(), default_bool)
                    .map_err(|e| GenerateError::WorkspaceError(e.to_string()))?;
                Ok(serde_json::Value::Bool(result))
            }
            TemplateInputType::Select => {
                let options = input.options().ok_or_else(|| {
                    GenerateError::InvalidParameter(format!(
                        "select input '{}' has no options defined",
                        input.id()
                    ))
                })?;
                if options.is_empty() {
                    return Err(GenerateError::InvalidParameter(format!(
                        "select input '{}' has an empty options list",
                        input.id()
                    )));
                }
                let select_options: Vec<SelectOption> =
                    options.iter().map(|s| SelectOption::new(s, s)).collect();
                let default_idx = input
                    .default()
                    .and_then(|v| v.as_str())
                    .and_then(|d| options.iter().position(|o| o == d));
                let selected = self
                    .prompt
                    .select(input.prompt(), &select_options, default_idx)
                    .map_err(|e| GenerateError::WorkspaceError(e.to_string()))?;
                Ok(serde_json::Value::String(selected.value().to_string()))
            }
            TemplateInputType::Multiselect => {
                let options = input.options().ok_or_else(|| {
                    GenerateError::InvalidParameter(format!(
                        "multiselect input '{}' has no options defined",
                        input.id()
                    ))
                })?;
                if options.is_empty() {
                    return Err(GenerateError::InvalidParameter(format!(
                        "multiselect input '{}' has an empty options list",
                        input.id()
                    )));
                }
                let select_options: Vec<SelectOption> =
                    options.iter().map(|s| SelectOption::new(s, s)).collect();
                let selected = self
                    .prompt
                    .multiselect(input.prompt(), &select_options, &[])
                    .map_err(|e| GenerateError::WorkspaceError(e.to_string()))?;
                let selected_values: Vec<serde_json::Value> = selected
                    .iter()
                    .map(|s| serde_json::Value::String(s.value().to_string()))
                    .collect();
                Ok(serde_json::Value::Array(selected_values))
            }
            TemplateInputType::Object => {
                println!("{}", input.prompt());
                let mut obj_map = serde_json::Map::new();
                let props = input.properties().ok_or_else(|| {
                    GenerateError::InvalidParameter(format!(
                        "object input '{}' has no properties defined",
                        input.id()
                    ))
                })?;
                for prop in props {
                    let id = prop.id();
                    let value = self.prompt_for_input(prop)?;
                    obj_map.insert(id.to_string(), value);
                }
                Ok(serde_json::Value::Object(obj_map))
            }
            TemplateInputType::List => {
                let mut list = Vec::new();
                if let Some(item_schema) = input.items() {
                    loop {
                        let add_more = self
                            .prompt
                            .confirm(&format!("Add an item to {}?", input.prompt()), false)
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

    /// Parses 'Key=Value' pairs from a comma-separated string, respecting quotes.
    /// Example: 'Key1=Value1,Key2="Value with, comma"'
    fn parse_param_pairs(&self, input: &str) -> Result<Vec<(String, String)>, GenerateError> {
        let mut pairs = Vec::new();
        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut in_quotes = false;
        let mut in_key = true;
        let chars = input.chars();

        for c in chars {
            match c {
                '=' if in_key && !in_quotes => {
                    in_key = false;
                }
                ',' if !in_key && !in_quotes => {
                    if current_key.trim().is_empty() {
                        return Err(GenerateError::InvalidParameter(
                            "parameter key cannot be empty in --param. Check for trailing or consecutive commas."
                                .to_string(),
                        ));
                    }
                    pairs.push((
                        current_key.trim().to_string(),
                        current_value.trim().to_string(),
                    ));
                    current_key.clear();
                    current_value.clear();
                    in_key = true;
                }
                '"' => {
                    in_quotes = !in_quotes;
                }
                _ => {
                    if in_key {
                        current_key.push(c);
                    } else {
                        current_value.push(c);
                    }
                }
            }
        }

        if in_quotes {
            return Err(GenerateError::InvalidParameter(
                "unclosed quotes in --param. Ensure all opening quotes have a matching closing quote."
                    .to_string(),
            ));
        }

        if !current_key.trim().is_empty() {
            if in_key {
                return Err(GenerateError::InvalidParameter(format!(
                    "invalid parameter format '{}' in --param. Each parameter must be in Key=Value format.",
                    current_key
                )));
            }
            pairs.push((
                current_key.trim().to_string(),
                current_value.trim().to_string(),
            ));
        } else if !current_value.trim().is_empty() {
            return Err(GenerateError::InvalidParameter(
                "parameter key cannot be empty in --param. Check for leading commas or missing keys."
                    .to_string(),
            ));
        }

        Ok(pairs)
    }
}
