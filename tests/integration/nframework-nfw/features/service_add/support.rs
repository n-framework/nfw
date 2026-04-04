use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use nframework_core_cli_abstraction::{PromptError, PromptService, SelectOption};
use nframework_nfw_application::features::service_management::commands::add_service::add_service_command::AddServiceCommand;
use nframework_nfw_application::features::service_management::models::errors::add_service_error::AddServiceError;
use nframework_nfw_application::features::service_management::models::service_template_resolution::ServiceTemplateResolution;
use nframework_nfw_application::features::service_management::services::abstraction::service_template_prompt::ServiceTemplatePrompt;
use nframework_nfw_application::features::service_management::services::abstraction::service_template_selector::ServiceTemplateSelector;
use nframework_nfw_application::features::service_management::services::add_service_input_resolution_service::AddServiceInputResolutionService;
use nframework_nfw_application::features::service_management::services::add_service_orchestration_service::AddServiceOrchestrationService;
use nframework_nfw_application::features::service_management::services::service_template_provenance_service::ServiceTemplateProvenanceService;
use nframework_nfw_application::features::workspace_management::services::abstraction::working_directory_provider::WorkingDirectoryProvider;
use nframework_nfw_domain::features::versioning::version::Version;
use nframework_nfw_infrastructure_filesystem::features::service_management::services::file_system_service_template_renderer::FileSystemServiceTemplateRenderer;
use nframework_nfw_infrastructure_filesystem::features::service_management::services::service_generation_cleanup::ServiceGenerationCleanup;
use nframework_nfw_infrastructure_yaml::features::workspace_management::services::workspace_metadata_writer::WorkspaceMetadataWriter;

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
pub struct StaticTemplateSelector {
    pub resolution: ServiceTemplateResolution,
}

impl ServiceTemplateSelector for StaticTemplateSelector {
    fn resolve_service_template(
        &self,
        _template_identifier: &str,
    ) -> Result<ServiceTemplateResolution, AddServiceError> {
        Ok(self.resolution.clone())
    }

    fn list_service_templates(&self) -> Result<Vec<ServiceTemplateResolution>, AddServiceError> {
        Ok(vec![self.resolution.clone()])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FirstTemplatePrompt;

impl ServiceTemplatePrompt for FirstTemplatePrompt {
    fn select_template(&self, templates: &[ServiceTemplateResolution]) -> Result<String, String> {
        templates
            .first()
            .map(ServiceTemplateResolution::qualified_template_id)
            .ok_or_else(|| "no templates available".to_owned())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FailingPromptService;

impl PromptService for FailingPromptService {
    fn is_interactive(&self) -> bool {
        false
    }

    fn text(&self, _message: &str, _default: Option<&str>) -> Result<String, PromptError> {
        Err(PromptError::internal("text prompt is unavailable in test"))
    }

    fn confirm(&self, _message: &str, _default: bool) -> Result<bool, PromptError> {
        Err(PromptError::internal(
            "confirm prompt is unavailable in test",
        ))
    }

    fn select(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: Option<usize>,
    ) -> Result<SelectOption, PromptError> {
        Err(PromptError::internal(
            "select prompt is unavailable in test",
        ))
    }

    fn select_index(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: Option<usize>,
    ) -> Result<usize, PromptError> {
        Err(PromptError::internal(
            "select-index prompt is unavailable in test",
        ))
    }
}

pub fn build_default_orchestration(
    workspace_root: &Path,
    template_resolution: ServiceTemplateResolution,
) -> AddServiceOrchestrationService<
    FixedWorkingDirectoryProvider,
    StaticTemplateSelector,
    FirstTemplatePrompt,
    FailingPromptService,
    FileSystemServiceTemplateRenderer,
    WorkspaceMetadataWriter,
> {
    let input_resolution = AddServiceInputResolutionService::new(
        StaticTemplateSelector {
            resolution: template_resolution,
        },
        FirstTemplatePrompt,
        FailingPromptService,
    );

    AddServiceOrchestrationService::new(
        FixedWorkingDirectoryProvider {
            current_directory: workspace_root.to_path_buf(),
        },
        input_resolution,
        FileSystemServiceTemplateRenderer::new(ServiceGenerationCleanup::new()),
        ServiceTemplateProvenanceService::new(WorkspaceMetadataWriter::new()),
    )
}

pub fn execute_non_interactive_add_service(
    orchestration_service: &AddServiceOrchestrationService<
        FixedWorkingDirectoryProvider,
        StaticTemplateSelector,
        FirstTemplatePrompt,
        FailingPromptService,
        FileSystemServiceTemplateRenderer,
        WorkspaceMetadataWriter,
    >,
    service_name: &str,
    template_id: &str,
) -> Result<
    nframework_nfw_application::features::service_management::commands::add_service::add_service_command::AddServiceCommandResult,
    AddServiceError,
>{
    let command = AddServiceCommand::new(
        Some(service_name.to_owned()),
        Some(template_id.to_owned()),
        true,
        false,
    );

    orchestration_service.execute(&command)
}

pub fn create_template_resolution(
    template_root: &Path,
    source_name: &str,
    template_id: &str,
) -> ServiceTemplateResolution {
    ServiceTemplateResolution {
        source_name: source_name.to_owned(),
        template_name: "Dotnet Service".to_owned(),
        template_id: template_id.to_owned(),
        resolved_version: Version::from_str("1.0.0").expect("version should parse"),
        template_type: "service".to_owned(),
        template_cache_path: template_root.to_path_buf(),
        description: "Service template".to_owned(),
    }
}

#[allow(dead_code)]
pub fn create_workspace_root(test_name: &str) -> PathBuf {
    let root = create_sandbox_directory(test_name);
    fs::create_dir_all(root.join("src")).expect("workspace src directory should be created");
    fs::write(
        root.join("nfw.yaml"),
        "#    _  ______                                   __\n#   / |/ / __/______ ___ _  ___ _    _____  ____/ /__\n#  /    / _// __/ _ `/  ' \\/ -_) |/|/ / _ \\/ __/  '_/\n# /_/|_/_/ /_/  \\_,_/_/_/_/\\__/|__,__/\\___/_/ /_/\\_\\\n\n# yaml-language-server: $schema=https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json\n$schema: https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json\n\nworkspace:\n  name: BillingPlatform\n  namespace: BillingPlatform\n",
    )
    .expect("workspace metadata should be written");
    root
}

pub fn create_service_template(
    root: &Path,
    template_name: &str,
    template_type: &str,
) -> PathBuf {
    let template_root = root.join(template_name);
    let content_root = template_root.join("content");
    fs::create_dir_all(content_root.join("Domain"))
        .expect("domain template directory should be created");
    fs::create_dir_all(content_root.join("Application"))
        .expect("application template directory should be created");
    fs::create_dir_all(content_root.join("Infrastructure"))
        .expect("infrastructure template directory should be created");
    fs::create_dir_all(content_root.join("Api")).expect("api template directory should be created");

    fs::write(
        template_root.join("template.yaml"),
        format!(
            "id: {template_name}\nname: {template_name}\ndescription: test\nversion: 1.0.0\ntype: {template_type}\n"
        ),
    )
    .expect("template metadata should be written");

    fs::write(
        content_root.join("Domain/__ServiceName__.Domain.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>\n",
    )
    .expect("domain csproj should be written");

    fs::write(
        content_root.join("Application/__ServiceName__.Application.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"><ItemGroup><ProjectReference Include=\"../Domain/__ServiceName__.Domain.csproj\" /></ItemGroup></Project>\n",
    )
    .expect("application csproj should be written");

    fs::write(
        content_root.join("Infrastructure/__ServiceName__.Infrastructure.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"><ItemGroup><ProjectReference Include=\"../Application/__ServiceName__.Application.csproj\" /><ProjectReference Include=\"../Domain/__ServiceName__.Domain.csproj\" /></ItemGroup></Project>\n",
    )
    .expect("infrastructure csproj should be written");

    fs::write(
        content_root.join("Api/__ServiceName__.WebApi.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk.Web\"><ItemGroup><ProjectReference Include=\"../Application/__ServiceName__.Application.csproj\" /><ProjectReference Include=\"../Infrastructure/__ServiceName__.Infrastructure.csproj\" /></ItemGroup></Project>\n",
    )
    .expect("api csproj should be written");

    fs::write(
        content_root.join("Api/Program.cs"),
        "var app = builder.Build();\napp.Run();\n",
    )
    .expect("api program should be written");

    template_root
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
