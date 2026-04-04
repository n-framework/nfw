#[path = "support.rs"]
mod support;

use nframework_nfw_application::features::service_management::models::errors::add_service_error::AddServiceError;

#[test]
fn fails_when_current_directory_is_not_inside_workspace() {
    let sandbox_root = support::create_sandbox_directory("service-outside-workspace");
    let template_root = support::create_service_template(
        &sandbox_root,
        "dotnet-service-template",
        "service",
        true,
        true,
    );
    let resolution =
        support::create_template_resolution(&template_root, "official", "dotnet-service");
    let orchestration = support::build_default_orchestration(&sandbox_root, resolution);

    let error = support::execute_non_interactive_add_service(
        &orchestration,
        "Orders",
        "official/dotnet-service",
    )
    .expect_err("generation should fail outside a workspace");

    match error {
        AddServiceError::InvalidWorkspaceContext(_) => {}
        other => panic!("unexpected error: {other}"),
    }

    support::cleanup_sandbox_directory(&sandbox_root);
}
