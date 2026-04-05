mod support;

use support::{ProjectConfig, add_project, cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_reports_multiple_violation_types_in_single_run() {
    let workspace_root = create_workspace("multi-violation");
    add_project(ProjectConfig {
        workspace_root: &workspace_root,
        relative_project_dir: "src/domain",
        project_name: "NFramework.Domain",
        project_references: &["../application/NFramework.Application.csproj"],
        package_references: &["Microsoft.AspNetCore.App"],
        source_files: &[(
            "DomainModel.cs",
            "using NFramework.Infrastructure.Persistence;\nnamespace NFramework.Domain;\npublic class DomainModel {}\n",
        )],
    });

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("type=project_reference"));
    assert!(stderr.contains("type=namespace_usage"));
    assert!(stderr.contains("type=package_usage"));

    cleanup_workspace(&workspace_root);
}
