mod support;

use support::{add_project, cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_produces_deterministic_output_for_same_input() {
    let workspace_root = create_workspace("deterministic");
    add_project(
        &workspace_root,
        "src/NFramework.Domain",
        "NFramework.Domain",
        &["../NFramework.Application/NFramework.Application.csproj"],
        &["Microsoft.AspNetCore.App"],
        &[(
            "DomainModel.cs",
            "using NFramework.Infrastructure.Persistence;\nnamespace NFramework.Domain;\npublic class DomainModel {}\n",
        )],
    );

    let first_run = run_nfw_check(&workspace_root);
    let second_run = run_nfw_check(&workspace_root);

    assert!(!first_run.status.success());
    assert_eq!(first_run.status.code(), second_run.status.code());
    assert_eq!(first_run.stderr, second_run.stderr);

    cleanup_workspace(&workspace_root);
}
