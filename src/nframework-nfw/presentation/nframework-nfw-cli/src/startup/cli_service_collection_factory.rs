use std::path::Path;

use nframework_core_cli_inquire::InquirerPromptService;
use nframework_nfw_application::features::service_management::services::add_service_input_resolution_service::AddServiceInputResolutionService;
use nframework_nfw_application::features::service_management::services::add_service_orchestration_service::AddServiceOrchestrationService;
use nframework_nfw_application::features::service_management::services::service_template_provenance_service::ServiceTemplateProvenanceService;
use nframework_nfw_application::features::service_management::services::service_template_selection_service::ServiceTemplateSelectionService;
use nframework_nfw_application::features::template_management::commands::add_template_source::add_template_source_command_handler::AddTemplateSourceCommandHandler;
use nframework_nfw_application::features::template_management::commands::ensure_default_source::ensure_default_source_command_handler::EnsureDefaultSourceCommandHandler;
use nframework_nfw_application::features::template_management::commands::refresh_templates::refresh_templates_command_handler::RefreshTemplatesCommandHandler;
use nframework_nfw_application::features::template_management::commands::remove_template_source::remove_template_source_command_handler::RemoveTemplateSourceCommandHandler;
use nframework_nfw_application::features::template_management::queries::list_templates::list_templates_query_handler::ListTemplatesQueryHandler;
use nframework_nfw_application::features::template_management::services::abstraction::validator::Validator;
use nframework_nfw_application::features::template_management::services::template_catalog_parser::TemplateCatalogParser;
use nframework_nfw_application::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use nframework_nfw_application::features::template_management::services::templates_service::TemplatesService;
use nframework_nfw_application::features::workspace_management::commands::new_workspace::new_workspace_command_handler::NewWorkspaceCommandHandler;
use nframework_nfw_application::features::workspace_management::services::abstraction::workspace_name_validator::WorkspaceNameValidator;
use nframework_nfw_application::features::workspace_management::services::template_selection_for_new_service::TemplateSelectionForNewService;
use nframework_nfw_application::validation::is_kebab_case;
use nframework_nfw_infrastructure_filesystem::features::cli::configuration::dirs_path_resolver::DirsPathResolver;
use nframework_nfw_infrastructure_filesystem::features::cli::configuration::nfw_configuration_loader::NfwFileSystemConfigurationLoader;
use nframework_nfw_infrastructure_filesystem::features::service_management::services::file_system_service_template_renderer::FileSystemServiceTemplateRenderer;
use nframework_nfw_infrastructure_filesystem::features::service_management::services::service_generation_cleanup::ServiceGenerationCleanup;
use nframework_nfw_infrastructure_filesystem::features::template_management::services::file_system_config_store::FileSystemWorkspaceArtifactWriter;
use nframework_nfw_infrastructure_filesystem::features::template_management::services::local_templates_catalog_source::LocalTemplatesCatalogSource;
use nframework_nfw_infrastructure_filesystem::features::template_management::services::placeholder_detector::PlaceholderDetector;
use nframework_nfw_infrastructure_filesystem::features::workspace_management::services::file_system_workspace_writer::FileSystemWorkspaceWriter;
use nframework_nfw_infrastructure_filesystem::features::workspace_management::services::standard_working_directory_provider::StandardWorkingDirectoryProvider;
use nframework_nfw_infrastructure_git::features::template_management::services::cli_git_repository::CliGitRepository as InfrastructureCliGitRepository;
use nframework_nfw_infrastructure_git::features::template_management::services::git_template_catalog_source::GitTemplateCatalogSource;
use nframework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use nframework_nfw_infrastructure_yaml::features::template_management::services::serde_yaml_parser::SerdeYamlParser;
use nframework_nfw_infrastructure_yaml::features::workspace_management::services::workspace_metadata_writer::WorkspaceMetadataWriter;

use crate::runtime::interactive_service_template_prompt::InteractiveServiceTemplatePrompt;

pub type CliPathResolver = DirsPathResolver;
pub type CliGitRepository = InfrastructureCliGitRepository;
pub type CliConfigurationLoader = NfwFileSystemConfigurationLoader<CliPathResolver>;
pub type CliConfigStore = FileSystemWorkspaceArtifactWriter<CliConfigurationLoader>;
pub type CliSourceSynchronizer = GitTemplateCatalogSource<CliGitRepository, CliPathResolver>;
pub type CliCatalogSource = LocalTemplatesCatalogSource;
pub type CliTemplatesService = TemplatesService<
    CliSourceSynchronizer,
    CliCatalogSource,
    SerdeYamlParser,
    CliValidator,
    SemverVersionComparator,
    CliConfigStore,
    CliGitRepository,
>;
pub type CliListTemplatesQueryHandler = ListTemplatesQueryHandler<CliTemplatesService>;
pub type CliWorkspaceWriter = FileSystemWorkspaceWriter;
pub type CliWorkingDirectoryProvider = StandardWorkingDirectoryProvider;
pub type CliNewWorkspaceCommandHandler = NewWorkspaceCommandHandler<
    InquirerPromptService,
    CliValidator,
    CliTemplatesService,
    CliWorkspaceWriter,
    CliWorkingDirectoryProvider,
    InquirerPromptService,
>;
pub type CliAddTemplateSourceCommandHandler =
    AddTemplateSourceCommandHandler<CliConfigStore, CliValidator, CliGitRepository>;
pub type CliRemoveTemplateSourceCommandHandler =
    RemoveTemplateSourceCommandHandler<CliConfigStore, CliSourceSynchronizer>;
pub type CliRefreshTemplatesCommandHandler = RefreshTemplatesCommandHandler<CliTemplatesService>;
pub type CliEnsureDefaultSourceCommandHandler = EnsureDefaultSourceCommandHandler<CliConfigStore>;
pub type CliServiceTemplateSelector = ServiceTemplateSelectionService<CliTemplatesService>;
pub type CliServiceTemplatePrompt = InteractiveServiceTemplatePrompt<InquirerPromptService>;
pub type CliServiceTemplateRenderer = FileSystemServiceTemplateRenderer;
pub type CliServiceProvenanceStore = WorkspaceMetadataWriter;
pub type CliAddServiceOrchestrationService = AddServiceOrchestrationService<
    CliWorkingDirectoryProvider,
    CliServiceTemplateSelector,
    CliServiceTemplatePrompt,
    InquirerPromptService,
    CliServiceTemplateRenderer,
    CliServiceProvenanceStore,
>;
#[derive(Debug, Default, Clone, Copy)]
pub struct CliValidator;

impl Validator for CliValidator {
    fn is_kebab_case(&self, value: &str) -> bool {
        is_kebab_case(value)
    }

    fn is_git_url(&self, value: &str) -> bool {
        Self::has_valid_git_url_format(value)
    }
}

impl WorkspaceNameValidator for CliValidator {
    fn is_valid_workspace_name(&self, value: &str) -> bool {
        Self::has_valid_workspace_name_format(value)
    }
}

impl CliValidator {
    /// Checks if a string has a valid Git URL format (HTTP, HTTPS, SSH, or local path)
    fn has_valid_git_url_format(value: &str) -> bool {
        value.starts_with("http://")
            || value.starts_with("https://")
            || value.starts_with("git@")
            || Path::new(value).exists()
    }

    fn has_valid_workspace_name_format(value: &str) -> bool {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return false;
        }

        trimmed.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
    }
}

#[derive(Clone)]
pub struct CliServiceCollection {
    pub new_workspace_command_handler: CliNewWorkspaceCommandHandler,
    pub add_service_orchestration_service: CliAddServiceOrchestrationService,
    pub list_templates_query_handler: CliListTemplatesQueryHandler,
    pub add_template_source_command_handler: CliAddTemplateSourceCommandHandler,
    pub remove_template_source_command_handler: CliRemoveTemplateSourceCommandHandler,
    pub refresh_templates_command_handler: CliRefreshTemplatesCommandHandler,
    pub ensure_default_source_command_handler: CliEnsureDefaultSourceCommandHandler,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CliServiceCollectionFactory;

impl CliServiceCollectionFactory {
    pub fn create() -> CliServiceCollection {
        let path_resolver = DirsPathResolver::new();
        let git_repository = CliGitRepository::new();

        let source_synchronizer = GitTemplateCatalogSource::new(git_repository, path_resolver);
        let catalog_source = LocalTemplatesCatalogSource::new(PlaceholderDetector::new());
        let catalog_parser = TemplateCatalogParser::new(
            SerdeYamlParser::new(),
            CliValidator,
            SemverVersionComparator::new(),
        );
        let catalog_resolver = TemplateCatalogSourceResolver::new(catalog_source, catalog_parser);

        let config_loader = NfwFileSystemConfigurationLoader::new(path_resolver);
        let config_store = FileSystemWorkspaceArtifactWriter::new(config_loader);

        let templates_service = TemplatesService::new(
            source_synchronizer.clone(),
            catalog_resolver,
            config_store.clone(),
            CliValidator,
            git_repository,
        );
        let list_templates_query_handler =
            ListTemplatesQueryHandler::new(templates_service.clone());

        let template_selection_for_new_service = TemplateSelectionForNewService::new(
            templates_service.clone(),
            InquirerPromptService::new(),
        );

        let new_workspace_command_handler = NewWorkspaceCommandHandler::new(
            InquirerPromptService::new(),
            CliValidator,
            template_selection_for_new_service,
            FileSystemWorkspaceWriter::new(),
            StandardWorkingDirectoryProvider::new(),
        );

        let add_service_input_resolution_service = AddServiceInputResolutionService::new(
            ServiceTemplateSelectionService::new(templates_service.clone()),
            InteractiveServiceTemplatePrompt::new(InquirerPromptService::new()),
            InquirerPromptService::new(),
        );
        let add_service_orchestration_service = AddServiceOrchestrationService::new(
            StandardWorkingDirectoryProvider::new(),
            add_service_input_resolution_service,
            FileSystemServiceTemplateRenderer::new(ServiceGenerationCleanup::new()),
            ServiceTemplateProvenanceService::new(WorkspaceMetadataWriter::new()),
        );

        let add_template_source_command_handler = AddTemplateSourceCommandHandler::new(
            config_store.clone(),
            CliValidator,
            git_repository,
        );

        let remove_template_source_command_handler = RemoveTemplateSourceCommandHandler::new(
            config_store.clone(),
            source_synchronizer.clone(),
        );

        let refresh_templates_command_handler =
            RefreshTemplatesCommandHandler::new(templates_service.clone());

        let ensure_default_source_command_handler =
            EnsureDefaultSourceCommandHandler::new(config_store);

        CliServiceCollection {
            new_workspace_command_handler,
            add_service_orchestration_service,
            list_templates_query_handler,
            add_template_source_command_handler,
            remove_template_source_command_handler,
            refresh_templates_command_handler,
            ensure_default_source_command_handler,
        }
    }
}
