use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use nframework_nfw_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use nframework_nfw_application::features::workspace_management::services::abstraction::workspace_writer::WorkspaceWriter;
use nframework_nfw_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;
use nframework_nfw_infrastructure_filesystem::features::workspace_management::services::file_system_workspace_writer::FileSystemWorkspaceWriter;

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

    let writer = FileSystemWorkspaceWriter::new();
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
    assert!(!output_path.join("BillingPlatform.sln").exists());
    assert!(
        !output_path
            .join("src/BillingPlatform/BillingPlatform.Service.sln")
            .exists()
    );
    assert!(output_path.join("README.md").is_file());
    assert!(output_path.join("nfw.yaml").is_file());

    let yaml = fs::read_to_string(output_path.join("nfw.yaml"))
        .expect("nfw.yaml should be readable");
    assert_eq!(
        yaml,
        format!(
            "workspace:\n  name: BillingPlatform\n  template: official/blank-workspace\n  namespace: BillingPlatform\n  projectGuid: {}\n",
            stable_project_guid("BillingPlatform", "official/blank-workspace")
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

    let writer = FileSystemWorkspaceWriter::new();
    let error = writer
        .write_workspace(&blueprint, &resolution)
        .expect_err("workspace generation should fail for non-empty target directory");
    assert!(
        error.contains("already exists and is not empty"),
        "unexpected error: {error}"
    );

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

    fs::create_dir_all(content_root.join("src/__WorkspaceName__"))
        .expect("template source directory should be created");
    fs::create_dir_all(content_root.join("tests"))
        .expect("template tests directory should be created");
    fs::create_dir_all(content_root.join("docs"))
        .expect("template docs directory should be created");

    fs::write(content_root.join("workspace.manifest"), "workspace for __WorkspaceName__")
        .expect("template manifest should be written");
    fs::write(
        content_root.join("src/__WorkspaceName__/service.manifest"),
        "service for __ServiceName__ in __Namespace__",
    )
    .expect("template service manifest should be written");
    fs::write(
        content_root.join("README.md"),
        "# __WorkspaceName__\nnamespace: __Namespace__\n",
    )
    .expect("template readme should be written");
    fs::write(
        content_root.join("nfw.yaml"),
        "workspace:\n  name: __WorkspaceName__\n  template: official/blank-workspace\n  namespace: __Namespace__\n  projectGuid: __ProjectGuid__\n",
    )
    .expect("template config should be written");

    template_root
}

fn stable_project_guid(workspace_name: &str, template_id: &str) -> String {
    let mut state_a: u64 = 0xcbf29ce484222325;
    let mut state_b: u64 = 0x8422_2325_cbf2_9ce4;
    for byte in workspace_name.bytes().chain(template_id.bytes()) {
        state_a ^= byte as u64;
        state_a = state_a.wrapping_mul(0x100000001b3);

        state_b ^= (byte as u64) << 1;
        state_b = state_b.wrapping_mul(0x100000001b3);
    }

    let part1 = (state_a >> 32) as u32;
    let part2 = ((state_a >> 16) & 0xffff) as u16;
    let part3 = (state_a & 0xffff) as u16;
    let part4 = ((state_b >> 48) & 0xffff) as u16;
    let part5 = state_b & 0xffff_ffff_ffff;

    format!("{part1:08x}-{part2:04x}-{part3:04x}-{part4:04x}-{part5:012x}")
}
