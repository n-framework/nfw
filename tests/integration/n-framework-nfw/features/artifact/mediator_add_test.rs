#[path = "../service_add/support.rs"]
mod support;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use n_framework_core_cli_abstractions::Command;
use n_framework_nfw_cli::commands::add::mediator::AddMediatorCliCommand;
use n_framework_nfw_cli::startup::cli_service_collection_factory::CliServiceCollectionFactory;

// Tests in this file must run sequentially because they temporarily mutate
// `std::env::current_dir()`, which is shared global process state.
static DIR_LOCK: Mutex<()> = Mutex::new(());

struct TestCommand {
    opts: HashMap<String, String>,
}

impl Command for TestCommand {
    fn name(&self) -> &str {
        "mediator"
    }
    fn args(&self) -> &[String] {
        &[]
    }
    fn option(&self, name: &str) -> Option<&str> {
        self.opts.get(name).map(|s| s.as_str())
    }
}

fn setup_mediator_workspace(sandbox: &Path) {
    fs::write(
        sandbox.join("nfw.yaml"),
        r#"
workspace:
  name: Test
  namespace: TestApp
services:
  TestService:
    path: src/TestService
    template:
      id: dotnet-service
template_sources:
  local: "templates"
"#,
    )
    .expect("failed to write nfw.yaml");

    let tpl_dir = sandbox
        .join("templates")
        .join("dotnet-service")
        .join("mediator");
    fs::create_dir_all(&tpl_dir).expect("failed to create template dir");
    fs::write(
        tpl_dir.join("template.yaml"),
        r#"
id: dotnet-service/mediator
steps:
  - action: render
    source: reg.cs.tera
    destination: "{{ Name }}.Mediator.cs"
"#,
    )
    .expect("failed to write template.yaml");
    fs::write(tpl_dir.join("reg.cs.tera"), "// registered").expect("failed to write tera");

    fs::create_dir_all(sandbox.join("src/TestService")).unwrap();
}

#[test]
fn add_mediator_updates_nfw_yaml_and_renders_template() {
    let sandbox = support::create_sandbox_directory("add-mediator-integration");
    setup_mediator_workspace(&sandbox);

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "TestService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddMediatorCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "add mediator failed: {:?}", result.err());

    // Check nfw.yaml updated using YAML parsing
    let nfw_yaml_content = fs::read_to_string(sandbox.join("nfw.yaml")).unwrap();
    let yaml: serde_yaml::Value = serde_yaml::from_str(&nfw_yaml_content).unwrap();

    let modules = yaml
        .get("services")
        .and_then(|s| s.get("TestService"))
        .and_then(|details| details.get("modules"))
        .and_then(|m| m.as_sequence())
        .expect("nfw.yaml should have modules sequence for TestService");

    assert!(
        modules.iter().any(|m| m.as_str() == Some("mediator")),
        "nfw.yaml modules should contain 'mediator'"
    );

    // Check file rendered
    assert!(
        sandbox
            .join("src/TestService/TestService.Mediator.cs")
            .exists()
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn add_mediator_rolls_back_yaml_if_template_execution_fails() {
    let sandbox = support::create_sandbox_directory("add-mediator-rollback");
    setup_mediator_workspace(&sandbox);

    // Corrupt the template to force a failure (e.g., non-existent source)
    let tpl_yaml_path = sandbox.join("templates/dotnet-service/mediator/template.yaml");
    fs::write(
        tpl_yaml_path,
        r#"
id: dotnet-service/mediator
steps:
  - action: render
    source: missing.tera
    destination: "Failed.cs"
"#,
    )
    .unwrap();

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "TestService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddMediatorCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(
        result.is_err(),
        "Expected error due to missing template source"
    );

    // Verify nfw.yaml was NOT updated
    let nfw_yaml_content = fs::read_to_string(sandbox.join("nfw.yaml")).unwrap();
    let yaml: serde_yaml::Value = serde_yaml::from_str(&nfw_yaml_content).unwrap();

    let modules = yaml
        .get("services")
        .and_then(|s| s.get("TestService"))
        .and_then(|details| details.get("modules"));

    assert!(
        modules.is_none()
            || (modules
                .unwrap()
                .as_sequence()
                .map_or(true, |s| s.is_empty())),
        "nfw.yaml should NOT have been updated with mediator module on failure"
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn add_mediator_fails_if_service_not_found() {
    let sandbox = support::create_sandbox_directory("add-mediator-fail");
    setup_mediator_workspace(&sandbox);

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "UnknownService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddMediatorCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found in workspace"));

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn add_mediator_preserves_comments_in_nfw_yaml() {
    let sandbox = support::create_sandbox_directory("add-mediator-comments");
    fs::write(
        sandbox.join("nfw.yaml"),
        r#"# Top level comment
workspace:
  name: Test
  namespace: TestApp # Inline comment

# Section comment
services:
  # Service block comment
  TestService:
    path: src/TestService
    template:
      id: dotnet-service
template_sources:
  local: "templates"
"#,
    )
    .expect("failed to write nfw.yaml");

    // Setup template
    let tpl_dir = sandbox
        .join("templates")
        .join("dotnet-service")
        .join("mediator");
    fs::create_dir_all(&tpl_dir).unwrap();
    fs::write(
        tpl_dir.join("template.yaml"),
        "id: dotnet-service/mediator\nsteps: []",
    )
    .unwrap();
    fs::create_dir_all(sandbox.join("src/TestService")).unwrap();

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "TestService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddMediatorCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "add mediator failed: {:?}", result.err());

    let content = fs::read_to_string(sandbox.join("nfw.yaml")).unwrap();

    // Check additions
    assert!(content.contains("- mediator"), "mediator module not added");

    // Check preservation
    assert!(
        content.contains("# Top level comment"),
        "Top level comment lost"
    );
    assert!(content.contains("# Inline comment"), "Inline comment lost");
    assert!(
        content.contains("# Section comment"),
        "Section comment lost"
    );
    assert!(
        content.contains("# Service block comment"),
        "Service block comment lost"
    );

    support::cleanup_sandbox_directory(&sandbox);
}
