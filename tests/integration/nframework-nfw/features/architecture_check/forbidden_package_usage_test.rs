mod support;

use std::fs;

use support::{add_project, cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_reports_forbidden_direct_package_usage() {
    let workspace_root = create_workspace("forbidden-package");
    add_project(
        &workspace_root,
        "src/NFramework.Domain",
        "NFramework.Domain",
        &[],
        &["Microsoft.AspNetCore.App"],
        &[("DomainModel.cs", "namespace NFramework.Domain;\npublic class DomainModel {}\n")],
    );

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("type=package_usage"));
    assert!(stderr.contains("Microsoft.AspNetCore.App"));

    cleanup_workspace(&workspace_root);
}

#[test]
fn check_ignores_transitive_dependency_only_entries() {
    let workspace_root = create_workspace("transitive-only");
    add_project(
        &workspace_root,
        "src/NFramework.Domain",
        "NFramework.Domain",
        &[],
        &[],
        &[("DomainModel.cs", "namespace NFramework.Domain;\npublic class DomainModel {}\n")],
    );

    let lock_file = workspace_root.join("src/NFramework.Domain/packages.lock.json");
    fs::write(
        lock_file,
        r#"{
  "version": 1,
  "dependencies": {
    "net9.0": {
      "Microsoft.AspNetCore.App": {
        "type": "Transitive"
      }
    }
  }
}"#,
    )
    .expect("lock file should be written");

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success());
    assert!(!stderr.contains("type=package_usage"));

    cleanup_workspace(&workspace_root);
}
