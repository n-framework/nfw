#[path = "support.rs"]
mod support;

use nframework_nfw_application::features::service_management::models::errors::add_service_error::AddServiceError;

#[test]
fn generated_service_respects_layer_dependency_contract() {
    let workspace_root = support::create_workspace_root("service-dependency-contract-pass");
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

    support::cleanup_sandbox_directory(&workspace_root);
}

#[test]
fn fails_when_generated_references_break_dependency_contract() {
    let workspace_root = support::create_workspace_root("service-dependency-contract-fail");
    let template_root = support::create_service_template(
        &workspace_root,
        "dotnet-service-template",
        "service",
        true,
        false,
    );
    let template_resolution =
        support::create_template_resolution(&template_root, "official", "dotnet-service");
    let orchestration = support::build_default_orchestration(&workspace_root, template_resolution);

    let error = support::execute_non_interactive_add_service(
        &orchestration,
        "Orders",
        "official/dotnet-service",
    )
    .expect_err("invalid dependency graph should fail");

    match error {
        AddServiceError::DependencyRuleViolation(_) => {}
        other => panic!("unexpected error: {other}"),
    }

    support::cleanup_sandbox_directory(&workspace_root);
}
