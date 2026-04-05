mod support;

use std::fs;

use support::{cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_detects_node_forbidden_usage() {
    let workspace_root = create_workspace("polyglot-node");

    let domain_dir = workspace_root.join("src/domain");
    fs::create_dir_all(domain_dir.join("src"))
        .expect("node domain src directory should be created");
    fs::write(
        domain_dir.join("package.json"),
        r#"{
  "name": "nframework-domain",
  "version": "1.0.0",
  "dependencies": {
    "express": "^5.0.0"
  }
}"#,
    )
    .expect("node manifest should be written");
    fs::write(
        domain_dir.join("src/index.ts"),
        "import { router } from 'nframework/presentation/http';\nexport const v = router;\n",
    )
    .expect("node source should be written");

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("type=package_usage"));
    assert!(stderr.contains("express"));
    assert!(stderr.contains("type=namespace_usage"));

    cleanup_workspace(&workspace_root);
}
