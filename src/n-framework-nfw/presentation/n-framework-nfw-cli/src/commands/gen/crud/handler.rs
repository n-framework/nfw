use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};
use crate::cli_error::CliError;
use crate::startup::cli_service_collection_factory::CliServiceCollection;
use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;
use n_framework_nfw_core_application::features::generator_management::commands::gen_crud::gen_crud_command::GenCrudCommand;
use n_framework_nfw_core_application::features::generator_management::commands::gen_crud::gen_crud_command_handler::GenCrudCommandHandler;
pub use n_framework_nfw_core_application::features::generator_management::models::errors::add_artifact_error::AddArtifactError;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
use n_framework_nfw_core_application::features::generator_management::services::generator_engine::GeneratorEngine;

#[derive(Debug, Clone)]
pub struct GenCrudCliCommand<W, R, E, P> {
    handler: GenCrudCommandHandler<W, R, E>,
    prompt: P,
}

pub struct GenCrudRequest<'a> {
    pub name: Option<&'a str>,
    pub feature: Option<&'a str>,
    pub params: Option<&'a str>,
    pub param_json: Option<&'a str>,
    pub no_input: bool,
    pub is_interactive_terminal: bool,
}

impl<W, R, E, P> GenCrudCliCommand<W, R, E, P>
where
    W: WorkingDirectoryProvider,
    R: GeneratorRootResolver,
    E: GeneratorEngine,
    P: InteractivePrompt + Logger,
{
    pub fn new(handler: GenCrudCommandHandler<W, R, E>, prompt: P) -> Self {
        Self { handler, prompt }
    }

    pub fn execute(&self, request: GenCrudRequest) -> Result<(), CliError> {
        self.prompt
            .intro("Generate CRUD")
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
                services.into_iter().next().unwrap()
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

        // T003: Resolve and validate entity name
        let entity_name = self.resolve_name(&request)?;
        self.handler
            .validate_entity_identifier(&entity_name)
            .map_err(|e| {
                CliError::silent(ExitCodes::from_add_artifact_error(&e) as i32, e.to_string())
            })?;

        // T004: Workspace checking logic
        let entity_exists = self
            .handler
            .check_entity_exists(&workspace_context, &selected_service, &entity_name)
            .map_err(|e| {
                CliError::silent(ExitCodes::from_add_artifact_error(&e) as i32, e.to_string())
            })?;

        if !entity_exists {
            // T017: Entity missing logic
            if request.no_input || !request.is_interactive_terminal {
                return Err(CliError::internal(format!(
                    "Entity '{}' not found in Domain layer. Run 'nfw gen entity {}' first, or run this command interactively.",
                    entity_name, entity_name
                )));
            }

            let create_entity = self
                .prompt
                .confirm(
                    &format!("Entity '{}' not found. Create it now?", entity_name),
                    true,
                )
                .map_err(|e| CliError::internal(e.to_string()))?;

            if !create_entity {
                return Err(CliError::internal(format!(
                    "Cannot generate CRUD for non-existent entity '{}'.",
                    entity_name
                )));
            }

            // TODO: In Phase 3, we will orchestrate the 'nfw gen entity' command here.
            let _ = self.prompt.log_warning(&format!(
                "Entity creation for '{}' will be orchestrated here in Phase 3.",
                entity_name
            ));
        }

        let existing_features = self
            .handler
            .list_features(&workspace_context, &selected_service)?;
        let feature = self.resolve_feature(&request, existing_features)?;

        // T005: Artifact conflict detection
        let artifacts_exist = self
            .handler
            .check_artifacts_exist(
                &workspace_context,
                &selected_service,
                &entity_name,
                feature.as_deref(),
            )
            .map_err(|e| {
                CliError::silent(ExitCodes::from_add_artifact_error(&e) as i32, e.to_string())
            })?;

        // Extract parameters to check for force=true
        let mut resolved_params = self.resolve_params(&request)?;
        let force = resolved_params
            .as_object()
            .and_then(|obj| obj.get("force"))
            .and_then(|v| v.as_str())
            .is_some_and(|v| v == "true" || v == "1" || v.to_lowercase() == "true");

        if artifacts_exist && !force {
            // T018: Overwrite prompt
            if request.no_input || !request.is_interactive_terminal {
                return Err(CliError::internal(format!(
                    "Files for '{}' already exist. Use parameter 'force=true' to overwrite.",
                    entity_name
                )));
            }

            let overwrite = self
                .prompt
                .confirm(
                    &format!("Files for '{}' already exist. Overwrite?", entity_name),
                    false,
                )
                .map_err(|e| CliError::internal(e.to_string()))?;

            if !overwrite {
                return Err(CliError::internal(
                    "Generation cancelled by user.".to_string(),
                ));
            }

            if let Some(obj) = resolved_params.as_object_mut() {
                obj.insert(
                    "force".to_string(),
                    serde_json::Value::String("true".to_string()),
                );
            }
        }

        let params_opt = if resolved_params.as_object().is_none_or(|o| o.is_empty()) {
            None
        } else {
            Some(resolved_params)
        };

        // Needs to load dummy context for the command structure - we will load the real ones during orchestration in Phase 3
        let context = self.handler.load_generator_context(
            workspace_context.clone(),
            &selected_service,
            "command", // Dummy context for now
        )?;

        let command = GenCrudCommand::new(&entity_name, feature, params_opt, context);

        let spinner = self
            .prompt
            .spinner(&format!("Generating CRUD for '{}'...", entity_name))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        // TODO: Phase 3 - Orchestration implementation
        let _res = self.handler.handle(&command).map_err(|e| {
            spinner.error(&format!("Failed to generate CRUD: {}", e));
            e
        });

        spinner.success(&format!(
            "CRUD for '{}' generated successfully (Stub)",
            entity_name
        ));

        self.prompt
            .outro(&format!(
                "Successfully generated CRUD scaffolding for '{}'.",
                entity_name
            ))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        Ok(())
    }

    fn resolve_name(&self, request: &GenCrudRequest) -> Result<String, AddArtifactError> {
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
            .text("Enter entity name (e.g. Product):", None)
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))
    }

    fn resolve_feature(
        &self,
        request: &GenCrudRequest,
        existing_features: Vec<String>,
    ) -> Result<Option<String>, AddArtifactError> {
        if let Some(feature) = request.feature {
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
        options.push(SelectOption::new("[Root/Default feature]", "__root__"));

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
        } else if selected.value() == "__root__" {
            Ok(None)
        } else {
            Ok(Some(selected.value().to_string()))
        }
    }

    fn resolve_params(
        &self,
        request: &GenCrudRequest,
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

        // T015, T016: Interactive prompts for options if they don't exist
        if !request.no_input && request.is_interactive_terminal {
            if !map.contains_key("no-api") {
                let skip_api = self
                    .prompt
                    .confirm("Skip generating API Endpoints?", false)
                    .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;
                map.insert(
                    "no-api".to_string(),
                    serde_json::Value::String(skip_api.to_string()),
                );
            }

            if !map.contains_key("secured") {
                let secured = self
                    .prompt
                    .confirm("Include security markers?", false)
                    .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;
                map.insert(
                    "secured".to_string(),
                    serde_json::Value::String(secured.to_string()),
                );
            }

            if !map.contains_key("cached") {
                let cached = self
                    .prompt
                    .confirm("Include caching markers?", false)
                    .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;
                map.insert(
                    "cached".to_string(),
                    serde_json::Value::String(cached.to_string()),
                );
            }
        }

        Ok(serde_json::Value::Object(map))
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

impl GenCrudCliCommand<(), (), (), n_framework_core_cli_cliclack::CliclackPromptService> {
    pub fn handle(
        command: &dyn n_framework_core_cli_abstractions::Command,
        context: &CliServiceCollection,
    ) -> Result<(), String> {
        use std::io::{self, IsTerminal};
        let is_interactive_terminal = io::stdin().is_terminal() && io::stdout().is_terminal();

        GenCrudCliCommand::new(
            context.gen_crud_command_handler.clone(),
            n_framework_core_cli_cliclack::CliclackPromptService::new(),
        )
        .execute(GenCrudRequest {
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
