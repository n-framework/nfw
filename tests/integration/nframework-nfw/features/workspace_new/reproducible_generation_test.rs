use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use nframework_nfw_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use nframework_nfw_application::features::workspace_management::services::abstractions::workspace_writer::WorkspaceWriter;
use nframework_nfw_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;
use nframework_nfw_infrastructure_filesystem::features::workspace_management::services::file_system_workspace_writer::FileSystemWorkspaceWriter;

#[test]
fn workspace_generation_is_reproducible_for_identical_inputs() {
    let sandbox_root = create_sandbox_directory("workspace-reproducibility");
    let template_root = create_template_directory(&sandbox_root);
    let first_output_path = sandbox_root.join("first/BillingPlatform");
    let second_output_path = sandbox_root.join("second/BillingPlatform");

    let blueprint = WorkspaceBlueprint::new("BillingPlatform");
    let first_resolution = NewCommandResolution {
        workspace_name: "BillingPlatform".to_owned(),
        template_id: "official/blank-workspace".to_owned(),
        template_cache_path: template_root.clone(),
        namespace_base: "BillingPlatform".to_owned(),
        output_path: first_output_path.clone(),
    };
    let second_resolution = NewCommandResolution {
        workspace_name: "BillingPlatform".to_owned(),
        template_id: "official/blank-workspace".to_owned(),
        template_cache_path: template_root,
        namespace_base: "BillingPlatform".to_owned(),
        output_path: second_output_path.clone(),
    };

    let writer = FileSystemWorkspaceWriter::new();
    writer
        .write_workspace(&blueprint, &first_resolution)
        .expect("first generation should succeed");
    writer
        .write_workspace(&blueprint, &second_resolution)
        .expect("second generation should succeed");

    let first_files = collect_relative_file_paths(&first_output_path);
    let second_files = collect_relative_file_paths(&second_output_path);
    assert_eq!(first_files, second_files);

    for relative_path in first_files {
        let first_content = fs::read(first_output_path.join(&relative_path))
            .expect("first file should be readable");
        let second_content = fs::read(second_output_path.join(&relative_path))
            .expect("second file should be readable");
        assert_eq!(
            first_content, second_content,
            "mismatch for {relative_path:?}"
        );
    }

    cleanup_sandbox_directory(&sandbox_root);
}

fn collect_relative_file_paths(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::<PathBuf>::new();
    collect_relative_file_paths_recursive(root, root, &mut files);
    files.sort();
    files
}

fn collect_relative_file_paths_recursive(root: &Path, current: &Path, files: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(current).expect("directory should be readable");
    for entry in entries {
        let entry = entry.expect("directory entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            collect_relative_file_paths_recursive(root, &path, files);
            continue;
        }

        let relative_path = path
            .strip_prefix(root)
            .expect("relative path should be derivable")
            .to_path_buf();
        files.push(relative_path);
    }
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

    fs::write(
        content_root.join("workspace.manifest"),
        "workspace for __WorkspaceName__",
    )
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
        "$schema: https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json\nworkspace:\n  name: __WorkspaceName__\n  template: official/blank-workspace\n  namespace: __Namespace__\n",
    )
    .expect("template config should be written");

    template_root
}
