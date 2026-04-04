use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn add_service_help_lists_expected_options() {
    let sandbox_home = create_sandbox_directory("add-service-help-home");
    let output = run_cli_command(&sandbox_home, &["add", "service", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nfw add service"));
    assert!(stdout.contains("--template <template>"));
    assert!(stdout.contains("--no-input"));

    cleanup_sandbox_directory(&sandbox_home);
}

#[test]
fn add_service_non_interactive_requires_template() {
    let sandbox_home = create_sandbox_directory("add-service-no-input-home");
    let output = run_cli_command(&sandbox_home, &["add", "service", "Orders", "--no-input"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required input 'template' is missing"));

    cleanup_sandbox_directory(&sandbox_home);
}

fn run_cli_command(home: &Path, args: &[&str]) -> Output {
    let config_home = home.join(".config");
    fs::create_dir_all(&config_home).expect("config home directory should exist");

    Command::new(env!("CARGO_BIN_EXE_nframework-nfw-cli"))
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
