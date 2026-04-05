#[path = "support.rs"]
mod support;

use std::fs;
use std::path::Path;

use nframework_nfw_core_application::features::service_management::commands::add_service::add_service_command::AddServiceCommand;
use nframework_nfw_core_application::features::service_management::commands::add_service::add_service_command_handler::AddServiceCommandHandler;
use nframework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;
use nframework_nfw_core_application::features::service_management::models::service_generation_plan::ServiceGenerationPlan;
use nframework_nfw_core_application::features::service_management::services::abstractions::service_template_renderer::ServiceTemplateRenderer;
use nframework_nfw_core_application::features::service_management::services::add_service_input_resolution_service::AddServiceInputResolutionService;
use nframework_nfw_core_application::features::service_management::services::service_template_provenance_service::ServiceTemplateProvenanceService;
use nframework_nfw_infrastructure_yaml::features::workspace_management::services::workspace_metadata_writer::WorkspaceMetadataWriter;

#[derive(Debug, Clone)]
struct InterruptingRenderer;

impl ServiceTemplateRenderer for InterruptingRenderer {
    fn render_service(&self, plan: &ServiceGenerationPlan) -> Result<(), AddServiceError> {
        fs::create_dir_all(&plan.output_root).expect("partial output should be created");
        fs::write(plan.output_root.join("partial.txt"), "partial")
            .expect("partial output should be written");
        Err(AddServiceError::Interrupted)
    }

    fn cleanup_partial_output(&self, output_root: &Path) -> Result<(), AddServiceError> {
        if output_root.exists() {
            fs::remove_dir_all(output_root).map_err(|error| {
                AddServiceError::CleanupFailed(format!(
                    "failed to remove '{}': {error}",
                    output_root.display()
                ))
            })?;
        }
        Ok(())
    }
}

#[test]
fn removes_partial_output_when_generation_fails_after_write_start() {
    let workspace_root = support::create_workspace_root("service-cleanup-render");
    let broken_template_root = workspace_root.join("broken-template");
    fs::create_dir_all(&broken_template_root).expect("broken template root should be created");
    fs::write(
        broken_template_root.join("template.yaml"),
        "id: dotnet-service\nname: Dotnet Service\ndescription: test\nversion: 1.0.0\ntype: service\n",
    )
    .expect("template metadata should be written");

    let resolution =
        support::create_template_resolution(&broken_template_root, "official", "dotnet-service");
    let orchestration = support::build_default_orchestration(&workspace_root, resolution);

    let error = support::execute_non_interactive_add_service(
        &orchestration,
        "Orders",
        "official/dotnet-service",
    )
    .expect_err("render should fail for missing template content");

    match error {
        AddServiceError::RenderFailed(_) => {}
        other => panic!("unexpected error: {other}"),
    }
    assert!(
        !workspace_root.join("src/Orders").exists(),
        "partial output should be cleaned up"
    );

    support::cleanup_sandbox_directory(&workspace_root);
}

#[test]
fn removes_partial_output_when_generation_is_interrupted() {
    let workspace_root = support::create_workspace_root("service-cleanup-interrupt");
    let template_root =
        support::create_service_template(&workspace_root, "dotnet-service-template", "service");
    let resolution =
        support::create_template_resolution(&template_root, "official", "dotnet-service");
    let input_resolution = AddServiceInputResolutionService::new(
        support::StaticTemplateSelector { resolution },
        support::FirstTemplatePrompt,
        support::FailingPromptService,
    );
    let orchestration = AddServiceCommandHandler::new(
        support::FixedWorkingDirectoryProvider {
            current_directory: workspace_root.clone(),
        },
        input_resolution,
        InterruptingRenderer,
        ServiceTemplateProvenanceService::new(WorkspaceMetadataWriter::new()),
    );

    let command = AddServiceCommand::new(
        Some("Orders".to_owned()),
        Some("official/dotnet-service".to_owned()),
        true,
        false,
    );
    let error = orchestration
        .handle(&command)
        .expect_err("interrupting renderer should fail");

    assert_eq!(error, AddServiceError::Interrupted);
    assert!(
        !workspace_root.join("src/Orders").exists(),
        "partial output should be cleaned up after interruption"
    );

    support::cleanup_sandbox_directory(&workspace_root);
}
