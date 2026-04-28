#[path = "../service_add/support.rs"]
mod support;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use n_framework_core_cli_abstractions::Command;
use n_framework_nfw_cli::runtime::nfw_cli_runtime::handle_add_mediator;
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

    let result = handle_add_mediator(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "add mediator failed: {:?}", result.err());

    // Check nfw.yaml updated
    let nfw_yaml = fs::read_to_string(sandbox.join("nfw.yaml")).unwrap();
    assert!(
        nfw_yaml.contains("- mediator"),
        "nfw.yaml should contain mediator module"
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

    let result = handle_add_mediator(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found in workspace"));

    support::cleanup_sandbox_directory(&sandbox);
}
