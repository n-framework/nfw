mod support;

use support::{add_project, cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_output_contains_type_location_offending_and_hint() {
    let workspace_root = create_workspace("actionable");
    add_project(
        &workspace_root,
        "src/NFramework.Domain",
        "NFramework.Domain",
        &["../NFramework.Application/NFramework.Application.csproj"],
        &[],
        &[("DomainModel.cs", "namespace NFramework.Domain;\npublic class DomainModel {}\n")],
    );

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("type=project_reference"));
    assert!(stderr.contains("location="));
    assert!(stderr.contains("offending="));
    assert!(stderr.contains("hint="));

    cleanup_workspace(&workspace_root);
}
