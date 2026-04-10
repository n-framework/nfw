mod support;

use std::fs;

use support::{ProjectConfig, add_project, cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_reports_lint_issue_when_make_lint_fails() {
    let workspace_root = create_workspace("lint-failure");
    fs::create_dir_all(workspace_root.join("src/Orders"))
        .expect("service directory should be created");
    fs::write(
        workspace_root.join("nfw.yaml"),
        "workspace:\n  name: TestWorkspace\nservices:\n  Orders:\n    path: src/Orders\n",
    )
    .expect("workspace metadata should be written");
    fs::write(
        workspace_root.join("src/Orders/Makefile"),
        ".PHONY: lint test\nlint:\n\t@echo \"lint violation\" >&2\n\t@exit 1\n\ntest:\n\t@echo \"tests ok\"\n",
    )
    .expect("failing service Makefile should be written");

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

    assert!(!output.status.success());
    assert!(stderr.contains("type=lint_issue"));
    assert!(stderr.contains("lint violation"));

    cleanup_workspace(&workspace_root);
}
