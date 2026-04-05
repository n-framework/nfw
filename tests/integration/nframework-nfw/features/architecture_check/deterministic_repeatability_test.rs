mod support;

use support::{ProjectConfig, add_project, cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_produces_deterministic_output_for_same_input() {
    let workspace_root = create_workspace("deterministic");
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

    let first_run = run_nfw_check(&workspace_root);
    let second_run = run_nfw_check(&workspace_root);

    assert!(!first_run.status.success());
    assert_eq!(first_run.status.code(), second_run.status.code());
    assert_eq!(first_run.stderr, second_run.stderr);

    cleanup_workspace(&workspace_root);
}
