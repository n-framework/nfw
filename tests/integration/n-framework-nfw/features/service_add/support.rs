#![allow(dead_code)]
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use n_framework_core_cli_abstractions::{InteractiveError, InteractivePrompt, Logger, LoggingError, SelectOption, Spinner};
use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command::AddServiceCommand;
use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command::AddServiceCommandResult;
use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command_handler::AddServiceCommandHandler;
use n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;
use n_framework_nfw_core_application::features::service_management::models::service_generator_resolution::ServiceGeneratorResolution;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_generator_prompt::ServiceGeneratorPrompt;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_generator_selector::{ServiceGeneratorSelectionContext, ServiceGeneratorSelector};
use n_framework_nfw_core_application::features::service_management::services::add_service_input_resolution_service::AddServiceInputResolutionService;
use n_framework_nfw_core_application::features::service_management::services::service_generator_provenance_service::ServiceGeneratorProvenanceService;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::versioning::version::Version;
use n_framework_nfw_infrastructure_filesystem::features::service_management::services::file_system_service_generator_renderer::FileSystemServiceGeneratorRenderer;
use n_framework_nfw_infrastructure_filesystem::features::service_management::services::service_generation_cleanup::ServiceGenerationCleanup;
use n_framework_nfw_infrastructure_yaml::features::workspace_management::services::workspace_metadata_writer::WorkspaceMetadataWriter;

#[derive(Debug, Clone)]
pub struct FixedWorkingDirectoryProvider {
    pub current_directory: PathBuf,
}

impl WorkingDirectoryProvider for FixedWorkingDirectoryProvider {
    fn current_dir(&self) -> Result<PathBuf, String> {
        Ok(self.current_directory.clone())
    }
}

#[derive(Debug, Clone)]
pub struct StaticGeneratorSelector {
    pub resolution: ServiceGeneratorResolution,
}

impl ServiceGeneratorSelector for StaticGeneratorSelector {
    fn resolve_service_generator(
        &self,
        _generator_identifier: &str,
        _context: ServiceGeneratorSelectionContext<'_>,
    ) -> Result<ServiceGeneratorResolution, AddServiceError> {
        Ok(self.resolution.clone())
    }

    fn list_service_generators(&self) -> Result<Vec<ServiceGeneratorResolution>, AddServiceError> {
        Ok(vec![self.resolution.clone()])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FirstGeneratorPrompt;

impl ServiceGeneratorPrompt for FirstGeneratorPrompt {
    fn select_generator(
        &self,
        generators: &[ServiceGeneratorResolution],
    ) -> Result<String, String> {
        generators
            .first()
            .map(ServiceGeneratorResolution::qualified_generator_id)
            .ok_or_else(|| "no generators available".to_owned())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FailingPromptService;

impl InteractivePrompt for FailingPromptService {
    fn is_interactive(&self) -> bool {
        false
    }

    fn text(&self, _message: &str, _default: Option<&str>) -> Result<String, InteractiveError> {
        Err(InteractiveError::internal(
            "text prompt is unavailable in test",
        ))
    }

    fn confirm(&self, _message: &str, _default: bool) -> Result<bool, InteractiveError> {
        Err(InteractiveError::internal(
            "confirm prompt is unavailable in test",
        ))
    }

    fn password(&self, _message: &str) -> Result<String, InteractiveError> {
        Err(InteractiveError::internal(
            "password prompt is unavailable in test",
        ))
    }

    fn select(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: Option<usize>,
    ) -> Result<SelectOption, InteractiveError> {
        Err(InteractiveError::internal(
            "select prompt is unavailable in test",
        ))
    }

    fn select_index(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: Option<usize>,
    ) -> Result<usize, InteractiveError> {
        Err(InteractiveError::internal(
            "select-index prompt is unavailable in test",
        ))
    }

    fn multiselect(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_indices: &[usize],
    ) -> Result<Vec<SelectOption>, InteractiveError> {
        Err(InteractiveError::internal(
            "multiselect prompt is unavailable in test",
        ))
    }
}

impl Logger for FailingPromptService {
    fn intro(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn outro(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_cancel(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_info(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_step(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_success(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_warning(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_error(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn spinner(&self, _message: &str) -> Result<Box<dyn Spinner>, LoggingError> {
        struct NoopSpinner;
        impl Spinner for NoopSpinner {
            fn stop(&self, _message: &str) {}
            fn success(&self, _message: &str) {}
            fn error(&self, _message: &str) {}
            fn cancel(&self, _message: &str) {}
            fn set_message(&self, _message: &str) {}
            fn is_finished(&self) -> bool {
                true
            }
        }
        Ok(Box::new(NoopSpinner))
    }
}

pub fn build_default_orchestration(
    workspace_root: &Path,
    generator_resolution: ServiceGeneratorResolution,
) -> AddServiceCommandHandler<
    FixedWorkingDirectoryProvider,
    StaticGeneratorSelector,
    FirstGeneratorPrompt,
    FailingPromptService,
    FileSystemServiceGeneratorRenderer,
    WorkspaceMetadataWriter,
> {
    let input_resolution = AddServiceInputResolutionService::new(
        StaticGeneratorSelector {
            resolution: generator_resolution,
        },
        FirstGeneratorPrompt,
        FailingPromptService,
    );

    AddServiceCommandHandler::new(
        FixedWorkingDirectoryProvider {
            current_directory: workspace_root.to_path_buf(),
        },
        input_resolution,
        FileSystemServiceGeneratorRenderer::new(ServiceGenerationCleanup::new()),
        ServiceGeneratorProvenanceService::new(WorkspaceMetadataWriter::new()),
    )
}

pub fn execute_non_interactive_add_service(
    orchestration_service: &AddServiceCommandHandler<
        FixedWorkingDirectoryProvider,
        StaticGeneratorSelector,
        FirstGeneratorPrompt,
        FailingPromptService,
        FileSystemServiceGeneratorRenderer,
        WorkspaceMetadataWriter,
    >,
    service_name: &str,
    generator_id: &str,
) -> Result<AddServiceCommandResult, AddServiceError> {
    let command = AddServiceCommand::new(
        Some(service_name.to_owned()),
        Some(generator_id.to_owned()),
        true,
        false,
    );

    orchestration_service.handle(&command)
}

pub fn create_generator_resolution(
    generator_root: &Path,
    source_name: &str,
    generator_id: &str,
) -> ServiceGeneratorResolution {
    ServiceGeneratorResolution {
        source_name: source_name.to_owned(),
        generator_name: "Dotnet Service".to_owned(),
        generator_id: generator_id.to_owned(),
        resolved_version: Version::from_str("1.0.0").expect("version should parse"),
        generator_type: "service".to_owned(),
        generator_cache_path: generator_root.to_path_buf(),
        description: "Service generator".to_owned(),
    }
}

#[allow(dead_code)]
pub fn create_workspace_root(test_name: &str) -> PathBuf {
    let root = create_sandbox_directory(test_name);
    fs::create_dir_all(root.join("src")).expect("workspace src directory should be created");
    fs::write(
        root.join("nfw.yaml"),
        "#    _  ______                                   __\n#   / |/ / __/______ ___ _  ___ _    _____  ____/ /__\n#  /    / _// __/ _ `/  ' \\/ -_) |/|/ / _ \\/ __/  '_/\n# /_/|_/_/ /_/  \\_,_/_/_/_/\\__/|__,__/\\___/_/ /_/\\_\\\n\n# yaml-language-server: $schema=https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json\n\nworkspace:\n  name: BillingPlatform\n  namespace: BillingPlatform\n",
    )
    .expect("workspace metadata should be written");
    root
}

pub fn create_service_generator(
    root: &Path,
    generator_name: &str,
    _generator_type: &str,
) -> PathBuf {
    let generator_root = root.join(generator_name);
    let service_root = generator_root.join("service");
    let content_root = service_root.join("content");

    fs::create_dir_all(content_root.join("Domain"))
        .expect("domain generator directory should be created");
    fs::create_dir_all(content_root.join("Application"))
        .expect("application generator directory should be created");
    fs::create_dir_all(content_root.join("Infrastructure"))
        .expect("infrastructure generator directory should be created");
    fs::create_dir_all(content_root.join("Api"))
        .expect("api generator directory should be created");

    // Root generator.yaml — identity metadata with generators pointing to service sub-generator
    fs::write(
         generator_root.join("nfw.generator.yaml"),
         format!("id: {generator_name}\nname: {generator_name}\ndescription: test\nversion: 1.0.0\ngenerators:\n  entity: service\n"),
     )
    .expect("root generator metadata should be written");

    // Service sub-generator — owns the rendering steps
    fs::write(
        service_root.join("nfw.workflow.yaml"),
        format!("id: {generator_name}/service\nname: {generator_name} service\ndescription: test\nversion: 1.0.0\nsteps:\n  - action: render_folder\n    source: 'content/'\n    destination: '.'\n"),
    )
    .expect("service generator.yaml should be written");

    fs::write(
        content_root.join("Domain/{{ServiceName}}.Domain.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>\n",
    )
    .expect("domain csproj should be written");

    fs::write(
        content_root.join("Application/{{ServiceName}}.Application.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"><ItemGroup><ProjectReference Include=\"../Domain/{{ServiceName}}.Domain.csproj\" /></ItemGroup></Project>\n",
    )
    .expect("application csproj should be written");

    fs::write(
        content_root.join("Infrastructure/{{ServiceName}}.Infrastructure.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"><ItemGroup><ProjectReference Include=\"../Application/{{ServiceName}}.Application.csproj\" /><ProjectReference Include=\"../Domain/{{ServiceName}}.Domain.csproj\" /></ItemGroup></Project>\n",
    )
    .expect("infrastructure csproj should be written");

    fs::write(
        content_root.join("Api/{{ServiceName}}.WebApi.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk.Web\"><ItemGroup><ProjectReference Include=\"../Application/{{ServiceName}}.Application.csproj\" /><ProjectReference Include=\"../Infrastructure/{{ServiceName}}.Infrastructure.csproj\" /></ItemGroup></Project>\n",
    )
    .expect("api csproj should be written");

    fs::write(
        content_root.join("Api/Program.cs"),
        "var app = builder.Build();\napp.Run();\n",
    )
    .expect("api program should be written");

    generator_root
}

pub fn create_sandbox_directory(test_name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("nfw-{test_name}-{unique}"));
    fs::create_dir_all(&path).expect("sandbox directory should be created");
    path
}

pub fn cleanup_sandbox_directory(path: &Path) {
    let _ = fs::remove_dir_all(path);
}
