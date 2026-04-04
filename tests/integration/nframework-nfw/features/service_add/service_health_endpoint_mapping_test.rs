#[path = "support.rs"]
mod support;

use std::fs;

#[test]
fn generated_api_contains_health_endpoint_mappings() {
    let workspace_root = support::create_workspace_root("service-health-mapping");
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

    let program_file = workspace_root.join("src/Orders/Api/Program.cs");
    let program_content =
        fs::read_to_string(&program_file).expect("generated Program.cs should be readable");
    assert!(program_content.contains("/health/live"));
    assert!(program_content.contains("/health/ready"));

    support::cleanup_sandbox_directory(&workspace_root);
}
