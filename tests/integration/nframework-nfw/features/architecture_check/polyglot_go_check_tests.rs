mod support;

use std::fs;

use support::{cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_detects_go_forbidden_usage() {
    let workspace_root = create_workspace("polyglot-go");

    let domain_dir = workspace_root.join("src/domain");
    fs::create_dir_all(&domain_dir).expect("go domain directory should be created");
    fs::write(
        domain_dir.join("go.mod"),
        r#"module example.com/domain

go 1.21

require (
	github.com/gin-gonic/gin v1.9.1
)
"#,
    )
    .expect("go manifest should be written");
    fs::write(
        domain_dir.join("main.go"),
        r#"package main

import "github.com/gin-gonic/gin"

func main() {
	r := gin.Default()
	r.GET("/", func(c *gin.Context) {})
}
"#,
    )
    .expect("go source should be written");

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("type=package_usage"));
    assert!(stderr.contains("gin-gonic/gin"));

    cleanup_workspace(&workspace_root);
}
