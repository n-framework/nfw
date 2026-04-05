mod support;

use support::{add_project, cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_succeeds_for_valid_workspace_fixture() {
    let workspace_root = create_workspace("valid");
    add_project(
        &workspace_root,
        "src/NFramework.Domain",
        "NFramework.Domain",
        &[],
        &[],
        &[("DomainModel.cs", "namespace NFramework.Domain;\npublic class DomainModel {}\n")],
    );

    let output = run_nfw_check(&workspace_root);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("architecture validation passed"));

    cleanup_workspace(&workspace_root);
}
