#[path = "support.rs"]
mod support;

use std::fs;

use nframework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;

#[test]
fn rejects_rendered_paths_that_escape_service_output_root() {
    let workspace_root = support::create_sandbox_directory("service-path-safety");
    fs::create_dir_all(workspace_root.join("src"))
        .expect("workspace src directory should be created");
    fs::write(
        workspace_root.join("nfw.yaml"),
        "$schema: https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json\nworkspace:\n  name: ../escaped\n  namespace: BillingPlatform\n",
    )
    .expect("workspace metadata should be written");

    let template_root =
        support::create_service_template(&workspace_root, "dotnet-service-template", "service");
    fs::create_dir_all(template_root.join("content/__WorkspaceName__"))
        .expect("malicious placeholder directory should be created");
    fs::write(
        template_root.join("content/__WorkspaceName__/payload.txt"),
        "unsafe path payload",
    )
    .expect("malicious placeholder file should be written");

    let resolution =
        support::create_template_resolution(&template_root, "official", "dotnet-service");
    let orchestration = support::build_default_orchestration(&workspace_root, resolution);

    let error = support::execute_non_interactive_add_service(
        &orchestration,
        "Orders",
        "official/dotnet-service",
    )
    .expect_err("unsafe rendered path should fail");

    match error {
        AddServiceError::RenderFailed(message) => {
            assert!(
                message.contains("unsafe rendered path"),
                "expected unsafe path render error, got: {message}"
            );
        }
        other => panic!("unexpected error: {other}"),
    }

    assert!(
        !workspace_root.join("src/escaped/payload.txt").exists(),
        "renderer must not write files outside service output root"
    );
    assert!(
        !workspace_root.join("src/Orders").exists(),
        "service output root should be cleaned when render fails"
    );

    support::cleanup_sandbox_directory(&workspace_root);
}
