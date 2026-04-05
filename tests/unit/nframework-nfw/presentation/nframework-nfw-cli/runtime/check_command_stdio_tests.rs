use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn violation_details_are_printed_to_stderr_not_stdout() {
    let sandbox_home = create_sandbox_directory("check-stdio-home");
    let workspace = create_sandbox_directory("check-stdio-workspace");
    fs::write(
        workspace.join("nfw.yaml"),
        "workspace:\n  name: TestWorkspace\n",
    )
    .expect("workspace marker should be created");

    let project_dir = workspace.join("src/domain");
    fs::create_dir_all(&project_dir).expect("project directory should be created");
    fs::write(
        project_dir.join("NFramework.Domain.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">
  <ItemGroup>
    <ProjectReference Include=\"../application/NFramework.Application.csproj\" />
  </ItemGroup>
</Project>\n",
    )
    .expect("project file should be written");
    fs::write(
        project_dir.join("DomainModel.cs"),
        "namespace NFramework.Domain;\npublic class DomainModel {}\n",
    )
    .expect("source file should be written");

    let output = run_cli_command(&sandbox_home, &workspace, &["check"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("type=project_reference"));
    assert!(!stdout.contains("type=project_reference"));

    cleanup_sandbox_directory(&workspace);
    cleanup_sandbox_directory(&sandbox_home);
}

fn run_cli_command(home: &Path, workspace: &Path, args: &[&str]) -> Output {
    let config_home = home.join(".config");
    fs::create_dir_all(&config_home).expect("config home directory should exist");

    Command::new(env!("CARGO_BIN_EXE_nframework-nfw-cli"))
        .args(args)
        .current_dir(workspace)
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", &config_home)
        .output()
        .expect("cli process should execute")
}

fn create_sandbox_directory(test_name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("nfw-{test_name}-{unique}"));
    fs::create_dir_all(&path).expect("sandbox directory should be created");
    path
}

fn cleanup_sandbox_directory(path: &Path) {
    let _ = fs::remove_dir_all(path);
}
