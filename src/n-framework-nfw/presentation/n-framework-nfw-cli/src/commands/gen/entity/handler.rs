use std::path::PathBuf;

use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};

use n_framework_nfw_core_application::features::entity_generation::abstractions::entity_schema_store::EntitySchemaStore;
use n_framework_nfw_core_application::features::entity_generation::commands::add_entity_command_handler::AddEntityCommandHandler;
use n_framework_nfw_core_application::features::entity_generation::services::property_syntax_parser::PropertySyntaxParser;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::entity_generation::entities::add_entity_command::{
    AddEntityCommand, EntityGenerationOptions, EntityType,
};
use n_framework_nfw_core_domain::features::entity_generation::errors::entity_generation_error::EntityGenerationError;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::general_type::GeneralType;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::service_info::ServiceInfo;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::workspace_context::WorkspaceContext;

use crate::cli_error::CliError;
use crate::startup::cli_service_collection_factory::CliServiceCollection;
use crate::startup::cli_service_types::{ArtServiceInfo, ArtWorkspaceContext};
use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;

pub struct GenEntityRequest<'a> {
    pub name: Option<&'a str>,
    pub feature: Option<&'a str>,
    pub properties: Option<&'a str>,
    pub id_type: Option<&'a str>,
    pub entity_type: Option<&'a str>,
    pub service: Option<&'a str>,
    pub from_schema: Option<&'a str>,
    pub schema_only: bool,
    pub no_input: bool,
    pub is_interactive_terminal: bool,
}

#[derive(Debug, Clone)]
pub struct GenEntityCliCommand<W, R, E, S, P> {
    handler: AddEntityCommandHandler<W, R, E, S>,
    prompt: P,
}

impl<W, R, E, S, P> GenEntityCliCommand<W, R, E, S, P>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
    S: EntitySchemaStore,
    P: InteractivePrompt + Logger,
{
    pub fn new(handler: AddEntityCommandHandler<W, R, E, S>, prompt: P) -> Self {
        Self { handler, prompt }
    }

    pub fn execute(&self, request: GenEntityRequest) -> Result<(), CliError> {
        self.prompt
            .intro("Generate Entity")
            .map_err(|e| CliError::internal(e.to_string()))?;

        let art_workspace = self
            .handler
            .get_workspace_context()
            .map_err(|e| CliError::internal(format!("failed to load workspace: {e}")))?;

        let art_services = self
            .handler
            .extract_services(&art_workspace)
            .map_err(|e| CliError::internal(format!("failed to extract services: {e}")))?;

        if art_services.is_empty() {
            return Err(CliError::new(
                ExitCodes::ValidationError as i32,
                "No services found in workspace. Add a service first with: nfw add service"
                    .to_owned(),
            ));
        }

        let selected_art_service = self.select_service(&request, &art_services)?;

        let modules =
            self.extract_modules_from_art_service(&art_workspace, &selected_art_service)?;
        let service_path = PathBuf::from(selected_art_service.path());

        let service = ServiceInfo::new(
            selected_art_service.name().to_owned(),
            service_path,
            modules,
        );

        let workspace = WorkspaceContext::new(
            art_workspace.workspace_root().clone(),
            vec![service.clone()],
        );

        let existing_features = self
            .handler
            .list_features(&art_workspace, &selected_art_service)
            .map_err(|e| CliError::internal(format!("failed to list features: {e}")))?;

        tracing::debug!("Existing features discovered: {:?}", existing_features);

        let feature = self.resolve_feature(&request, existing_features)?;

        let name = self.resolve_name(&request)?;
        let properties_input = self.resolve_properties(&request)?;
        let id_type = Self::resolve_id_type(request.id_type)?;
        let entity_type = Self::resolve_entity_type(request.entity_type)?;
        let from_schema = request.from_schema.map(PathBuf::from);

        let properties = if from_schema.is_none() {
            PropertySyntaxParser::parse(&properties_input)?
        } else {
            Vec::new()
        };

        let command = AddEntityCommand::try_new(
            name.clone(),
            properties,
            id_type,
            entity_type,
            EntityGenerationOptions::new(
                None,
                feature,
                request.schema_only,
                from_schema,
                request.no_input,
            ),
        )
        .map_err(|e| CliError::new(ExitCodes::ValidationError as i32, e.to_string()))?;

        let spinner = self
            .prompt
            .spinner(&format!("Generating entity '{name}'..."))
            .map_err(|e| CliError::internal(e.to_string()))?;

        match self.handler.handle(&command, &workspace, &service) {
            Ok((schema, schema_path)) => {
                spinner.success(&format!(
                    "Entity '{}' generated successfully",
                    schema.entity()
                ));

                self.prompt
                    .outro(&format!(
                        "Successfully generated entity '{}'. Schema: {}",
                        schema.entity(),
                        schema_path.display()
                    ))
                    .map_err(|e| CliError::internal(e.to_string()))?;

                if request.schema_only {
                    println!("\n◇  Schema file created at: {}", schema_path.display());
                    println!(
                        "│  You can now refine the schema and run without --schema-only to generate code.\n"
                    );
                }

                Ok(())
            }
            Err(e) => {
                spinner.error(&format!("Failed to generate entity: {e}"));
                Err(CliError::silent(
                    ExitCodes::ValidationError as i32,
                    e.to_string(),
                ))
            }
        }
    }

    fn select_service(
        &self,
        request: &GenEntityRequest,
        services: &[ArtServiceInfo],
    ) -> Result<ArtServiceInfo, CliError>
    where
        ArtServiceInfo: Clone,
    {
        if let Some(service_name) = request.service {
            return services
                .iter()
                .find(|s| s.name() == service_name)
                .cloned()
                .ok_or_else(|| {
                    CliError::new(
                        ExitCodes::ValidationError as i32,
                        format!("Service '{}' not found in workspace", service_name),
                    )
                });
        }

        if (request.no_input || !request.is_interactive_terminal) && services.len() == 1 {
            return Ok(services[0].clone());
        }

        if services.len() == 1 {
            return Ok(services[0].clone());
        }

        let options: Vec<SelectOption> = services
            .iter()
            .map(|s| SelectOption::new(s.name(), s.name()))
            .collect();

        let selected = self
            .prompt
            .select("Select target service:", &options, Some(0))
            .map_err(|e| CliError::internal(e.to_string()))?;

        services
            .iter()
            .find(|s| s.name() == selected.value())
            .cloned()
            .ok_or_else(|| CliError::internal("Service selection failed".to_owned()))
    }

    fn extract_modules_from_art_service(
        &self,
        workspace: &ArtWorkspaceContext,
        service: &ArtServiceInfo,
    ) -> Result<Vec<String>, CliError> {
        if let Some(services) = workspace.nfw_yaml().get("services")
            && let Some(map) = services.as_mapping()
        {
            for (name_val, details_val) in map {
                if let Some(name) = name_val.as_str()
                    && name == service.name()
                {
                    match details_val.get("modules") {
                        Some(modules_val) => {
                            let seq = modules_val.as_sequence().ok_or_else(|| {
                                CliError::internal(format!(
                                    "'modules' for service '{}' in nfw.yaml must be a sequence",
                                    name
                                ))
                            })?;

                            return Ok(seq
                                .iter()
                                .filter_map(|v| v.as_str())
                                .map(|s| s.to_string())
                                .collect());
                        }
                        None => {
                            return Ok(Vec::new());
                        }
                    }
                }
            }
        }

        Err(CliError::internal(format!(
            "Service '{}' details not found in nfw.yaml",
            service.name()
        )))
    }

    fn resolve_feature(
        &self,
        request: &GenEntityRequest,
        existing_features: Vec<String>,
    ) -> Result<String, CliError> {
        if let Some(feature) = request.feature {
            return Ok(feature.to_owned());
        }

        if request.no_input || !request.is_interactive_terminal {
            return Err(CliError::internal("Feature is required for entities. Provide it using --feature or run interactively.".to_owned()));
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
            .map_err(|e| CliError::internal(e.to_string()))?;

        if selected.value() == CREATE_NEW {
            let feature = self
                .prompt
                .text("Enter new feature name (e.g. Catalog):", None)
                .map_err(|e| CliError::internal(e.to_string()))?;

            if feature.trim().is_empty() {
                Err(CliError::internal(
                    "Feature name cannot be empty".to_owned(),
                ))
            } else {
                Ok(feature.trim().to_string())
            }
        } else {
            Ok(selected.value().to_string())
        }
    }

    fn resolve_name(&self, request: &GenEntityRequest) -> Result<String, EntityGenerationError> {
        if let Some(name) = request.name {
            return Ok(name.to_owned());
        }

        if request.no_input || !request.is_interactive_terminal {
            return Err(EntityGenerationError::InvalidEntityName {
                name: String::new(),
                reason:
                    "name is required. Provide it as a positional argument or run interactively."
                        .to_owned(),
            });
        }

        self.prompt
            .text("Enter entity name (e.g. Product, OrderItem):", None)
            .map_err(|e| EntityGenerationError::PromptError {
                reason: e.to_string(),
            })
    }

    fn resolve_properties(
        &self,
        request: &GenEntityRequest,
    ) -> Result<String, EntityGenerationError> {
        if let Some(props) = request.properties {
            return Ok(props.to_owned());
        }

        if request.from_schema.is_some() {
            return Ok(String::new());
        }

        if request.no_input || !request.is_interactive_terminal {
            return Err(EntityGenerationError::EmptyProperties);
        }

        let mut property_parts = Vec::new();

        loop {
            let name = self
                .prompt
                .text("Property name (e.g. Title):", None)
                .map_err(|e| EntityGenerationError::PromptError {
                    reason: e.to_string(),
                })?;

            if name.trim().is_empty() {
                break;
            }

            let type_options: Vec<SelectOption> = GeneralType::supported_cli_types()
                .iter()
                .map(|t| SelectOption::new(*t, *t))
                .collect();

            let selected_type = self
                .prompt
                .select(
                    &format!("Select type for '{}':", name),
                    &type_options,
                    Some(0),
                )
                .map_err(|e| EntityGenerationError::PromptError {
                    reason: e.to_string(),
                })?;

            let is_nullable = self
                .prompt
                .confirm(&format!("Is '{}' nullable?", name), false)
                .map_err(|e| EntityGenerationError::PromptError {
                    reason: e.to_string(),
                })?;

            let mut part = format!("{}:{}", name.trim(), selected_type.value());
            if is_nullable {
                part.push('?');
            }

            property_parts.push(part);

            let add_another = self
                .prompt
                .confirm("Add another property?", true)
                .map_err(|e| EntityGenerationError::PromptError {
                    reason: e.to_string(),
                })?;

            if !add_another {
                break;
            }
        }

        if property_parts.is_empty() {
            return Err(EntityGenerationError::EmptyProperties);
        }

        Ok(property_parts.join(","))
    }

    fn resolve_id_type(input: Option<&str>) -> Result<GeneralType, EntityGenerationError> {
        match input {
            None => Ok(GeneralType::Uuid),
            Some(i) if i.eq_ignore_ascii_case("guid") || i.eq_ignore_ascii_case("uuid") => {
                Ok(GeneralType::Uuid)
            }
            Some(i)
                if i.eq_ignore_ascii_case("int")
                    || i.eq_ignore_ascii_case("integer")
                    || i.eq_ignore_ascii_case("long") =>
            {
                Ok(GeneralType::Integer)
            }
            Some(i) if i.eq_ignore_ascii_case("string") => Ok(GeneralType::String),
            Some(other) => Err(EntityGenerationError::UnsupportedIdType {
                id_type: other.to_owned(),
            }),
        }
    }

    fn resolve_entity_type(input: Option<&str>) -> Result<EntityType, EntityGenerationError> {
        match input {
            None | Some("entity") => Ok(EntityType::Entity),
            Some("auditable-entity") | Some("auditable_entity") => Ok(EntityType::AuditableEntity),
            Some("soft-deletable-entity") | Some("soft_deletable_entity") => {
                Ok(EntityType::SoftDeletableEntity)
            }
            Some(other) => Err(EntityGenerationError::InvalidEntityName {
                name: other.to_owned(),
                reason: format!(
                    "unknown entity type '{}'. Supported: entity, auditable_entity, soft_deletable_entity",
                    other
                ),
            }),
        }
    }
}

impl GenEntityCliCommand<(), (), (), (), n_framework_core_cli_cliclack::CliclackPromptService> {
    pub fn handle(
        command: &dyn n_framework_core_cli_abstractions::Command,
        context: &CliServiceCollection,
    ) -> Result<(), String> {
        use std::io::{self, IsTerminal};
        let is_interactive_terminal = io::stdin().is_terminal() && io::stdout().is_terminal();

        GenEntityCliCommand::new(
            context.gen_entity_command_handler.clone(),
            n_framework_core_cli_cliclack::CliclackPromptService::new(),
        )
        .execute(GenEntityRequest {
            name: command.option("name"),
            feature: command.option("feature"),
            properties: command.option("properties"),
            id_type: command.option("id-type"),
            entity_type: command.option("entity-type"),
            service: command.option("service"),
            from_schema: command.option("from-schema"),
            schema_only: command.option("schema-only").is_some(),
            no_input: command.option("no-input").is_some(),
            is_interactive_terminal,
        })
        .map_err(|error| error.to_string())
    }
}
