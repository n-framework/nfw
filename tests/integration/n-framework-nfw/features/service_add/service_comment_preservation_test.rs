#[path = "support.rs"]
mod support;

use std::fs;

#[test]
fn preserves_all_comments_in_nfw_yaml_when_adding_service() {
    let workspace_root = support::create_workspace_root("comment-preservation-all");
    let workspace_file = workspace_root.join("nfw.yaml");

    let original_content = r#"#    _  ______                                   __
#   / |/ / __/______ ___ _  ___ _    _____  ____/ /__
#  /    / _// __/ _ `/  ' \/ -_) |/|/ / _ \/ __/  '_/
# /_/|_/_/ /_/  \_,_/_/_/_/\__/|__,__/\___/_/ /_/\_\

# yaml-language-server: $schema=https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json
$schema: https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json

# Important workspace configuration
workspace:
  name: BillingPlatform
  # The namespace is used for code generation
  namespace: BillingPlatform

# Sources for our templates
template_sources:
  local: "../../../src/nfw-templates"

services:
  # Core billing service
  BillingService:
    path: src/BillingService # inline comment
    template:
      id: local/dotnet-service
      version: 1.0.0
"#;
    fs::write(&workspace_file, original_content).expect("nfw.yaml should be writable");

    let template_root =
        support::create_service_template(&workspace_root, "dotnet-service-template", "service");
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
        fs::read_to_string(&workspace_file).expect("nfw.yaml should be readable");

    // Assert that standard comments are preserved (current behavior)
    assert!(metadata_content.contains("# Important workspace configuration"));

    // Assert that comments the current implementation likely loses (THE BUG)
    assert!(
        metadata_content.contains("# The namespace is used for code generation"),
        "Nested workspace comments should be preserved"
    );
    assert!(
        metadata_content.contains("# Sources for our templates"),
        "template_sources comments should be preserved"
    );
    assert!(
        metadata_content.contains("# Core billing service"),
        "Service-specific comments should be preserved"
    );
    assert!(
        metadata_content.contains("# inline comment"),
        "Inline comments should be preserved"
    );

    support::cleanup_sandbox_directory(&workspace_root);
}
