use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};
use crate::cli_error::CliError;
use crate::startup::cli_service_collection_factory::CliServiceCollection;
use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;
use n_framework_nfw_core_application::features::template_management::commands::gen_endpoint::gen_endpoint_command::{GenEndpointCommand, HttpMethod};
use n_framework_nfw_core_application::features::template_management::commands::gen_endpoint::gen_endpoint_command_handler::GenEndpointCommandHandler;
pub use n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_domain::features::template_management::template_config::{TemplateInput, TemplateInputType};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct GenEndpointCliCommand<W, R, E, P> {
    handler: GenEndpointCommandHandler<W, R, E>,
    prompt: P,
}

pub struct GenEndpointRequest<'a> {
    pub operation_type: Option<&'a str>,
    pub name: Option<&'a str>,
    pub feature: Option<&'a str>,
    pub params: Option<&'a str>,
    pub param_json: Option<&'a str>,
    pub no_input: bool,
    pub is_interactive_terminal: bool,
}

impl<W, R, E, P> GenEndpointCliCommand<W, R, E, P>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
    P: InteractivePrompt + Logger,
{
    pub fn new(handler: GenEndpointCommandHandler<W, R, E>, prompt: P) -> Self {
        Self { handler, prompt }
    }

    pub fn execute(&self, request: GenEndpointRequest) -> Result<(), CliError> {
        self.prompt
            .intro("Generate Endpoint")
            .map_err(|e| CliError::internal(e.to_string()))?;

        let workspace_context = self.handler.get_workspace_context()?;
        let services = self.handler.extract_services(&workspace_context)?;

        if services.is_empty() {
            return Err(AddArtifactError::WorkspaceError(
                "No services found in workspace. Add a service first.".to_string(),
            )
            .into());
        }

        let selected_service =
            if (request.no_input || !request.is_interactive_terminal) && services.len() == 1 {
                services.into_iter().next().ok_or_else(|| {
                    AddArtifactError::WorkspaceError(
                        "Expected at least one service, but found none.".to_string(),
                    )
                })?
            } else {
                let options: Vec<SelectOption> = services
                    .iter()
                    .map(|s| SelectOption::new(s.name(), s.name()))
                    .collect();
                let selected = self
                    .prompt
                    .select("Select target service:", &options, Some(0))
                    .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

                services
                    .into_iter()
                    .find(|s| s.name() == selected.value())
                    .ok_or_else(|| {
                        AddArtifactError::WorkspaceError(
                            "No service found matching the selection".to_string(),
                        )
                    })?
            };

        let context = self.handler.load_template_context(
            "endpoint",
            &selected_service,
            &workspace_context,
        )?;

        // Validate required modules immediately after service selection, before any
        // further prompts. This gives the user an early, actionable error instead of
        // a late failure after they have answered all the interactive questions.
        self.handler
            .validate_required_modules(&context)
            .map_err(|e| CliError::silent(
                n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes::from_add_artifact_error(&e) as i32,
                e.to_string(),
            ))?;

        let existing_features = self
            .handler
            .list_features(&workspace_context, &selected_service)?;
        let feature = self.resolve_feature(&request, existing_features)?;
        let op_type_str = self.resolve_operation_type(&request)?;
        let op_type = HttpMethod::from_str(&op_type_str)
            .map_err(|e| AddArtifactError::InvalidParameter(e.to_string()))?;
        let mediator_sources = context.config().mediator_sources().to_vec();
        let (name, attach_to_mediator) = self.resolve_name(
            &request,
            &workspace_context,
            &selected_service,
            feature.as_deref(),
            &op_type_str,
            &mediator_sources,
        )?;

        let resolved_params = self.resolve_params(&request, context.config().inputs())?;

        let params_opt = if resolved_params.as_object().is_none_or(|o| o.is_empty()) {
            None
        } else {
            Some(resolved_params)
        };

        let command = GenEndpointCommand::new(
            name.to_string(),
            feature,
            op_type,
            params_opt,
            context,
            attach_to_mediator,
        )?;

        let spinner = self
            .prompt
            .spinner(&format!("Generating endpoint '{}'...", name))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        let res = self.handler.handle(command).map_err(|e| {
            spinner.error(&format!("Failed to generate endpoint: {}", e));
            e
        });

        if let Err(e) = res {
            return Err(CliError::silent(
                ExitCodes::from_add_artifact_error(&e) as i32,
                e.to_string(),
            ));
        }

        spinner.success(&format!("Endpoint '{}' generated successfully", name));

        self.prompt
            .outro(&format!("Successfully generated endpoint '{}'.", name))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        Ok(())
    }

    fn resolve_operation_type(
        &self,
        request: &GenEndpointRequest,
    ) -> Result<String, AddArtifactError> {
        if let Some(op_type_str) = request.operation_type {
            let normalized = match op_type_str.to_uppercase().as_str() {
                "GET" => "Get",
                "POST" => "Post",
                "PUT" => "Put",
                "DELETE" => "Delete",
                _ => {
                    return Err(AddArtifactError::InvalidParameter(format!(
                        "Invalid operation type '{}'. Must be GET, POST, PUT, or DELETE.",
                        op_type_str
                    )));
                }
            };
            return Ok(normalized.to_string());
        }

        if request.no_input || !request.is_interactive_terminal {
            return Err(AddArtifactError::InvalidIdentifier(
                "operation-type is required. Provide it as a positional argument (GET, POST, PUT, DELETE) or run interactively."
                    .to_string(),
            ));
        }

        let options = vec![
            SelectOption::new("GET", "Get"),
            SelectOption::new("POST", "Post"),
            SelectOption::new("PUT", "Put"),
            SelectOption::new("DELETE", "Delete"),
        ];

        let selected = self
            .prompt
            .select("Select HTTP operation type:", &options, Some(0))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        Ok(selected.value().to_string())
    }

    /// Resolves the endpoint name interactively and returns `(name, attach_to_mediator)`.
    ///
    /// `attach_to_mediator` is `true` only when the user confirmed they want to attach to an
    /// existing Command/Query.  When `false` the application handler will skip the mediator
    /// artifact existence check, allowing free-form endpoint names.
    fn resolve_name(
        &self,
        request: &GenEndpointRequest,
        workspace_context: &n_framework_nfw_core_application::features::template_management::services::artifact_generation_service::WorkspaceContext,
        selected_service: &n_framework_nfw_core_application::features::template_management::services::artifact_generation_service::ServiceInfo,
        feature: Option<&str>,
        op_type: &str,
        mediator_sources: &[String],
    ) -> Result<(String, bool), AddArtifactError> {
        if let Some(name) = request.name {
            if name.is_empty() {
                return Err(AddArtifactError::InvalidIdentifier(
                    "name cannot be empty".to_string(),
                ));
            }

            // Check if name corresponds to an existing command or query if mediator_sources are defined
            let is_query = op_type.eq_ignore_ascii_case("Get");
            let mut attach = false;

            if !mediator_sources.is_empty()
                && let Some(feature_name) = feature
            {
                let items = self
                    .handler
                    .get_mediator_items(workspace_context, selected_service, feature_name, is_query)
                    .unwrap_or_default();

                attach = items.contains(&name.to_string());
            }

            return Ok((name.to_owned(), attach));
        }

        if request.no_input || !request.is_interactive_terminal {
            return Err(AddArtifactError::InvalidIdentifier(
                "name is required. Provide it as a positional argument or run interactively."
                    .to_string(),
            ));
        }

        let has_mediator = !mediator_sources.is_empty()
            && self.handler.has_mediator_sources(
                workspace_context,
                selected_service,
                mediator_sources,
            );

        if has_mediator {
            let is_query = op_type.eq_ignore_ascii_case("GET");

            let items = if let Some(feature_name) = feature {
                self.handler
                    .get_mediator_items(workspace_context, selected_service, feature_name, is_query)
                    .unwrap_or_default()
            } else {
                Vec::new()
            };

            let attach = self
                .prompt
                .confirm(
                    &format!(
                        "Do you want to attach to an existing {}?",
                        if is_query { "Query" } else { "Command" }
                    ),
                    true,
                )
                .unwrap_or(false);

            if attach {
                let kind_label = if is_query { "query" } else { "command" };
                let kind_example = if is_query { "Get" } else { "Create" };

                let mut options: Vec<SelectOption> =
                    items.iter().map(|i| SelectOption::new(i, i)).collect();
                options.push(SelectOption::new(
                    format!("[Generate new {}]", kind_label),
                    "__create_new__",
                ));

                let selected = self
                    .prompt
                    .select(&format!("Select {}:", kind_label), &options, Some(0))
                    .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

                if selected.value() == "__create_new__" {
                    let new_name = self
                        .prompt
                        .text(
                            &format!(
                                "Enter new {} name (e.g. {}Product):",
                                kind_label, kind_example
                            ),
                            None,
                        )
                        .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

                    let template_context = self.handler.load_template_context(
                        kind_label,
                        selected_service,
                        workspace_context,
                    )?;

                    let mut map = serde_json::Map::new();
                    for input in template_context.config().inputs() {
                        let value = self.prompt_for_input(input)?;
                        map.insert(input.id().to_string(), value);
                    }
                    let params = if map.is_empty() {
                        None
                    } else {
                        Some(serde_json::Value::Object(map))
                    };

                    self.handler.generate_mediator_item(
                        &new_name,
                        feature,
                        &params,
                        &template_context,
                    )?;

                    let _ = self.prompt.log_success(&format!(
                        "Generated Mediator {} '{}'.",
                        if is_query { "Query" } else { "Command" },
                        new_name
                    ));

                    // The artifact was just created — attach_to_mediator=true so the handler
                    // validates it exists (which it now does).
                    return Ok((new_name, true));
                } else {
                    // User selected an existing artifact from the list — attach.
                    return Ok((selected.value().to_string(), true));
                }
            }
            // User said No → free-form name, skip mediator check.
        }

        let name = self
            .prompt
            .text(
                "Enter command or query name to map (e.g. GetProduct):",
                None,
            )
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;
        Ok((name, false))
    }

    fn resolve_feature(
        &self,
        request: &GenEndpointRequest,
        existing_features: Vec<String>,
    ) -> Result<Option<String>, AddArtifactError> {
        if let Some(feature) = request.feature {
            if feature.is_empty() {
                return Ok(None);
            }
            return Ok(Some(feature.to_owned()));
        }

        if request.no_input || !request.is_interactive_terminal {
            return Ok(None);
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
                .text("Enter feature name (e.g. Catalog):", None)
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
        request: &GenEndpointRequest,
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

impl GenEndpointCliCommand<(), (), (), n_framework_core_cli_cliclack::CliclackPromptService> {
    pub fn handle(
        command: &dyn n_framework_core_cli_abstractions::Command,
        context: &CliServiceCollection,
    ) -> Result<(), String> {
        use std::io::{self, IsTerminal};
        let is_interactive_terminal = io::stdin().is_terminal() && io::stdout().is_terminal();

        GenEndpointCliCommand::new(
            context.gen_endpoint_command_handler.clone(),
            n_framework_core_cli_cliclack::CliclackPromptService::new(),
        )
        .execute(GenEndpointRequest {
            operation_type: command.option("operation-type"),
            name: command.option("name"),
            feature: command.option("feature"),
            params: command.option("param"),
            param_json: command.option("param-json"),
            no_input: command.option("no-input").is_some(),
            is_interactive_terminal,
        })
        .map_err(|error| error.to_string())
    }
}
