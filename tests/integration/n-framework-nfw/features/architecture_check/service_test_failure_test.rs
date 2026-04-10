mod support;

use std::fs;

use support::{cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_reports_test_issue_when_service_make_test_fails() {
    let workspace_root = create_workspace("service-test-failure");
    fs::create_dir_all(workspace_root.join("src/Orders"))
        .expect("service directory should be created");
    fs::write(
        workspace_root.join("nfw.yaml"),
        "workspace:\n  name: TestWorkspace\nservices:\n  Orders:\n    path: src/Orders\n",
    )
    .expect("workspace metadata should be written");
    fs::write(
        workspace_root.join("src/Orders/Makefile"),
        ".PHONY: test\n\ntest:\n\t@echo \"service tests failed\" >&2\n\t@exit 1\n",
    )
    .expect("service Makefile should be written");

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("type=test_issue"));
    assert!(stderr.contains("service tests failed"));

    cleanup_workspace(&workspace_root);
}
