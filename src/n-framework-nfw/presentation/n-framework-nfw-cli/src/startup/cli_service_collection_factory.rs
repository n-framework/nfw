use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command_handler::AddServiceCommandHandler;
use n_framework_nfw_core_application::features::check::commands::check::check_command_handler::CheckCommandHandler;
use n_framework_core_cli_cliclack::CliclackPromptService;
use n_framework_nfw_core_application::features::service_management::services::add_service_input_resolution_service::AddServiceInputResolutionService;
use n_framework_nfw_core_application::features::service_management::services::service_generator_provenance_service::ServiceGeneratorProvenanceService;
use n_framework_nfw_core_application::features::service_management::services::service_generator_selection_service::ServiceGeneratorSelectionService;
use n_framework_nfw_core_application::features::generator_management::commands::add_generator_source::add_generator_source_command_handler::AddGeneratorSourceCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::add_webapi::add_webapi_command_handler::AddWebApiCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::ensure_default_source::ensure_default_source_command_handler::EnsureDefaultSourceCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::refresh_generators::refresh_generators_command_handler::RefreshGeneratorsCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::remove_generator_source::remove_generator_source_command_handler::RemoveGeneratorSourceCommandHandler;
use n_framework_nfw_core_application::features::generator_management::queries::list_generators::list_generators_query_handler::ListGeneratorsQueryHandler;
use n_framework_nfw_core_application::features::generator_management::services::generator_catalog_parser::GeneratorCatalogParser;
use n_framework_nfw_core_application::features::generator_management::services::generator_catalog_source_resolver::GeneratorCatalogSourceResolver;
use n_framework_nfw_core_application::features::generator_management::services::generators_service::GeneratorsService;
use n_framework_nfw_core_application::features::generator_management::commands::add_mediator::add_mediator_command_handler::AddMediatorCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::add_persistence::add_persistence_command_handler::AddPersistenceCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::gen_mediator_command::gen_mediator_command_command_handler::GenMediatorCommandCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::gen_mediator_query::gen_mediator_query_command_handler::GenMediatorQueryCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::gen_endpoint::gen_endpoint_command_handler::GenEndpointCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::gen_repository::gen_repository_command_handler::GenRepositoryCommandHandler;
use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::file_system_generator_root_resolver::FileSystemGeneratorRootResolver;
use n_framework_nfw_core_application::features::workspace_management::commands::new_workspace::new_workspace_command_handler::NewWorkspaceCommandHandler;
use n_framework_nfw_core_application::features::workspace_management::services::generator_selection_for_new_service::GeneratorSelectionForNewService;
use n_framework_nfw_infrastructure_filesystem::features::cli::configuration::dirs_path_resolver::DirsPathResolver;
use n_framework_nfw_infrastructure_filesystem::features::cli::configuration::nfw_configuration_loader::NfwFileSystemConfigurationLoader;
use n_framework_nfw_infrastructure_filesystem::features::service_management::services::file_system_service_generator_renderer::FileSystemServiceGeneratorRenderer;
use n_framework_nfw_infrastructure_filesystem::features::service_management::services::service_generation_cleanup::ServiceGenerationCleanup;
use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::file_system_config_store::FileSystemWorkspaceArtifactWriter;
use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::local_generators_catalog_source::LocalGeneratorsCatalogSource;
use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::file_system_workspace_writer::FileSystemWorkspaceWriter;
use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::standard_working_directory_provider::StandardWorkingDirectoryProvider;
use n_framework_nfw_infrastructure_git::features::generator_management::services::cli_git_repository::CliGitRepository as InfrastructureCliGitRepository;
use n_framework_nfw_infrastructure_git::features::generator_management::services::git_generator_catalog_source::GitGeneratorCatalogSource;
use n_framework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use n_framework_nfw_infrastructure_yaml::features::generator_management::services::serde_yaml_parser::SerdeYamlParser;
use n_framework_nfw_infrastructure_yaml::features::workspace_management::services::workspace_metadata_writer::WorkspaceMetadataWriter;

use crate::runtime::interactive_service_generator_prompt::InteractiveServiceGeneratorPrompt;
use crate::startup::cli_validator::CliValidator;

pub use crate::startup::cli_service_types::CliServiceCollection;

#[derive(Debug, Default, Clone, Copy)]
pub struct CliServiceCollectionFactory;

impl CliServiceCollectionFactory {
    pub fn create() -> CliServiceCollection {
        let generators = GeneratorFeatureFactory::build();
        let workspace = WorkspaceFeatureFactory::build(generators.generators_service.clone());
        let service = ServiceFeatureFactory::build(generators.generators_service.clone());

        CliServiceCollection {
            prompt_service: CliclackPromptService::new(),
            new_workspace_command_handler: workspace.new_workspace_command_handler,
            add_service_command_handler: service.add_service_command_handler,
            check_command_handler: CheckCommandHandler::default(),
            list_generators_query_handler: generators.list_generators_query_handler,
            add_generator_source_command_handler: generators.add_generator_source_command_handler,
            remove_generator_source_command_handler: generators
                .remove_generator_source_command_handler,
            refresh_generators_command_handler: generators.refresh_generators_command_handler,
            ensure_default_source_command_handler: generators.ensure_default_source_command_handler,
            add_mediator_command_handler: AddMediatorCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                FileSystemGeneratorRootResolver::new(),
                n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine::new(),
            ),
            add_persistence_command_handler: AddPersistenceCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                FileSystemGeneratorRootResolver::new(),
                n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine::new(),
            ),
            add_webapi_command_handler: AddWebApiCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                FileSystemGeneratorRootResolver::new(),
                n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine::new(),
            ),
            gen_mediator_command_command_handler: GenMediatorCommandCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                FileSystemGeneratorRootResolver::new(),
                n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine::new(),
            ),
            gen_mediator_query_command_handler: GenMediatorQueryCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                FileSystemGeneratorRootResolver::new(),
                n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine::new(),
            ),
            gen_endpoint_command_handler: GenEndpointCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                FileSystemGeneratorRootResolver::new(),
                n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine::new(),
            ),
            gen_repository_command_handler: GenRepositoryCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                FileSystemGeneratorRootResolver::new(),
                n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine::new(),
            ),
            gen_entity_command_handler: n_framework_nfw_core_application::features::entity_generation::commands::add_entity_command_handler::AddEntityCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                FileSystemGeneratorRootResolver::new(),
                n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine::new(),
                n_framework_nfw_infrastructure_filesystem::features::entity_generation::adapters::file_system_entity_schema_store::FileSystemEntitySchemaStore::new(),
            ),
            generator_engine: n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine::new(),
        }
    }
}

#[allow(clippy::type_complexity)]
#[derive(Clone)]
struct GeneratorFeatureServices {
    generators_service: GeneratorsService<
        GitGeneratorCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
        LocalGeneratorsCatalogSource,
        SerdeYamlParser,
        CliValidator,
        SemverVersionComparator,
        FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
        InfrastructureCliGitRepository,
    >,
    list_generators_query_handler: ListGeneratorsQueryHandler<
        GeneratorsService<
            GitGeneratorCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
            LocalGeneratorsCatalogSource,
            SerdeYamlParser,
            CliValidator,
            SemverVersionComparator,
            FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
            InfrastructureCliGitRepository,
        >,
    >,
    add_generator_source_command_handler: AddGeneratorSourceCommandHandler<
        FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
        CliValidator,
        InfrastructureCliGitRepository,
    >,
    remove_generator_source_command_handler: RemoveGeneratorSourceCommandHandler<
        FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
        GitGeneratorCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
    >,
    refresh_generators_command_handler: RefreshGeneratorsCommandHandler<
        GeneratorsService<
            GitGeneratorCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
            LocalGeneratorsCatalogSource,
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

struct GeneratorFeatureFactory;

impl GeneratorFeatureFactory {
    fn build() -> GeneratorFeatureServices {
        let path_resolver = DirsPathResolver::new();
        let git_repository = InfrastructureCliGitRepository::new();
        let source_synchronizer = GitGeneratorCatalogSource::new(git_repository, path_resolver);
        let catalog_source = LocalGeneratorsCatalogSource::new();
        let catalog_parser = GeneratorCatalogParser::new(
            SerdeYamlParser::new(),
            CliValidator,
            SemverVersionComparator::new(),
        );
        let catalog_resolver = GeneratorCatalogSourceResolver::new(catalog_source, catalog_parser);
        let config_loader = NfwFileSystemConfigurationLoader::new(path_resolver);
        let config_store = FileSystemWorkspaceArtifactWriter::new(config_loader);

        let generators_service = GeneratorsService::new(
            source_synchronizer.clone(),
            catalog_resolver,
            config_store.clone(),
            CliValidator,
            git_repository,
        );

        GeneratorFeatureServices {
            list_generators_query_handler: ListGeneratorsQueryHandler::new(
                generators_service.clone(),
            ),
            add_generator_source_command_handler: AddGeneratorSourceCommandHandler::new(
                config_store.clone(),
                CliValidator,
                git_repository,
            ),
            remove_generator_source_command_handler: RemoveGeneratorSourceCommandHandler::new(
                config_store.clone(),
                source_synchronizer,
            ),
            refresh_generators_command_handler: RefreshGeneratorsCommandHandler::new(
                generators_service.clone(),
            ),
            ensure_default_source_command_handler: EnsureDefaultSourceCommandHandler::new(
                config_store,
            ),
            generators_service,
        }
    }
}

#[allow(clippy::type_complexity)]
#[derive(Clone)]
struct WorkspaceFeatureServices {
    new_workspace_command_handler: NewWorkspaceCommandHandler<
        CliclackPromptService,
        CliValidator,
        GeneratorsService<
            GitGeneratorCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
            LocalGeneratorsCatalogSource,
            SerdeYamlParser,
            CliValidator,
            SemverVersionComparator,
            FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
            InfrastructureCliGitRepository,
        >,
        FileSystemWorkspaceWriter<n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine>,
        StandardWorkingDirectoryProvider,
        CliclackPromptService,
    >,
}

struct WorkspaceFeatureFactory;

#[allow(clippy::type_complexity)]
impl WorkspaceFeatureFactory {
    fn build(
        generators_service: GeneratorsService<
            GitGeneratorCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
            LocalGeneratorsCatalogSource,
            SerdeYamlParser,
            CliValidator,
            SemverVersionComparator,
            FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
            InfrastructureCliGitRepository,
        >,
    ) -> WorkspaceFeatureServices {
        let generator_selection_for_new_service =
            GeneratorSelectionForNewService::new(generators_service, CliclackPromptService::new());

        WorkspaceFeatureServices {
            new_workspace_command_handler: NewWorkspaceCommandHandler::new(
                CliclackPromptService::new(),
                CliValidator,
                generator_selection_for_new_service,
                FileSystemWorkspaceWriter::new(n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine::new()),
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
        ServiceGeneratorSelectionService<
            GeneratorsService<
                GitGeneratorCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
                LocalGeneratorsCatalogSource,
                SerdeYamlParser,
                CliValidator,
                SemverVersionComparator,
                FileSystemWorkspaceArtifactWriter<
                    NfwFileSystemConfigurationLoader<DirsPathResolver>,
                >,
                InfrastructureCliGitRepository,
            >,
            FileSystemGeneratorRootResolver,
        >,
        InteractiveServiceGeneratorPrompt<CliclackPromptService>,
        CliclackPromptService,
        FileSystemServiceGeneratorRenderer,
        WorkspaceMetadataWriter,
    >,
}

struct ServiceFeatureFactory;

#[allow(clippy::type_complexity)]
impl ServiceFeatureFactory {
    fn build(
        generators_service: GeneratorsService<
            GitGeneratorCatalogSource<InfrastructureCliGitRepository, DirsPathResolver>,
            LocalGeneratorsCatalogSource,
            SerdeYamlParser,
            CliValidator,
            SemverVersionComparator,
            FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<DirsPathResolver>>,
            InfrastructureCliGitRepository,
        >,
    ) -> ServiceFeatureServices {
        let add_service_input_resolution_service = AddServiceInputResolutionService::new(
            ServiceGeneratorSelectionService::new(
                generators_service,
                FileSystemGeneratorRootResolver::new(),
            ),
            InteractiveServiceGeneratorPrompt::new(CliclackPromptService::new()),
            CliclackPromptService::new(),
        );

        ServiceFeatureServices {
            add_service_command_handler: AddServiceCommandHandler::new(
                StandardWorkingDirectoryProvider::new(),
                add_service_input_resolution_service,
                FileSystemServiceGeneratorRenderer::new(ServiceGenerationCleanup::new()),
                ServiceGeneratorProvenanceService::new(WorkspaceMetadataWriter::new()),
            ),
        }
    }
}
