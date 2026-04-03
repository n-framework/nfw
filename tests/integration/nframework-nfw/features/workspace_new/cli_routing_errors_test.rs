use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn unknown_subcommand_returns_actionable_error() {
    let sandbox_home = create_sandbox_directory("routing-unknown-subcommand-home");
    let output = run_cli_command(&sandbox_home, &["unknown"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unrecognized subcommand"));

    cleanup_sandbox_directory(&sandbox_home);
}

#[test]
fn unknown_option_for_new_command_returns_actionable_error() {
    let sandbox_home = create_sandbox_directory("routing-unknown-option-home");
    let output = run_cli_command(&sandbox_home, &["new", "billing", "--unsupported"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unexpected argument '--unsupported'"));

    cleanup_sandbox_directory(&sandbox_home);
}

#[test]
fn valid_new_command_shape_routes_to_workspace_handler() {
    let sandbox_home = create_sandbox_directory("routing-valid-shape-home");
    write_local_official_source_config(&sandbox_home);
    let output = run_cli_command(
        &sandbox_home,
        &[
            "new",
            "billing-platform",
            "--template",
            "does-not-exist/template",
            "--no-input",
        ],
    );

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let is_expected_workspace_failure = stderr.contains("template 'does-not-exist/template' was not found")
        || stderr.contains("workspace initialization failed");
    assert!(
        is_expected_workspace_failure,
        "stderr was: {stderr}"
    );
    assert!(!stderr.contains("unsupported command"));

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

fn write_local_official_source_config(home: &Path) {
    let config_home = home.join(".config").join("nfw");
    fs::create_dir_all(&config_home).expect("nfw config directory should exist");

    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../");
    let local_templates_repository = workspace_root.join("../nfw-templates");
    let source_url = local_templates_repository
        .canonicalize()
        .expect("local nfw-templates repository should exist");

    let config = format!(
        "sources:\n  - name: official\n    url: {}\n    enabled: true\n",
        source_url.to_string_lossy()
    );
    fs::write(config_home.join("sources.yaml"), config)
        .expect("sources.yaml should be written for test sandbox");
}

fn cleanup_sandbox_directory(path: &Path) {
    let _ = fs::remove_dir_all(path);
}
