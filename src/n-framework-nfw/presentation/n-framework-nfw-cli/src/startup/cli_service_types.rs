use n_framework_core_cli_inquire::InquirerPromptService;
use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command_handler::AddServiceCommandHandler;
use n_framework_nfw_core_application::features::template_management::commands::add_template_source::add_template_source_command_handler::AddTemplateSourceCommandHandler;
use n_framework_nfw_core_application::features::template_management::commands::ensure_default_source::ensure_default_source_command_handler::EnsureDefaultSourceCommandHandler;
use n_framework_nfw_core_application::features::template_management::commands::refresh_templates::refresh_templates_command_handler::RefreshTemplatesCommandHandler;
use n_framework_nfw_core_application::features::template_management::commands::remove_template_source::remove_template_source_command_handler::RemoveTemplateSourceCommandHandler;
use n_framework_nfw_core_application::features::template_management::queries::list_templates::list_templates_query_handler::ListTemplatesQueryHandler;
use n_framework_nfw_core_application::features::template_management::services::templates_service::TemplatesService;
use n_framework_nfw_core_application::features::workspace_management::commands::new_workspace::new_workspace_command_handler::NewWorkspaceCommandHandler;
use n_framework_nfw_infrastructure_filesystem::features::cli::configuration::dirs_path_resolver::DirsPathResolver;
use n_framework_nfw_infrastructure_filesystem::features::cli::configuration::nfw_configuration_loader::NfwFileSystemConfigurationLoader;
use n_framework_nfw_infrastructure_filesystem::features::service_management::services::file_system_service_template_renderer::FileSystemServiceTemplateRenderer;
use n_framework_nfw_infrastructure_filesystem::features::template_management::services::file_system_config_store::FileSystemWorkspaceArtifactWriter;
use n_framework_nfw_infrastructure_filesystem::features::template_management::services::local_templates_catalog_source::LocalTemplatesCatalogSource;
use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::file_system_workspace_writer::FileSystemWorkspaceWriter;
use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::standard_working_directory_provider::StandardWorkingDirectoryProvider;
use n_framework_nfw_infrastructure_git::features::template_management::services::cli_git_repository::CliGitRepository as InfrastructureCliGitRepository;
use n_framework_nfw_infrastructure_git::features::template_management::services::git_template_catalog_source::GitTemplateCatalogSource;
use n_framework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use n_framework_nfw_infrastructure_yaml::features::template_management::services::serde_yaml_parser::SerdeYamlParser;

use crate::startup::cli_validator::CliValidator;

use n_framework_nfw_infrastructure_filesystem::features::template_management::template_engine::FileSystemTemplateEngine;

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
pub type CliWorkspaceWriter = FileSystemWorkspaceWriter<FileSystemTemplateEngine>;
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
pub type CliAddServiceCommandHandler = AddServiceCommandHandler<
    CliWorkingDirectoryProvider,
    n_framework_nfw_core_application::features::service_management::services::service_template_selection_service::ServiceTemplateSelectionService<CliTemplatesService>,
    crate::runtime::interactive_service_template_prompt::InteractiveServiceTemplatePrompt<InquirerPromptService>,
    InquirerPromptService,
    FileSystemServiceTemplateRenderer,
    n_framework_nfw_infrastructure_yaml::features::workspace_management::services::workspace_metadata_writer::WorkspaceMetadataWriter,
>;
use n_framework_nfw_core_application::features::template_management::commands::generate::generate_command_handler::GenerateCommandHandler;
use n_framework_nfw_infrastructure_filesystem::features::template_management::services::file_system_template_root_resolver::FileSystemTemplateRootResolver;

pub type CliGenerateCommandHandler = GenerateCommandHandler<
    CliWorkingDirectoryProvider,
    FileSystemTemplateRootResolver,
    FileSystemTemplateEngine,
>;

use n_framework_nfw_core_application::features::check::commands::check::check_command_handler::CheckCommandHandler;
pub type CliCheckCommandHandler = CheckCommandHandler;

pub struct CliServiceCollection {
    pub new_workspace_command_handler: CliNewWorkspaceCommandHandler,
    pub add_service_command_handler: CliAddServiceCommandHandler,
    pub check_command_handler: CliCheckCommandHandler,
    pub list_templates_query_handler: CliListTemplatesQueryHandler,
    pub add_template_source_command_handler: CliAddTemplateSourceCommandHandler,
    pub remove_template_source_command_handler: CliRemoveTemplateSourceCommandHandler,
    pub refresh_templates_command_handler: CliRefreshTemplatesCommandHandler,
    pub ensure_default_source_command_handler: CliEnsureDefaultSourceCommandHandler,
    pub generate_command_handler: CliGenerateCommandHandler,
    pub template_engine: FileSystemTemplateEngine,
}

