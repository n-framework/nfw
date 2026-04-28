use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn new_command_help_lists_workspace_argument_and_flags() {
    let sandbox_home = create_sandbox_directory("routing-help-home");
    let output = run_cli_command(&sandbox_home, &["new", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage: nfw new [OPTIONS] [workspace-name]"));
    assert!(stdout.contains("--template <template>"));
    assert!(stdout.contains("--no-input"));

    cleanup_sandbox_directory(&sandbox_home);
}

#[test]
fn new_command_accepts_no_input_as_presence_flag() {
    let sandbox_home = create_sandbox_directory("routing-no-input-home");
    let output = run_cli_command(&sandbox_home, &["new", "--no-input"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required input 'workspace-name' is missing"));
    assert!(!stderr.contains("unexpected argument '--no-input'"));

    cleanup_sandbox_directory(&sandbox_home);
}

fn run_cli_command(home: &Path, args: &[&str]) -> Output {
    let config_home = home.join(".config");
    fs::create_dir_all(&config_home).expect("config home directory should exist");

    Command::new(env!("CARGO_BIN_EXE_n-framework-nfw-cli"))
        .args(args)
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
