#[path = "../service_add/support.rs"]
mod support;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use n_framework_core_cli_abstractions::Command;
use n_framework_nfw_cli::commands::add::webapi::AddWebApiCliCommand;
use n_framework_nfw_cli::startup::cli_service_collection_factory::CliServiceCollectionFactory;

static DIR_LOCK: Mutex<()> = Mutex::new(());

struct TestCommand {
    opts: HashMap<String, String>,
}

impl Command for TestCommand {
    fn name(&self) -> &str {
        "webapi"
    }
    fn args(&self) -> &[String] {
        &[]
    }
    fn option(&self, name: &str) -> Option<&str> {
        self.opts.get(name).map(|s| s.as_str())
    }
}

fn setup_webapi_workspace(sandbox: &Path) {
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

    let root_tpl_dir = sandbox.join("templates").join("dotnet-service");
    fs::create_dir_all(&root_tpl_dir).expect("failed to create root template dir");
    fs::write(
        root_tpl_dir.join("template.yaml"),
        "id: dotnet-service\nname: Dotnet Service\nversion: 1.0.0\ngenerators:\n  webapi: webapi\n",
    )
    .expect("failed to write root template.yaml");

    let tpl_dir = root_tpl_dir.join("webapi");
    fs::create_dir_all(&tpl_dir).expect("failed to create sub-template dir");
    fs::write(
        tpl_dir.join("template.yaml"),
        r#"
id: dotnet-service/webapi
steps:
  - action: render
    source: Program.cs.tera
    destination: "Program.cs"
"#,
    )
    .expect("failed to write sub-template template.yaml");
    fs::write(
        tpl_dir.join("Program.cs.tera"),
        "// Program.cs for {{ Name }}",
    )
    .expect("failed to write tera");

    fs::create_dir_all(sandbox.join("src/TestService")).unwrap();
}

#[test]
fn given_valid_service_when_add_webapi_then_updates_yaml_and_renders_template() {
    let sandbox = support::create_sandbox_directory("add-webapi-integration");
    setup_webapi_workspace(&sandbox);

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "TestService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddWebApiCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "add webapi failed: {:?}", result.err());

    let nfw_yaml_content = fs::read_to_string(sandbox.join("nfw.yaml")).unwrap();
    let yaml: serde_yaml::Value = serde_yaml::from_str(&nfw_yaml_content).unwrap();

    let modules = yaml
        .get("services")
        .and_then(|s| s.get("TestService"))
        .and_then(|details| details.get("modules"))
        .and_then(|m| m.as_sequence())
        .expect("nfw.yaml should have modules sequence for TestService");

    assert!(
        modules.iter().any(|m| m.as_str() == Some("webapi")),
        "nfw.yaml modules should contain 'webapi'"
    );

    assert!(
        sandbox
            .join("src/TestService/Program.cs")
            .exists()
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn given_template_execution_fails_when_add_webapi_then_rolls_back_yaml() {
    let sandbox = support::create_sandbox_directory("add-webapi-rollback");
    setup_webapi_workspace(&sandbox);

    let tpl_yaml_path = sandbox.join("templates/dotnet-service/webapi/template.yaml");
    fs::write(
        tpl_yaml_path,
        r#"
id: dotnet-service/webapi
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

    let result = AddWebApiCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(
        result.is_err(),
        "Expected error due to missing template source"
    );

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
        "nfw.yaml should NOT have been updated with webapi module on failure"
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn given_invalid_service_name_when_add_webapi_then_returns_not_found_error() {
    let sandbox = support::create_sandbox_directory("add-webapi-fail");
    setup_webapi_workspace(&sandbox);

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "UnknownService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddWebApiCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found in workspace"));

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn given_yaml_with_comments_when_add_webapi_then_preserves_comments() {
    let sandbox = support::create_sandbox_directory("add-webapi-comments");
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

    let root_tpl_dir = sandbox.join("templates").join("dotnet-service");
    fs::create_dir_all(&root_tpl_dir).unwrap();
    fs::write(
        root_tpl_dir.join("template.yaml"),
        "id: dotnet-service\nname: Dotnet Service\nversion: 1.0.0\ngenerators:\n  webapi: webapi\n",
    )
    .unwrap();

    let tpl_dir = root_tpl_dir.join("webapi");
    fs::create_dir_all(&tpl_dir).unwrap();
    fs::write(
        tpl_dir.join("template.yaml"),
        "id: dotnet-service/webapi\nsteps: []",
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

    let result = AddWebApiCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "add webapi failed: {:?}", result.err());

    let content = fs::read_to_string(sandbox.join("nfw.yaml")).unwrap();

    assert!(
        content.contains("- webapi"),
        "webapi module not added"
    );

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

#[test]
fn given_webapi_exists_when_add_webapi_then_returns_duplicate_error() {
    let sandbox = support::create_sandbox_directory("add-webapi-duplicate");
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
    modules:
      - webapi
template_sources:
  local: "templates"
"#,
    )
    .expect("failed to write nfw.yaml");

    let root_tpl_dir = sandbox.join("templates").join("dotnet-service");
    fs::create_dir_all(&root_tpl_dir).unwrap();
    fs::write(
        root_tpl_dir.join("template.yaml"),
        "id: dotnet-service\nname: Dotnet Service\nversion: 1.0.0\ngenerators:\n  webapi: webapi\n",
    )
    .unwrap();

    let tpl_dir = root_tpl_dir.join("webapi");
    fs::create_dir_all(&tpl_dir).unwrap();
    fs::write(
        tpl_dir.join("template.yaml"),
        r#"
id: dotnet-service/webapi
steps:
  - action: render
    source: Program.cs.tera
    destination: "Program.cs"
"#,
    )
    .unwrap();
    fs::write(
        tpl_dir.join("Program.cs.tera"),
        "// Program.cs for {{ Name }}",
    )
    .expect("failed to write tera");
    fs::create_dir_all(sandbox.join("src/TestService")).unwrap();

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "TestService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let _result = AddWebApiCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    let nfw_yaml_content = fs::read_to_string(sandbox.join("nfw.yaml")).unwrap();
    let yaml: serde_yaml::Value = serde_yaml::from_str(&nfw_yaml_content).unwrap();

    let modules = yaml
        .get("services")
        .and_then(|s| s.get("TestService"))
        .and_then(|details| details.get("modules"))
        .and_then(|m| m.as_sequence())
        .expect("modules should exist");

    let webapi_count = modules
        .iter()
        .filter(|m| m.as_str() == Some("webapi"))
        .count();

    assert!(
        webapi_count <= 1,
        "webapi module should not be duplicated, found {} entries",
        webapi_count
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn given_no_nfw_yaml_when_add_webapi_then_returns_workspace_error() {
    let sandbox = support::create_sandbox_directory("add-webapi-missing-yaml");

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "TestService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddWebApiCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.to_lowercase().contains("nfw.yaml") || err_msg.to_lowercase().contains("workspace"),
        "Error should mention nfw.yaml or workspace configuration"
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn given_malformed_yaml_when_add_webapi_then_returns_parse_error() {
    let sandbox = support::create_sandbox_directory("add-webapi-malformed-yaml");

    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  namespace: Test\n  services:\n    BadYaml [[[",
    )
    .unwrap();

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "TestService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddWebApiCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.to_lowercase().contains("yaml") || err_msg.to_lowercase().contains("parse"),
        "Error should mention YAML parsing issue"
    );

    support::cleanup_sandbox_directory(&sandbox);
}
