use n_framework_core_cli_cliclack::CliclackPromptService;
use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command_handler::AddServiceCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::add_generator_source::add_generator_source_command_handler::AddGeneratorSourceCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::ensure_default_source::ensure_default_source_command_handler::EnsureDefaultSourceCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::refresh_generators::refresh_generators_command_handler::RefreshGeneratorsCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::remove_generator_source::remove_generator_source_command_handler::RemoveGeneratorSourceCommandHandler;
use n_framework_nfw_core_application::features::generator_management::queries::list_generators::list_generators_query_handler::ListGeneratorsQueryHandler;
use n_framework_nfw_core_application::features::generator_management::services::generators_service::GeneratorsService;
use n_framework_nfw_core_application::features::workspace_management::commands::new_workspace::new_workspace_command_handler::NewWorkspaceCommandHandler;
use n_framework_nfw_infrastructure_filesystem::features::cli::configuration::dirs_path_resolver::DirsPathResolver;
use n_framework_nfw_infrastructure_filesystem::features::cli::configuration::nfw_configuration_loader::NfwFileSystemConfigurationLoader;
use n_framework_nfw_infrastructure_filesystem::features::service_management::services::file_system_service_generator_renderer::FileSystemServiceGeneratorRenderer;
use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::file_system_config_store::FileSystemWorkspaceArtifactWriter;
use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::local_generators_catalog_source::LocalGeneratorsCatalogSource;
use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::file_system_workspace_writer::FileSystemWorkspaceWriter;
use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::standard_working_directory_provider::StandardWorkingDirectoryProvider;
use n_framework_nfw_infrastructure_git::features::generator_management::services::cli_git_repository::CliGitRepository as InfrastructureCliGitRepository;
use n_framework_nfw_infrastructure_git::features::generator_management::services::git_generator_catalog_source::GitGeneratorCatalogSource;
use n_framework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use n_framework_nfw_infrastructure_yaml::features::generator_management::services::serde_yaml_parser::SerdeYamlParser;

#[allow(unused_imports)]
use crate::commands::r#gen::endpoint::handler::GenEndpointRequest;
use crate::runtime::interactive_service_generator_prompt::InteractiveServiceGeneratorPrompt;
use crate::startup::cli_validator::CliValidator;

use n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine;
use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::file_system_generator_root_resolver::FileSystemGeneratorRootResolver;

pub type CliPathResolver = DirsPathResolver;
pub type CliGitRepository = InfrastructureCliGitRepository;
pub type CliConfigurationLoader = NfwFileSystemConfigurationLoader<CliPathResolver>;
pub type CliConfigStore = FileSystemWorkspaceArtifactWriter<CliConfigurationLoader>;
pub type CliSourceSynchronizer = GitGeneratorCatalogSource<CliGitRepository, CliPathResolver>;
pub type CliCatalogSource = LocalGeneratorsCatalogSource;
pub type CliGeneratorsService = GeneratorsService<
    CliSourceSynchronizer,
    CliCatalogSource,
    SerdeYamlParser,
    CliValidator,
    SemverVersionComparator,
    CliConfigStore,
    CliGitRepository,
>;
pub type CliListGeneratorsQueryHandler = ListGeneratorsQueryHandler<CliGeneratorsService>;
pub type CliWorkspaceWriter = FileSystemWorkspaceWriter<FileSystemGeneratorEngine>;
pub type CliWorkingDirectoryProvider = StandardWorkingDirectoryProvider;
pub type CliNewWorkspaceCommandHandler = NewWorkspaceCommandHandler<
    CliclackPromptService,
    CliValidator,
    CliGeneratorsService,
    CliWorkspaceWriter,
    CliWorkingDirectoryProvider,
    CliclackPromptService,
>;
pub type CliAddGeneratorSourceCommandHandler =
    AddGeneratorSourceCommandHandler<CliConfigStore, CliValidator, CliGitRepository>;
pub type CliRemoveGeneratorSourceCommandHandler =
    RemoveGeneratorSourceCommandHandler<CliConfigStore, CliSourceSynchronizer>;
pub type CliRefreshGeneratorsCommandHandler = RefreshGeneratorsCommandHandler<CliGeneratorsService>;
pub type CliEnsureDefaultSourceCommandHandler = EnsureDefaultSourceCommandHandler<CliConfigStore>;
pub type CliAddServiceCommandHandler = AddServiceCommandHandler<
    CliWorkingDirectoryProvider,
    n_framework_nfw_core_application::features::service_management::services::service_generator_selection_service::ServiceGeneratorSelectionService<CliGeneratorsService, FileSystemGeneratorRootResolver>,
    InteractiveServiceGeneratorPrompt<CliclackPromptService>,
    CliclackPromptService,
    FileSystemServiceGeneratorRenderer,
    n_framework_nfw_infrastructure_yaml::features::workspace_management::services::workspace_metadata_writer::WorkspaceMetadataWriter,
>;

use n_framework_nfw_core_application::features::generator_management::commands::add_mediator::add_mediator_command_handler::AddMediatorCommandHandler;
pub type CliAddMediatorCommandHandler = AddMediatorCommandHandler<
    CliWorkingDirectoryProvider,
    FileSystemGeneratorRootResolver,
    FileSystemGeneratorEngine,
>;

use n_framework_nfw_core_application::features::generator_management::commands::add_persistence::add_persistence_command_handler::AddPersistenceCommandHandler;
pub type CliAddPersistenceCommandHandler = AddPersistenceCommandHandler<
    CliWorkingDirectoryProvider,
    FileSystemGeneratorRootResolver,
    FileSystemGeneratorEngine,
>;

use n_framework_nfw_core_application::features::generator_management::commands::add_webapi::add_webapi_command_handler::AddWebApiCommandHandler;
pub type CliAddWebApiCommandHandler = AddWebApiCommandHandler<
    CliWorkingDirectoryProvider,
    FileSystemGeneratorRootResolver,
    FileSystemGeneratorEngine,
>;

use n_framework_nfw_core_application::features::generator_management::commands::gen_mediator_command::gen_mediator_command_command_handler::GenMediatorCommandCommandHandler;
pub type CliGenMediatorCommandCommandHandler = GenMediatorCommandCommandHandler<
    CliWorkingDirectoryProvider,
    FileSystemGeneratorRootResolver,
    FileSystemGeneratorEngine,
>;

use n_framework_nfw_core_application::features::generator_management::commands::gen_mediator_query::gen_mediator_query_command_handler::GenMediatorQueryCommandHandler;
pub type CliGenMediatorQueryCommandHandler = GenMediatorQueryCommandHandler<
    CliWorkingDirectoryProvider,
    FileSystemGeneratorRootResolver,
    FileSystemGeneratorEngine,
>;

use n_framework_nfw_core_application::features::generator_management::commands::gen_repository::gen_repository_command_handler::GenRepositoryCommandHandler;
use n_framework_nfw_core_application::features::generator_management::commands::gen_endpoint::gen_endpoint_command_handler::GenEndpointCommandHandler;
pub type CliGenRepositoryCommandHandler = GenRepositoryCommandHandler<
    CliWorkingDirectoryProvider,
    FileSystemGeneratorRootResolver,
    FileSystemGeneratorEngine,
>;

pub type CliGenEndpointCommandHandler = GenEndpointCommandHandler<
    CliWorkingDirectoryProvider,
    FileSystemGeneratorRootResolver,
    FileSystemGeneratorEngine,
>;

use n_framework_nfw_core_application::features::check::commands::check::check_command_handler::CheckCommandHandler;
pub type CliCheckCommandHandler = CheckCommandHandler;

use n_framework_nfw_core_application::features::entity_generation::commands::add_entity_command_handler::AddEntityCommandHandler;
use n_framework_nfw_infrastructure_filesystem::features::entity_generation::adapters::file_system_entity_schema_store::FileSystemEntitySchemaStore;
pub type CliAddEntityCommandHandler = AddEntityCommandHandler<
    CliWorkingDirectoryProvider,
    FileSystemGeneratorRootResolver,
    FileSystemGeneratorEngine,
    FileSystemEntitySchemaStore,
>;

pub type ArtServiceInfo = n_framework_nfw_core_application::features::generator_management::services::artifact_generation_service::ServiceInfo;
pub type ArtWorkspaceContext = n_framework_nfw_core_application::features::generator_management::services::artifact_generation_service::WorkspaceContext;

pub struct CliServiceCollection {
    pub prompt_service: CliclackPromptService,
    pub new_workspace_command_handler: CliNewWorkspaceCommandHandler,
    pub add_service_command_handler: CliAddServiceCommandHandler,
    pub check_command_handler: CliCheckCommandHandler,
    pub list_generators_query_handler: CliListGeneratorsQueryHandler,
    pub add_generator_source_command_handler: CliAddGeneratorSourceCommandHandler,
    pub remove_generator_source_command_handler: CliRemoveGeneratorSourceCommandHandler,
    pub refresh_generators_command_handler: CliRefreshGeneratorsCommandHandler,
    pub ensure_default_source_command_handler: CliEnsureDefaultSourceCommandHandler,
    pub add_mediator_command_handler: CliAddMediatorCommandHandler,
    pub add_persistence_command_handler: CliAddPersistenceCommandHandler,
    pub add_webapi_command_handler: CliAddWebApiCommandHandler,
    pub gen_mediator_command_command_handler: CliGenMediatorCommandCommandHandler,
    pub gen_mediator_query_command_handler: CliGenMediatorQueryCommandHandler,
    pub gen_repository_command_handler: CliGenRepositoryCommandHandler,
    pub gen_endpoint_command_handler: CliGenEndpointCommandHandler,
    pub gen_entity_command_handler: CliAddEntityCommandHandler,
    pub generator_engine: FileSystemGeneratorEngine,
}
