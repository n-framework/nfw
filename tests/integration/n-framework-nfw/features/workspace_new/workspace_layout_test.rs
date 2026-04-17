use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use n_framework_nfw_core_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::workspace_writer::WorkspaceWriter;
use n_framework_nfw_core_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;
use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::file_system_workspace_writer::FileSystemWorkspaceWriter;
use n_framework_nfw_infrastructure_filesystem::features::template_management::template_engine::FileSystemTemplateEngine;

const EXPECTED_NFW_YAML_PREFIX: &str = "\
#    _  ______                                   __
#   / |/ / __/______ ___ _  ___ _    _____  ____/ /__
#  /    / _// __/ _ `/  ' \\/ -_) |/|/ / _ \\/ __/  '_/
# /_/|_/_/ /_/  \\_,_/_/_/_/\\__/|__,__/\\___/_/ /_/\\_\\

# yaml-language-server: $schema=https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json
";

#[test]
fn generates_expected_workspace_layout_and_yaml_baseline_configuration() {
    let sandbox_root = create_sandbox_directory("workspace-layout");
    let template_root = create_template_directory(&sandbox_root);
    let output_path = sandbox_root.join("BillingPlatform");
    let blueprint = WorkspaceBlueprint::new("BillingPlatform");
    let resolution = NewCommandResolution {
        workspace_name: "BillingPlatform".to_owned(),
        template_id: "official/blank-workspace".to_owned(),
        template_cache_path: template_root.clone(),
        namespace_base: "BillingPlatform".to_owned(),
        output_path: output_path.clone(),
    };

    let writer = FileSystemWorkspaceWriter::new(FileSystemTemplateEngine::new());
    writer
        .write_workspace(&blueprint, &resolution)
        .expect("workspace generation should succeed");

    assert!(output_path.join("src").is_dir());
    assert!(output_path.join("tests").is_dir());
    assert!(output_path.join("docs").is_dir());
    assert!(output_path.join("workspace.manifest").is_file());
    assert!(
        output_path
            .join("src/BillingPlatform/service.manifest")
            .is_file()
    );
    assert!(!output_path.join("nfw.schema.json").exists());
    assert!(!output_path.join("BillingPlatform.sln").exists());
    assert!(
        !output_path
            .join("src/BillingPlatform/BillingPlatform.Service.sln")
            .exists()
    );
    assert!(output_path.join("README.md").is_file());
    assert!(output_path.join("nfw.yaml").is_file());

    let yaml =
        fs::read_to_string(output_path.join("nfw.yaml")).expect("nfw.yaml should be readable");
    assert_eq!(
        yaml,
        format!(
            "{}$schema: https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json\n\nworkspace:\n  name: BillingPlatform\n  template: official/blank-workspace\n  namespace: BillingPlatform\n",
            EXPECTED_NFW_YAML_PREFIX
        )
    );

    cleanup_sandbox_directory(&sandbox_root);
}

#[test]
fn fails_when_target_directory_already_exists_and_is_not_empty() {
    let sandbox_root = create_sandbox_directory("workspace-layout-non-empty");
    let template_root = create_template_directory(&sandbox_root);
    let output_path = sandbox_root.join("BillingPlatform");
    fs::create_dir_all(&output_path).expect("target directory should be created");
    fs::write(output_path.join("existing.txt"), "content").expect("seed file should be written");

    let blueprint = WorkspaceBlueprint::new("BillingPlatform");
    let resolution = NewCommandResolution {
        workspace_name: "BillingPlatform".to_owned(),
        template_id: "official/blank-workspace".to_owned(),
        template_cache_path: template_root,
        namespace_base: "BillingPlatform".to_owned(),
        output_path: output_path.clone(),
    };

    let writer = FileSystemWorkspaceWriter::new(FileSystemTemplateEngine::new());
    let error = writer
        .write_workspace(&blueprint, &resolution)
        .expect_err("workspace generation should fail for non-empty target directory");
    assert!(
        error.contains("already exists and is not empty"),
        "unexpected error: {error}"
    );

    cleanup_sandbox_directory(&sandbox_root);
}

#[test]
fn creates_nfw_yaml_when_template_does_not_provide_it() {
    let sandbox_root = create_sandbox_directory("workspace-layout-missing-yaml");
    let template_root = create_template_directory_without_nfw_yaml(&sandbox_root);
    let output_path = sandbox_root.join("BillingPlatform");
    let blueprint = WorkspaceBlueprint::new("BillingPlatform");
    let resolution = NewCommandResolution {
        workspace_name: "BillingPlatform".to_owned(),
        template_id: "official/blank-workspace".to_owned(),
        template_cache_path: template_root,
        namespace_base: "BillingPlatform".to_owned(),
        output_path: output_path.clone(),
    };

    let writer = FileSystemWorkspaceWriter::new(FileSystemTemplateEngine::new());
    writer
        .write_workspace(&blueprint, &resolution)
        .expect("workspace generation should succeed");

    let yaml =
        fs::read_to_string(output_path.join("nfw.yaml")).expect("nfw.yaml should be readable");
    assert_eq!(
        yaml,
        format!(
            "{}$schema: https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json\n\nworkspace:\n  name: BillingPlatform\n  template: official/blank-workspace\n  namespace: BillingPlatform\n",
            EXPECTED_NFW_YAML_PREFIX
        )
    );

    cleanup_sandbox_directory(&sandbox_root);
}

#[test]
fn does_not_create_default_root_directories_when_template_omits_them() {
    let sandbox_root = create_sandbox_directory("workspace-layout-no-default-roots");
    let template_root = create_template_directory_with_only_nfw_yaml(&sandbox_root);
    let output_path = sandbox_root.join("BillingPlatform");
    let blueprint = WorkspaceBlueprint::new("BillingPlatform");
    let resolution = NewCommandResolution {
        workspace_name: "BillingPlatform".to_owned(),
        template_id: "official/blank-workspace".to_owned(),
        template_cache_path: template_root,
        namespace_base: "BillingPlatform".to_owned(),
        output_path: output_path.clone(),
    };

    let writer = FileSystemWorkspaceWriter::new(FileSystemTemplateEngine::new());
    writer
        .write_workspace(&blueprint, &resolution)
        .expect("workspace generation should succeed");

    assert!(!output_path.join("src").exists());
    assert!(!output_path.join("tests").exists());
    assert!(!output_path.join("docs").exists());
    assert!(output_path.join("nfw.yaml").is_file());

    cleanup_sandbox_directory(&sandbox_root);
}

fn create_sandbox_directory(test_name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("nfw-{test_name}-{unique}"));
    fs::create_dir_all(&path).expect("sandbox directory should be created");
    path
}

fn cleanup_sandbox_directory(path: &Path) {
    let _ = fs::remove_dir_all(path);
}

fn create_template_directory(sandbox_root: &Path) -> PathBuf {
    let template_root = sandbox_root.join("templates/official/src/blank-workspace");
    let content_root = template_root.join("content");

    fs::create_dir_all(content_root.join("src/{{WorkspaceName}}"))
        .expect("template source directory should be created");
    fs::create_dir_all(content_root.join("tests"))
        .expect("template tests directory should be created");
    fs::create_dir_all(content_root.join("docs"))
        .expect("template docs directory should be created");

    fs::write(
        content_root.join("workspace.manifest"),
        "workspace for {{WorkspaceName}}",
    )
    .expect("template manifest should be written");
    fs::write(
        content_root.join("src/{{WorkspaceName}}/service.manifest"),
        "service for {{ServiceName}} in {{Namespace}}",
    )
    .expect("template service manifest should be written");
    fs::write(
        content_root.join("README.md"),
        "# {{WorkspaceName}}\nnamespace: {{Namespace}}\n",
    )
    .expect("template readme should be written");
    fs::write(
        content_root.join("nfw.yaml"),
        "$schema: https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json\nworkspace:\n  name: {{WorkspaceName}}\n  template: official/blank-workspace\n  namespace: {{Namespace}}\n",
    )
    .expect("template config should be written");

    template_root
}

fn create_template_directory_without_nfw_yaml(sandbox_root: &Path) -> PathBuf {
    let template_root = sandbox_root.join("templates/official/src/blank-workspace-no-yaml");
    let content_root = template_root.join("content");

    fs::create_dir_all(content_root.join("src/{{WorkspaceName}}"))
        .expect("template source directory should be created");
    fs::create_dir_all(content_root.join("tests"))
        .expect("template tests directory should be created");
    fs::create_dir_all(content_root.join("docs"))
        .expect("template docs directory should be created");

    fs::write(
        content_root.join("workspace.manifest"),
        "workspace for {{WorkspaceName}}",
    )
    .expect("template manifest should be written");

    template_root
}

fn create_template_directory_with_only_nfw_yaml(sandbox_root: &Path) -> PathBuf {
    let template_root = sandbox_root.join("templates/official/src/blank-workspace-only-yaml");
    let content_root = template_root.join("content");

    fs::create_dir_all(&content_root).expect("template content directory should be created");
    fs::write(
        content_root.join("nfw.yaml"),
        "$schema: https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json\nworkspace:\n  name: __WorkspaceName__\n  template: official/blank-workspace\n  namespace: __Namespace__\n",
    )
    .expect("template config should be written");

    template_root
}
