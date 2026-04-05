use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn root_help_lists_check_command() {
    let sandbox_home = create_sandbox_directory("check-routing-help-home");
    let output = run_cli_command(&sandbox_home, &["--help"], None);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("check"));

    cleanup_sandbox_directory(&sandbox_home);
}

#[test]
fn check_command_executes_and_reports_workspace_error() {
    let sandbox_home = create_sandbox_directory("check-routing-command-home");
    let workspace = create_sandbox_directory("check-routing-command-workspace");
    let output = run_cli_command(&sandbox_home, &["check"], Some(&workspace));
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("could not find nfw.yaml"));

    cleanup_sandbox_directory(&workspace);
    cleanup_sandbox_directory(&sandbox_home);
}

fn run_cli_command(home: &Path, args: &[&str], current_dir: Option<&Path>) -> Output {
    let config_home = home.join(".config");
    fs::create_dir_all(&config_home).expect("config home directory should exist");

    let mut command = Command::new(env!("CARGO_BIN_EXE_nframework-nfw-cli"));
    command
        .args(args)
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", &config_home);

    if let Some(path) = current_dir {
        command.current_dir(path);
    }

    command.output().expect("cli process should execute")
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
