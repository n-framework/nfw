use std::fs;
use std::path::PathBuf;
use std::process::Command;

const TEMPLATE_ID: &str = "official/blank-workspace";

fn create_sandbox_directory(name: &str) -> PathBuf {
    let tmp_dir = std::env::temp_dir().join(format!("nfw-test-{}", name));
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir).expect("should create sandbox directory");
    tmp_dir
}

fn create_template_directory(sandbox_root: &PathBuf) -> PathBuf {
    let template_root = sandbox_root.join("templates").join(TEMPLATE_ID);
    fs::create_dir_all(&template_root).expect("should create template directory");

    let template_yaml = r#"name: "Blank Workspace"
description: "A minimal workspace template"
version: "1.0.0"
"#;
    fs::write(template_root.join("template.yaml"), template_yaml)
        .expect("should write template.yaml");

    let blank_dir = template_root.join("blank");
    fs::create_dir_all(blank_dir.join("src")).expect("should create template src");
    fs::create_dir_all(blank_dir.join("tests")).expect("should create template tests");
    fs::create_dir_all(blank_dir.join("docs")).expect("should create template docs");

    template_root
}

fn run_cli_command(cmd: &str, args: &[&str], cwd: &PathBuf) -> std::process::Output {
    Command::new(cmd)
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("should execute command")
}

#[test]
fn test_build_workflow_succeeds_on_generated_workspace() {
    let sandbox_root = create_sandbox_directory("build-workflow");
    let workspace_dir = sandbox_root.join("TestWorkspace");

    let output = run_cli_command(
        "nfw",
        &[
            "new",
            "TestWorkspace",
            "--template",
            TEMPLATE_ID,
            "--no-input",
        ],
        &sandbox_root,
    );

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("nfw new failed: {}", stderr);
    }

    assert!(workspace_dir.is_dir(), "workspace should be created");

    let build_output = Command::new("make")
        .arg("build")
        .current_dir(&workspace_dir)
        .output()
        .expect("make build should execute");

    let stdout = String::from_utf8_lossy(&build_output.stdout);
    let stderr = String::from_utf8_lossy(&build_output.stderr);

    if !build_output.status.success() {
        eprintln!("make build failed:\nstdout: {}\nstderr: {}", stdout, stderr);
    }

    assert!(
        build_output.status.success(),
        "build should succeed on first run"
    );
}

#[test]
fn test_test_workflow_succeeds_on_generated_workspace() {
    let sandbox_root = create_sandbox_directory("test-workflow");
    let workspace_dir = sandbox_root.join("TestWorkspace");

    let output = run_cli_command(
        "nfw",
        &[
            "new",
            "TestWorkspace",
            "--template",
            TEMPLATE_ID,
            "--no-input",
        ],
        &sandbox_root,
    );

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("nfw new failed: {}", stderr);
    }

    assert!(workspace_dir.is_dir(), "workspace should be created");

    let test_output = Command::new("make")
        .arg("test")
        .current_dir(&workspace_dir)
        .output()
        .expect("make test should execute");

    let stdout = String::from_utf8_lossy(&test_output.stdout);
    let stderr = String::from_utf8_lossy(&test_output.stderr);

    if !test_output.status.success() {
        eprintln!("make test failed:\nstdout: {}\nstderr: {}", stdout, stderr);
    }

    assert!(
        test_output.status.success(),
        "test should succeed on first run"
    );
}

#[test]
fn test_build_failure_identifies_failing_project() {
    let sandbox_root = create_sandbox_directory("build-failure");
    let workspace_dir = sandbox_root.join("TestWorkspace");

    let output = run_cli_command(
        "nfw",
        &[
            "new",
            "TestWorkspace",
            "--template",
            TEMPLATE_ID,
            "--no-input",
        ],
        &sandbox_root,
    );

    assert!(workspace_dir.is_dir(), "workspace should be created");

    let broken_project_dir = workspace_dir.join("src").join("BrokenProject");
    fs::create_dir_all(&broken_project_dir).expect("should create broken project dir");

    let broken_csproj = r#"
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net999.0</TargetFramework>
  </PropertyGroup>
</Project>
"#;
    fs::write(
        broken_project_dir.join("BrokenProject.csproj"),
        broken_csproj,
    )
    .expect("should write broken csproj");

    let build_output = Command::new("make")
        .arg("build")
        .current_dir(&workspace_dir)
        .output()
        .expect("make build should execute");

    let stdout = String::from_utf8_lossy(&build_output.stdout);
    let stderr = String::from_utf8_lossy(&build_output.stderr);

    assert!(
        !build_output.status.success(),
        "build should fail with broken project"
    );
    assert!(
        stderr.contains("BrokenProject") || stdout.contains("BrokenProject"),
        "error output should identify failing project"
    );
}
