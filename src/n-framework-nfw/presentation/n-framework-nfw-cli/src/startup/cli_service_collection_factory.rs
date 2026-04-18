use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command_handler::AddServiceCommandHandler;
use n_framework_nfw_core_application::features::check::commands::check::check_command_handler::CheckCommandHandler;
use n_framework_core_cli_inquire::InquirerPromptService;
use n_framework_nfw_core_application::features::service_management::services::add_service_input_resolution_service::AddServiceInputResolutionService;
use n_framework_nfw_core_application::features::service_management::services::service_template_provenance_service::ServiceTemplateProvenanceService;
use n_framework_nfw_core_application::features::service_management::services::service_template_selection_service::ServiceTemplateSelectionService;
use n_framework_nfw_core_application::features::template_management::commands::add_template_source::add_template_source_command_handler::AddTemplateSourceCommandHandler;
use n_framework_nfw_core_application::features::template_management::commands::ensure_default_source::ensure_default_source_command_handler::EnsureDefaultSourceCommandHandler;
use n_framework_nfw_core_application::features::template_management::commands::refresh_templates::refresh_templates_command_handler::RefreshTemplatesCommandHandler;
use n_framework_nfw_core_application::features::template_management::commands::remove_template_source::remove_template_source_command_handler::RemoveTemplateSourceCommandHandler;
use n_framework_nfw_core_application::features::template_management::queries::list_templates::list_templates_query_handler::ListTemplatesQueryHandler;
use n_framework_nfw_core_application::features::template_management::commands::generate::generate_command_handler::GenerateCommandHandler;
use n_framework_nfw_core_application::features::template_management::services::template_catalog_parser::TemplateCatalogParser;
use n_framework_nfw_core_application::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use n_framework_nfw_core_application::features::template_management::services::templates_service::TemplatesService;
use n_framework_nfw_infrastructure_filesystem::features::template_management::services::file_system_template_root_resolver::FileSystemTemplateRootResolver;
use n_framework_nfw_core_application::features::workspace_management::commands::new_workspace::new_workspace_command_handler::NewWorkspaceCommandHandler;
use n_framework_nfw_core_application::features::workspace_management::services::template_selection_for_new_service::TemplateSelectionForNewService;
use n_framework_nfw_infrastructure_filesystem::features::cli::configuration::dirs_path_resolver::DirsPathResolver;
use n_framework_nfw_infrastructure_filesystem::features::cli::configuration::nfw_configuration_loader::NfwFileSystemConfigurationLoader;
use n_framework_nfw_infrastructure_filesystem::features::service_management::services::file_system_service_template_renderer::FileSystemServiceTemplateRenderer;
use n_framework_nfw_infrastructure_filesystem::features::service_management::services::service_generation_cleanup::ServiceGenerationCleanup;
use n_framework_nfw_infrastructure_filesystem::features::template_management::services::file_system_config_store::FileSystemWorkspaceArtifactWriter;
use n_framework_nfw_infrastructure_filesystem::features::template_management::services::local_templates_catalog_source::LocalTemplatesCatalogSource;
use n_framework_nfw_infrastructure_filesystem::features::template_management::services::placeholder_detector::PlaceholderDetector;
use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::file_system_workspace_writer::FileSystemWorkspaceWriter;
use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::standard_working_directory_provider::StandardWorkingDirectoryProvider;
use n_framework_nfw_infrastructure_git::features::template_management::services::cli_git_repository::CliGitRepository as InfrastructureCliGitRepository;
use n_framework_nfw_infrastructure_git::features::template_management::services::git_template_catalog_source::GitTemplateCatalogSource;
use n_framework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use n_framework_nfw_infrastructure_yaml::features::template_management::services::serde_yaml_parser::SerdeYamlParser;
use n_framework_nfw_infrastructure_yaml::features::workspace_management::services::workspace_metadata_writer::WorkspaceMetadataWriter;

use crate::runtime::interactive_service_template_prompt::InteractiveServiceTemplatePrompt;
use crate::startup::cli_validator::CliValidator;

pub use crate::startup::cli_service_types::CliServiceCollection;

#[derive(Debug, Default, Clone, Copy)]
pub struct CliServiceCollectionFactory;

impl CliServiceCollectionFactory {
    pub fn create() -> CliServiceCollection {
        let templates = TemplateFeatureFactory::build();
        let workspace = WorkspaceFeatureFactory::build(templates.templates_service.clone());
        let service = ServiceFeatureFactory::build(templates.templates_service.clone());

        CliServiceCollection {
            new_workspace_command_handler: workspace.new_workspace_command_handler,
            add_service_command_handler: service.add_service_command_handler,
            check_command_handler: CheckCommandHandler::default(),
            list_templates_query_handler: templates.list_templates_query_handler,
            add_template_source_command_handler: templates.add_template_source_command_handler,
            remove_template_source_command_handler: templates
                .remove_template_source_command_handler,
            refresh_templates_command_handler: templates.refresh_templates_command_handler,
            ensure_default_source_command_handler: templates.ensure_default_source_command_handler,
            generate_command_handler: GenerateCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                FileSystemTemplateRootResolver::new(),
                n_framework_nfw_infrastructure_filesystem::features::template_management::template_engine::FileSystemTemplateEngine::new(),
            ),
            template_engine: n_framework_nfw_infrastructure_filesystem::features::template_management::template_engine::FileSystemTemplateEngine::new(),
        }
    }
}

#[allow(clippy::type_complexity)]
#[derive(Clone)]
struct TemplateFeatureServices {
    templates_service: TemplatesService<
        GitTemplateCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
        LocalTemplatesCatalogSource,
        SerdeYamlParser,
        CliValidator,
        SemverVersionComparator,
        FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
        InfrastructureCliGitRepository,
    >,
    list_templates_query_handler: ListTemplatesQueryHandler<
        TemplatesService<
            GitTemplateCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
            LocalTemplatesCatalogSource,
            SerdeYamlParser,
            CliValidator,
            SemverVersionComparator,
            FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
            InfrastructureCliGitRepository,
        >,
    >,
    add_template_source_command_handler: AddTemplateSourceCommandHandler<
        FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
        CliValidator,
        InfrastructureCliGitRepository,
    >,
    remove_template_source_command_handler: RemoveTemplateSourceCommandHandler<
        FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
        GitTemplateCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
    >,
    refresh_templates_command_handler: RefreshTemplatesCommandHandler<
        TemplatesService<
            GitTemplateCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
            LocalTemplatesCatalogSource,
            SerdeYamlParser,
            CliValidator,
            SemverVersionComparator,
            FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
            InfrastructureCliGitRepository,
        >,
    >,
    ensure_default_source_command_handler: EnsureDefaultSourceCommandHandler<
        FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
    >,
}

struct TemplateFeatureFactory;

impl TemplateFeatureFactory {
    fn build() -> TemplateFeatureServices {
        let path_resolver = DirsPathResolver::new();
        let git_repository = InfrastructureCliGitRepository::new();
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

        TemplateFeatureServices {
            list_templates_query_handler: ListTemplatesQueryHandler::new(templates_service.clone()),
            add_template_source_command_handler: AddTemplateSourceCommandHandler::new(
                config_store.clone(),
                CliValidator,
                git_repository,
            ),
            remove_template_source_command_handler: RemoveTemplateSourceCommandHandler::new(
                config_store.clone(),
                source_synchronizer,
            ),
            refresh_templates_command_handler: RefreshTemplatesCommandHandler::new(
                templates_service.clone(),
            ),
            ensure_default_source_command_handler: EnsureDefaultSourceCommandHandler::new(
                config_store,
            ),
            templates_service,
        }
    }
}

#[allow(clippy::type_complexity)]
#[derive(Clone)]
struct WorkspaceFeatureServices {
    new_workspace_command_handler: NewWorkspaceCommandHandler<
        InquirerPromptService,
        CliValidator,
        TemplatesService<
            GitTemplateCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
            LocalTemplatesCatalogSource,
            SerdeYamlParser,
            CliValidator,
            SemverVersionComparator,
            FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
            InfrastructureCliGitRepository,
        >,
        FileSystemWorkspaceWriter<n_framework_nfw_infrastructure_filesystem::features::template_management::template_engine::FileSystemTemplateEngine>,
        StandardWorkingDirectoryProvider,
        InquirerPromptService,
    >,
}

struct WorkspaceFeatureFactory;

#[allow(clippy::type_complexity)]
impl WorkspaceFeatureFactory {
    fn build(
        templates_service: TemplatesService<
            GitTemplateCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
            LocalTemplatesCatalogSource,
            SerdeYamlParser,
            CliValidator,
            SemverVersionComparator,
            FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
            InfrastructureCliGitRepository,
        >,
    ) -> WorkspaceFeatureServices {
        let template_selection_for_new_service =
            TemplateSelectionForNewService::new(templates_service, InquirerPromptService::new());

        WorkspaceFeatureServices {
            new_workspace_command_handler: NewWorkspaceCommandHandler::new(
                InquirerPromptService::new(),
                CliValidator,
                template_selection_for_new_service,
                FileSystemWorkspaceWriter::new(n_framework_nfw_infrastructure_filesystem::features::template_management::template_engine::FileSystemTemplateEngine::new()),
                StandardWorkingDirectoryProvider::new(),
            ),
        }
    }
}

#[allow(clippy::type_complexity)]
#[derive(Clone)]
struct ServiceFeatureServices {
    add_service_command_handler: AddServiceCommandHandler<
        StandardWorkingDirectoryProvider,
        ServiceTemplateSelectionService<
            TemplatesService<
                GitTemplateCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
                LocalTemplatesCatalogSource,
                SerdeYamlParser,
                CliValidator,
                SemverVersionComparator,
                FileSystemWorkspaceArtifactWriter<
                    NfwFileSystemConfigurationLoader<DirsPathResolver>,
                >,
                InfrastructureCliGitRepository,
            >,
        >,
        InteractiveServiceTemplatePrompt<InquirerPromptService>,
        InquirerPromptService,
        FileSystemServiceTemplateRenderer,
        WorkspaceMetadataWriter,
    >,
}

struct ServiceFeatureFactory;

#[allow(clippy::type_complexity)]
impl ServiceFeatureFactory {
    fn build(
        templates_service: TemplatesService<
            GitTemplateCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
            LocalTemplatesCatalogSource,
            SerdeYamlParser,
            CliValidator,
            SemverVersionComparator,
            FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
            InfrastructureCliGitRepository,
        >,
    ) -> ServiceFeatureServices {
        let add_service_input_resolution_service = AddServiceInputResolutionService::new(
            ServiceTemplateSelectionService::new(templates_service),
            InteractiveServiceTemplatePrompt::new(InquirerPromptService::new()),
            InquirerPromptService::new(),
        );

        ServiceFeatureServices {
            add_service_command_handler: AddServiceCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                add_service_input_resolution_service,
                FileSystemServiceTemplateRenderer::new(ServiceGenerationCleanup::new()),
                ServiceTemplateProvenanceService::new(WorkspaceMetadataWriter::new()),
            ),
        }
    }
}
