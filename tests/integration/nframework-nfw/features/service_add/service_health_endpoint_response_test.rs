#[path = "support.rs"]
mod support;

use nframework_nfw_infrastructure_filesystem::features::service_management::services::generated_api_contract_inspector::FileSystemGeneratedApiContractInspector;
use nframework_nfw_application::features::service_management::services::abstraction::generated_api_contract_inspector::GeneratedApiContractInspector;

#[test]
fn generated_api_health_contract_inspection_passes() {
    let workspace_root = support::create_workspace_root("service-health-response");
    let template_root = support::create_service_template(
        &workspace_root,
        "dotnet-service-template",
        "service",
        true,
        true,
    );
    let template_resolution =
        support::create_template_resolution(&template_root, "official", "dotnet-service");
    let orchestration = support::build_default_orchestration(&workspace_root, template_resolution);

    support::execute_non_interactive_add_service(
        &orchestration,
        "Orders",
        "official/dotnet-service",
    )
    .expect("service generation should succeed");

    let inspector = FileSystemGeneratedApiContractInspector::new();
    inspector
        .assert_health_endpoints(&workspace_root.join("src/Orders"))
        .expect("health contract inspection should pass");

    support::cleanup_sandbox_directory(&workspace_root);
}
