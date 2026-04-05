mod support;

use std::fs;

use support::{cleanup_workspace, create_workspace, run_nfw_check};

#[test]
fn check_detects_rust_forbidden_usage_and_references() {
    let workspace_root = create_workspace("polyglot-rust");

    let domain_dir = workspace_root.join("src/nframework-domain");
    fs::create_dir_all(domain_dir.join("src")).expect("rust domain src directory should be created");
    fs::write(
        domain_dir.join("Cargo.toml"),
        r#"[package]
name = "nframework-domain"
version = "0.1.0"
edition = "2024"

[dependencies]
nframework-application = { path = "../nframework-application" }
axum = "0.7"
"#,
    )
    .expect("rust domain manifest should be written");
    fs::write(
        domain_dir.join("src/lib.rs"),
        "use nframework::infrastructure::db::Connection;\npub struct DomainModel;\n",
    )
    .expect("rust domain source should be written");

    let application_dir = workspace_root.join("src/nframework-application");
    fs::create_dir_all(application_dir.join("src"))
        .expect("rust application src directory should be created");
    fs::write(
        application_dir.join("Cargo.toml"),
        r#"[package]
name = "nframework-application"
version = "0.1.0"
edition = "2024"
"#,
    )
    .expect("rust application manifest should be written");
    fs::write(application_dir.join("src/lib.rs"), "pub struct ApplicationService;\n")
        .expect("rust application source should be written");

    let output = run_nfw_check(&workspace_root);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("type=project_reference"));
    assert!(stderr.contains("type=package_usage"));
    assert!(stderr.contains("type=namespace_usage"));

    cleanup_workspace(&workspace_root);
}
