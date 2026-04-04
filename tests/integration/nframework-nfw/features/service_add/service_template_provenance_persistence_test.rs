#[path = "support.rs"]
mod support;

use std::fs;

#[test]
fn persists_service_template_provenance_in_nfw_yaml() {
    let workspace_root = support::create_workspace_root("service-provenance");
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

    let metadata_content =
        fs::read_to_string(workspace_root.join("nfw.yaml")).expect("nfw.yaml should be readable");
    assert!(metadata_content.starts_with("#    _  ______"));
    assert!(metadata_content.contains(
        "\n# yaml-language-server: $schema=https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json\n"
    ));
    assert!(metadata_content.contains("services:"));
    assert!(metadata_content.contains("Orders:"));
    assert!(metadata_content.contains("path: src/Orders"));
    assert!(metadata_content.contains("id: official/dotnet-service"));
    assert!(metadata_content.contains("version: 1.0.0"));

    support::cleanup_sandbox_directory(&workspace_root);
}
