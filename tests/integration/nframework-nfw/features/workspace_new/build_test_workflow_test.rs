use std::fs;
use std::path::PathBuf;
use std::process::Command;

const TEMPLATE_ID: &str = "official/blank-workspace";

fn create_sandbox_directory(name: &str) -> PathBuf {
    let tmp_dir = std::env::temp_dir().join(format!("nfw-test-{}", name));

    // Clean up any existing directory
    if let Err(e) = fs::remove_dir_all(&tmp_dir) {
        // Ignore errors if directory doesn't exist
        if e.kind() != std::io::ErrorKind::NotFound {
            panic!("Failed to remove existing sandbox directory at {}: {}", tmp_dir.display(), e);
        }
    }

    // Create the directory with proper error handling
    fs::create_dir_all(&tmp_dir).unwrap_or_else(|e| {
        panic!(
            "Failed to create sandbox directory at {}: {}. \
             Check permissions and disk space.",
            tmp_dir.display(), e
        )
    });

    tmp_dir
}

fn run_cli_command(cmd: &str, args: &[&str], cwd: &PathBuf) -> std::process::Output {
    Command::new(cmd)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap_or_else(|e| {
            panic!(
                "Failed to execute command '{}': {}. \
                 Check that the command exists in PATH.",
                cmd, e
            )
        })
}

// Note: Removed unused create_template_directory function (G9 - Remove Dead Code)

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
        .unwrap_or_else(|e| {
            panic!(
                "Failed to execute 'make build' in directory {}: {}",
                workspace_dir.display(), e
            )
        });

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
        .unwrap_or_else(|e| {
            panic!(
                "Failed to execute 'make test' in directory {}: {}",
                workspace_dir.display(), e
            )
        });

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
    fs::create_dir_all(&broken_project_dir).unwrap_or_else(|e| {
        panic!(
            "Failed to create broken project directory at {}: {}",
            broken_project_dir.display(), e
        )
    });

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
    .unwrap_or_else(|e| {
        panic!(
            "Failed to write broken csproj to {}: {}",
            broken_project_dir.join("BrokenProject.csproj").display(),
            e
        )
    });

    let build_output = Command::new("make")
        .arg("build")
        .current_dir(&workspace_dir)
        .output()
        .unwrap_or_else(|e| {
            panic!(
                "Failed to execute 'make build' in directory {}: {}",
                workspace_dir.display(), e
            )
        });

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
