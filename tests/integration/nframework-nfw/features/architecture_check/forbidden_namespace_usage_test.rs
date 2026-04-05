mod support;

use support::{ProjectConfig, add_project, cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_reports_forbidden_namespace_usage() {
    let workspace_root = create_workspace("forbidden-namespace");
    add_project(ProjectConfig {
        workspace_root: &workspace_root,
        relative_project_dir: "src/domain",
        project_name: "NFramework.Domain",
        project_references: &[],
        package_references: &[],
        source_files: &[(
            "DomainModel.cs",
            "using NFramework.Infrastructure.Persistence;\nnamespace NFramework.Domain;\npublic class DomainModel {}\n",
        )],
    });

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("type=namespace_usage"));
    assert!(stderr.contains("NFramework.Infrastructure.Persistence"));

    cleanup_workspace(&workspace_root);
}
