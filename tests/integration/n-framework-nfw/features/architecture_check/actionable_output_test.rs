mod support;

use support::{ProjectConfig, add_project, cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_output_contains_type_location_offending_and_hint() {
    let workspace_root = create_workspace("actionable");
    add_project(ProjectConfig {
        workspace_root: &workspace_root,
        relative_project_dir: "src/domain",
        project_name: "NFramework.Domain",
        project_references: &["../application/NFramework.Application.csproj"],
        package_references: &[],
        source_files: &[(
            "DomainModel.cs",
            "namespace NFramework.Domain;\npublic class DomainModel {}\n",
        )],
    });
    // Create the referenced Application project
    add_project(ProjectConfig {
        workspace_root: &workspace_root,
        relative_project_dir: "src/application",
        project_name: "NFramework.Application",
        project_references: &[],
        package_references: &[],
        source_files: &[(
            "ApplicationService.cs",
            "namespace NFramework.Application;\npublic class ApplicationService {}\n",
        )],
    });

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("type=project_reference"));
    assert!(stderr.contains("location="));
    assert!(stderr.contains("offending="));
    assert!(stderr.contains("hint="));

    cleanup_workspace(&workspace_root);
}
