use crate::cli_error::CliError;
use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};
use n_framework_nfw_core_application::features::template_management::services::artifact_generation_service::ServiceInfo;
use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;

pub struct GenRepositoryRequest<'a> {
    pub name: Option<&'a str>,
    pub feature: Option<&'a str>,
    pub service: Option<&'a str>,
    pub no_input: bool,
    pub is_interactive_terminal: bool,
}

pub struct GenRepositoryCliCommand<P> {
    prompt: P,
}

impl<P> GenRepositoryCliCommand<P>
where
    P: InteractivePrompt + Logger,
{
    pub fn new(prompt: P) -> Self {
        Self { prompt }
    }

    pub fn execute(
        &self,
        request: GenRepositoryRequest,
        context: &crate::startup::cli_service_collection_factory::CliServiceCollection,
    ) -> Result<(), CliError> {
        self.prompt
            .intro("Generate Repository")
            .map_err(|e| CliError::internal(e.to_string()))?;

        let command = context.gen_repository_command_handler.clone();

        let workspace = command.get_workspace_context()?;
        let services = command.extract_services(&workspace)?;

        // 1. Prompt for Service Selection (if not provided and multiple exist)
        let service = self.resolve_service(&request, services)?;

        // Check if persistence module is configured before prompting further
        let mut has_persistence = false;
        if let Some(services_map) = workspace.nfw_yaml().get("services")
            && let Some(service_info) = services_map.get(service.name())
            && let Some(modules) = service_info.get("modules")
            && let Some(modules_seq) = modules.as_sequence()
        {
            has_persistence = modules_seq
                .iter()
                .any(|m| m.as_str() == Some("persistence"));
        }

        if !has_persistence {
            return Err(CliError::internal(format!(
                "Service '{}' does not have 'persistence' module configured. Run 'nfw add persistence' first.",
                service.name()
            )));
        }

        // Get existing features to populate our prompt options
        let existing_features = command
            .list_features(&workspace, &service)
            .unwrap_or_default();

        // 2. Prompt for Feature Selection (if not provided, preventing creation of new features)
        let feature_name = self.resolve_feature(&request, existing_features, &service)?;

        let mut available_entities = Vec::new();
        let feature_entities_dir = std::path::Path::new(service.path())
            .join("specs/features")
            .join(&feature_name)
            .join("entities");

        if feature_entities_dir.exists()
            && let Ok(entities) = std::fs::read_dir(feature_entities_dir)
        {
            for entity in entities.flatten() {
                if let Some(file_name) = entity.file_name().to_str()
                    && (file_name.ends_with(".yaml") || file_name.ends_with(".yml"))
                {
                    let entity_name = file_name
                        .trim_end_matches(".yaml")
                        .trim_end_matches(".yml")
                        .to_string();
                    if !available_entities.contains(&entity_name) {
                        available_entities.push(entity_name);
                    }
                }
            }
        }
        available_entities.sort();

        // 3. Prompt for Entity Name Selection (if not provided)
        let entity_name = self.resolve_name(&request, available_entities)?;

        self.prompt
            .outro(&format!(
                "Ready to scaffold repository for entity '{}' in feature '{}'.",
                entity_name, feature_name
            ))
            .map_err(|e| CliError::internal(e.to_string()))?;

        let template_context = command.load_template_context(workspace, &service, "repository")?;

        let gen_cmd = n_framework_nfw_core_application::features::template_management::commands::gen_repository::gen_repository_command::GenRepositoryCommand::new(
            entity_name,
            Some(feature_name),
            template_context,
        );

        command
            .handle(&gen_cmd)
            .map_err(|e| CliError::new(ExitCodes::ValidationError as i32, e.to_string()))?;

        Ok(())
    }

    fn resolve_service(
        &self,
        request: &GenRepositoryRequest,
        services: Vec<ServiceInfo>,
    ) -> Result<ServiceInfo, CliError> {
        if let Some(s) = request.service {
            return services
                .into_iter()
                .find(|x| x.name() == s)
                .ok_or_else(|| CliError::internal(format!("Service '{}' not found", s)));
        }

        if services.len() == 1 {
            let service = services.into_iter().next().unwrap();
            let _ = self
                .prompt
                .log_step(&format!("Auto-selected service: {}", service.name()));
            return Ok(service);
        }

        if request.no_input || !request.is_interactive_terminal {
            return Err(CliError::internal(
                "Multiple services found. Provide one using --service or run interactively."
                    .to_owned(),
            ));
        }

        let options: Vec<SelectOption> = services
            .iter()
            .map(|s| SelectOption::new(s.name(), s.name()))
            .collect();

        let selected = self
            .prompt
            .select("Select service:", &options, Some(0))
            .map_err(|e| CliError::internal(e.to_string()))?;

        let selected_value = selected.value().to_string();

        services
            .into_iter()
            .find(|x| x.name() == selected_value)
            .ok_or_else(|| CliError::internal("Service not found".to_string()))
    }

    fn resolve_feature(
        &self,
        request: &GenRepositoryRequest,
        existing_features: Vec<String>,
        service: &ServiceInfo,
    ) -> Result<String, CliError> {
        if let Some(feature) = request.feature {
            return Ok(feature.to_owned());
        }

        if request.no_input || !request.is_interactive_terminal {
            return Err(CliError::internal(
                "Feature is required. Provide it using --feature or run interactively.".to_owned(),
            ));
        }

        if existing_features.is_empty() {
            let error_msg = format!(
                "Create entity first. Run 'nfw gen entity --service {}'.",
                service.name()
            );
            let _ = self.prompt.log_error(&error_msg);
            return Err(CliError::silent(
                ExitCodes::ValidationError as i32,
                error_msg,
            ));
        }

        if existing_features.len() == 1 {
            let feature = existing_features[0].clone();
            let _ = self
                .prompt
                .log_step(&format!("Auto-selected feature: {}", feature));
            return Ok(feature);
        }

        let options: Vec<SelectOption> = existing_features
            .iter()
            .map(|f| SelectOption::new(f, f))
            .collect();

        let selected = self
            .prompt
            .select("Select feature:", &options, Some(0))
            .map_err(|e| CliError::internal(e.to_string()))?;

        Ok(selected.value().to_string())
    }

    fn resolve_name(
        &self,
        request: &GenRepositoryRequest,
        available_entities: Vec<String>,
    ) -> Result<String, CliError> {
        if let Some(name) = request.name {
            return Ok(name.to_owned());
        }

        if request.no_input || !request.is_interactive_terminal {
            return Err(CliError::new(
                ExitCodes::ValidationError as i32,
                "Entity name is required. Provide it as a positional argument or run interactively.".to_owned(),
            ));
        }

        if available_entities.is_empty() {
            return self
                .prompt
                .text("Enter entity name (e.g. Product, OrderItem):", None)
                .map_err(|e| CliError::internal(e.to_string()));
        }

        let mut options: Vec<SelectOption> = available_entities
            .iter()
            .map(|e| SelectOption::new(e, e))
            .collect();

        const MANUAL_ENTRY: &str = "__manual_entry__";
        options.push(SelectOption::new("[Type manually]", MANUAL_ENTRY));

        let selected = self
            .prompt
            .select("Select entity:", &options, Some(0))
            .map_err(|e| CliError::internal(e.to_string()))?;

        if selected.value() == MANUAL_ENTRY {
            let entity = self
                .prompt
                .text("Enter entity name (e.g. Product, OrderItem):", None)
                .map_err(|e| CliError::internal(e.to_string()))?;

            if entity.trim().is_empty() {
                Err(CliError::internal("Entity name cannot be empty".to_owned()))
            } else {
                Ok(entity.trim().to_string())
            }
        } else {
            Ok(selected.value().to_string())
        }
    }
}

impl GenRepositoryCliCommand<n_framework_core_cli_cliclack::CliclackPromptService> {
    pub fn handle(
        command: &dyn n_framework_core_cli_abstractions::Command,
        context: &crate::startup::cli_service_collection_factory::CliServiceCollection,
    ) -> Result<(), String> {
        use std::io::{self, IsTerminal};
        let is_interactive_terminal = io::stdin().is_terminal() && io::stdout().is_terminal();

        GenRepositoryCliCommand::new(n_framework_core_cli_cliclack::CliclackPromptService::new())
            .execute(
                GenRepositoryRequest {
                    name: command.option("name"),
                    feature: command.option("feature"),
                    service: command.option("service"),
                    no_input: command.option("no-input").is_some(),
                    is_interactive_terminal,
                },
                context,
            )
            .map_err(|error| error.to_string())
    }
}
