mod support;

use support::{ProjectConfig, add_project, cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_succeeds_for_valid_workspace_fixture() {
    let workspace_root = create_workspace("valid");
    add_project(ProjectConfig {
        workspace_root: &workspace_root,
        relative_project_dir: "src/NFramework.Domain",
        project_name: "NFramework.Domain",
        project_references: &[],
        package_references: &[],
        source_files: &[(
            "DomainModel.cs",
            "namespace NFramework.Domain;\npublic class DomainModel {}\n",
        )],
    });

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success());
    assert!(stderr.contains("architecture validation passed"));

    cleanup_workspace(&workspace_root);
}
