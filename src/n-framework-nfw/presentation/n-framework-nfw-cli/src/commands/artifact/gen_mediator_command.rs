use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};
use n_framework_nfw_core_application::features::template_management::commands::gen_mediator_command::gen_mediator_command_command::GenMediatorCommandCommand;
use n_framework_nfw_core_application::features::template_management::commands::gen_mediator_command::gen_mediator_command_command_handler::GenMediatorCommandCommandHandler;
use n_framework_nfw_core_application::features::template_management::services::artifact_generation_service::WorkspaceContext;
pub use n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_domain::features::template_management::template_config::{TemplateInput, TemplateInputType};

#[derive(Debug, Clone)]
pub struct GenMediatorCommandCliCommand<W, R, E, P> {
    handler: GenMediatorCommandCommandHandler<W, R, E>,
    prompt: P,
}

pub struct GenMediatorCommandRequest<'a> {
    pub name: Option<&'a str>,
    pub feature: Option<&'a str>,
    pub params: Option<&'a str>,
    pub param_json: Option<&'a str>,
    pub no_input: bool,
    pub is_interactive_terminal: bool,
}

impl<W, R, E, P> GenMediatorCommandCliCommand<W, R, E, P>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
    P: InteractivePrompt + Logger,
{
    pub fn new(handler: GenMediatorCommandCommandHandler<W, R, E>, prompt: P) -> Self {
        Self { handler, prompt }
    }

    pub fn execute(&self, request: GenMediatorCommandRequest) -> Result<(), AddArtifactError> {
        self.prompt
            .intro("Generate Mediator Command")
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        let workspace_context = self.handler.get_workspace_context()?;
        let services = self.handler.extract_services(&workspace_context)?;

        if services.is_empty() {
            return Err(AddArtifactError::WorkspaceError(
                "No services found in workspace. Add a service first.".to_string(),
            ));
        }

        let selected_service =
            if (request.no_input || !request.is_interactive_terminal) && services.len() == 1 {
                services.into_iter().next().unwrap()
            } else {
                let options: std::vec::Vec<SelectOption> = services
                    .iter()
                    .map(|s| SelectOption::new(&s.name, &s.name))
                    .collect();
                let selected = self
                    .prompt
                    .select("Select a service for the new command:", &options, Some(0))
                    .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

                services
                    .into_iter()
                    .find(|s| s.name == selected.value())
                    .unwrap()
            };

        let context_workspace = WorkspaceContext {
            workspace_root: workspace_context.workspace_root.clone(),
            nfw_yaml: workspace_context.nfw_yaml.clone(),
        };

        // Note: Generator type is strictly "command" for Mediator Commands
        let context =
            self.handler
                .load_template_context(workspace_context, &selected_service, "command")?;

        let existing_features = self
            .handler
            .list_features(&context_workspace, &selected_service)?;
        let feature = self.resolve_feature(&request, existing_features)?;
        let name = self.resolve_name(&request)?;

        let resolved_params = self.resolve_params(&request, context.config.inputs())?;

        let params_opt = if resolved_params.as_object().is_none_or(|o| o.is_empty()) {
            None
        } else {
            Some(resolved_params)
        };

        let command = GenMediatorCommandCommand::new(name.as_str(), feature, params_opt);

        let spinner = self
            .prompt
            .spinner(&format!("Generating mediator command '{}'...", name))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        self.handler.handle(&command, context).map_err(|e| {
            spinner.error(&format!("Failed to generate command: {}", e));
            e
        })?;

        spinner.success(&format!("Command '{}' generated successfully", name));

        self.prompt
            .outro(&format!(
                "Successfully generated Mediator Command '{}'.",
                name
            ))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        Ok(())
    }

    fn resolve_name(
        &self,
        request: &GenMediatorCommandRequest,
    ) -> Result<String, AddArtifactError> {
        if let Some(name) = request.name {
            return Ok(name.to_owned());
        }

        if request.no_input || !request.is_interactive_terminal {
            return Err(AddArtifactError::InvalidIdentifier(
                "name is required. Provide it as a positional argument or run interactively."
                    .to_string(),
            ));
        }

        self.prompt
            .text("Enter command name (e.g. ApproveOrderCommand):", None)
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))
    }

    fn resolve_feature(
        &self,
        request: &GenMediatorCommandRequest,
        existing_features: Vec<String>,
    ) -> Result<Option<String>, AddArtifactError> {
        if let Some(feature) = request.feature {
            return Ok(Some(feature.to_owned()));
        }

        if request.no_input || !request.is_interactive_terminal {
            return Ok(None);
        }

        if existing_features.is_empty() {
            let feature = self
                .prompt
                .text("Enter feature name (e.g. Catalog):", None)
                .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

            if feature.trim().is_empty() {
                return Ok(None);
            } else {
                return Ok(Some(feature.trim().to_string()));
            }
        }

        let mut options: Vec<SelectOption> = existing_features
            .iter()
            .map(|f| SelectOption::new(f, f))
            .collect();

        const CREATE_NEW: &str = "__create_new__";
        options.push(SelectOption::new("[Create new feature]", CREATE_NEW));

        let selected = self
            .prompt
            .select("Select feature:", &options, Some(0))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        if selected.value() == CREATE_NEW {
            let feature = self
                .prompt
                .text("Enter new feature name (e.g. Catalog):", None)
                .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

            if feature.trim().is_empty() {
                Ok(None)
            } else {
                Ok(Some(feature.trim().to_string()))
            }
        } else {
            Ok(Some(selected.value().to_string()))
        }
    }

    fn resolve_params(
        &self,
        request: &GenMediatorCommandRequest,
        inputs: &[TemplateInput],
    ) -> Result<serde_json::Value, AddArtifactError> {
        let mut map = serde_json::Map::new();

        if let Some(params_str) = request.params {
            for (key, value) in self.parse_param_pairs(params_str)? {
                map.insert(key, serde_json::Value::String(value));
            }
        }

        if let Some(json_str) = request.param_json {
            let json_val: serde_json::Value = serde_json::from_str(json_str).map_err(|e| {
                AddArtifactError::InvalidParameter(format!("invalid JSON in --param-json: {e}"))
            })?;

            if let serde_json::Value::Object(json_map) = json_val {
                for (k, v) in json_map {
                    if map.contains_key(&k) {
                        return Err(AddArtifactError::InvalidParameter(format!(
                            "parameter conflict: '{}' is defined in both --param and --param-json. Use one or the other for each key.",
                            k
                        )));
                    }
                    map.insert(k, v);
                }
            } else {
                return Err(AddArtifactError::InvalidParameter(
                    "--param-json must be a JSON object (e.g., --param-json '{\"key\": \"value\"}')"
                        .to_string(),
                ));
            }
        }

        if !request.no_input && request.is_interactive_terminal {
            for input in inputs {
                let id = input.id();
                if map.contains_key(id) {
                    continue;
                }
                let value = self.prompt_for_input(input)?;
                map.insert(id.to_string(), value);
            }
        }

        for input in inputs {
            let id = input.id();
            if !map.contains_key(id) {
                return Err(AddArtifactError::InvalidParameter(format!(
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
    ) -> Result<serde_json::Value, AddArtifactError> {
        let mut prompt_message = input.prompt().to_string();
        if let Some(description) = input.description() {
            prompt_message = format!("{}\n  {}", prompt_message, description);
        }

        match input.input_type() {
            TemplateInputType::Text => self
                .prompt
                .text(&prompt_message, None)
                .map(serde_json::Value::String)
                .map_err(|e| AddArtifactError::WorkspaceError(e.to_string())),
            TemplateInputType::Password => self
                .prompt
                .password(&prompt_message)
                .map(serde_json::Value::String)
                .map_err(|e| AddArtifactError::WorkspaceError(e.to_string())),
            TemplateInputType::Confirm => {
                let default_bool = input.default().and_then(|v| v.as_bool()).unwrap_or(false);
                let result = self
                    .prompt
                    .confirm(&prompt_message, default_bool)
                    .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;
                Ok(serde_json::Value::Bool(result))
            }
            TemplateInputType::Select => {
                let options = input.options().ok_or_else(|| {
                    AddArtifactError::InvalidParameter(format!(
                        "select input '{}' has no options defined",
                        input.id()
                    ))
                })?;
                let select_options: Vec<SelectOption> =
                    options.iter().map(|s| SelectOption::new(s, s)).collect();
                let default_idx = input
                    .default()
                    .and_then(|v| v.as_str())
                    .and_then(|d| options.iter().position(|o| o == d));
                let selected = self
                    .prompt
                    .select(&prompt_message, &select_options, default_idx)
                    .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;
                Ok(serde_json::Value::String(selected.value().to_string()))
            }
            TemplateInputType::Multiselect => {
                let options = input.options().ok_or_else(|| {
                    AddArtifactError::InvalidParameter(format!(
                        "multiselect input '{}' has no options defined",
                        input.id()
                    ))
                })?;
                let select_options: Vec<SelectOption> =
                    options.iter().map(|s| SelectOption::new(s, s)).collect();

                let defaults = input.default().and_then(|v| v.as_array());
                let default_indices: Vec<usize> = if let Some(arr) = defaults {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .filter_map(|d| options.iter().position(|o| o == d))
                        .collect()
                } else {
                    Vec::new()
                };

                let selected = self
                    .prompt
                    .multiselect(&prompt_message, &select_options, &default_indices)
                    .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

                let selected_values: Vec<serde_json::Value> = selected
                    .iter()
                    .map(|s| serde_json::Value::String(s.value().to_string()))
                    .collect();
                Ok(serde_json::Value::Array(selected_values))
            }
            TemplateInputType::Object => {
                let mut obj_map = serde_json::Map::new();
                let props = input.properties().ok_or_else(|| {
                    AddArtifactError::InvalidParameter(format!(
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
                            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;
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

    fn parse_param_pairs(&self, input: &str) -> Result<Vec<(String, String)>, AddArtifactError> {
        let mut pairs = Vec::new();
        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut in_quotes = false;
        let mut in_key = true;

        for c in input.chars() {
            match c {
                '=' if in_key && !in_quotes => {
                    in_key = false;
                }
                ',' if !in_key && !in_quotes => {
                    pairs.push((
                        current_key.trim().to_string(),
                        current_value.trim().to_string(),
                    ));
                    current_key.clear();
                    current_value.clear();
                    in_key = true;
                }
                '"' => in_quotes = !in_quotes,
                _ => {
                    if in_key {
                        current_key.push(c);
                    } else {
                        current_value.push(c);
                    }
                }
            }
        }

        if !current_key.trim().is_empty() {
            pairs.push((
                current_key.trim().to_string(),
                current_value.trim().to_string(),
            ));
        }

        Ok(pairs)
    }
}
