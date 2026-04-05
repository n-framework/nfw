mod support;

use std::fs;

use support::{cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_uses_service_make_lint_and_not_workspace_root() {
    let workspace_root = create_workspace("service-lint-scope");

    fs::create_dir_all(workspace_root.join("src/Orders"))
        .expect("service directory should be created");
    fs::write(
        workspace_root.join("nfw.yaml"),
        "workspace:\n  name: TestWorkspace\nservices:\n  Orders:\n    path: src/Orders\n",
    )
    .expect("workspace metadata should be written");
    fs::write(
        workspace_root.join("Makefile"),
        ".PHONY: test\ntest:\n\t@echo \"root test only\"\n",
    )
    .expect("workspace Makefile should be written");
    fs::write(
        workspace_root.join("src/Orders/Makefile"),
        ".PHONY: lint test\nlint:\n\t@echo \"service lint ok\"\n\ntest:\n\t@echo \"service test ok\"\n",
    )
    .expect("service Makefile should be written");

    let output = run_nfw_check(&workspace_root);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success());
    assert!(stdout.contains("architecture validation passed"));
    assert!(!stderr.contains("type=lint_issue"));

    cleanup_workspace(&workspace_root);
}
