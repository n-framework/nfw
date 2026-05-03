#[path = "../service_add/support.rs"]
mod support;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use n_framework_core_cli_abstractions::Command;
use n_framework_nfw_cli::commands::r#gen::entity::handler::GenEntityCliCommand;
use n_framework_nfw_cli::startup::cli_service_collection_factory::CliServiceCollectionFactory;

static DIR_LOCK: Mutex<()> = Mutex::new(());

struct TestCommand {
    opts: HashMap<String, String>,
}

impl Command for TestCommand {
    fn name(&self) -> &str {
        "entity"
    }
    fn args(&self) -> &[String] {
        &[]
    }
    fn option(&self, name: &str) -> Option<&str> {
        self.opts.get(name).map(|s| s.as_str())
    }
}

fn make_opts(
    name: &str,
    feature: Option<&str>,
    properties: Option<&str>,
    from_schema: Option<&str>,
    no_input: bool,
) -> HashMap<String, String> {
    let mut opts = HashMap::new();
    opts.insert("name".to_string(), name.to_string());
    if let Some(f) = feature {
        opts.insert("feature".to_string(), f.to_string());
    }
    if let Some(p) = properties {
        opts.insert("properties".to_string(), p.to_string());
    }
    if let Some(s) = from_schema {
        opts.insert("from-schema".to_string(), s.to_string());
    }
    if no_input {
        opts.insert("no-input".to_string(), "true".to_string());
    }
    opts
}

fn run(sandbox: &Path, opts: HashMap<String, String>) -> Result<(), String> {
    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().expect("should have current dir");
    std::env::set_current_dir(sandbox).expect("should set current dir");
    let result = GenEntityCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).expect("should restore current dir");
    result.map_err(|e| e.to_string())
}

#[test]
fn fails_on_invalid_entity_name() {
    let sandbox = support::create_sandbox_directory("gen-entity-invalid-name");

    fs::create_dir_all(sandbox.join("src/Application/Features/Catalog")).unwrap();
    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\nservices:\n  Application:\n    path: src/Application\n    template:\n      id: basic-api\n    modules:\n      - persistence\n",
    ).unwrap();

    let opts = make_opts(
        "invalid_name",
        Some("Catalog"),
        Some("Name:string"),
        None,
        true,
    );

    let result = run(&sandbox, opts);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("invalid entity name"),
        "Actual error: {}",
        err_msg
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn fails_on_invalid_property_syntax() {
    let sandbox = support::create_sandbox_directory("gen-entity-invalid-prop");

    fs::create_dir_all(sandbox.join("src/Application/Features/Catalog")).unwrap();
    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\nservices:\n  Application:\n    path: src/Application\n    template:\n      id: basic-api\n    modules:\n      - persistence\n",
    ).unwrap();

    let opts = make_opts("Product", Some("Catalog"), Some("Name-string"), None, true); // Missing colon

    let result = run(&sandbox, opts);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("invalid property syntax"),
        "Actual error: {}",
        err_msg
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn fails_when_no_persistence_module_found() {
    let sandbox = support::create_sandbox_directory("gen-entity-no-persist");

    fs::create_dir_all(sandbox.join("src/Application/Features/Catalog")).unwrap();
    // Missing persistence module
    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\nservices:\n  Application:\n    path: src/Application\n    template:\n      id: basic-api\n",
    ).unwrap();

    let opts = make_opts("Product", Some("Catalog"), Some("Name:string"), None, true);

    let result = run(&sandbox, opts);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("does not have the persistence module"),
        "Actual error: {}",
        err_msg
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn fails_on_missing_schema_file() {
    let sandbox = support::create_sandbox_directory("gen-entity-missing-schema");

    fs::create_dir_all(sandbox.join("src/Application/Features/Catalog")).unwrap();
    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\nservices:\n  Application:\n    path: src/Application\n    template:\n      id: basic-api\n    modules:\n      - persistence\n",
    ).unwrap();

    let opts = make_opts(
        "Product",
        Some("Catalog"),
        Some("Name:string"),
        Some("missing.yaml"),
        true,
    );

    let result = run(&sandbox, opts);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("schema file not found"),
        "Actual error: {}",
        err_msg
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn fails_on_invalid_schema_yaml() {
    let sandbox = support::create_sandbox_directory("gen-entity-invalid-schema-yaml");

    fs::create_dir_all(sandbox.join("src/Application/Features/Catalog")).unwrap();
    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\nservices:\n  Application:\n    path: src/Application\n    template:\n      id: basic-api\n    modules:\n      - persistence\n",
    ).unwrap();

    let schema_path = sandbox.join("invalid.yaml");
    fs::write(&schema_path, "invalid: yaml: [ : }").unwrap();

    let opts = make_opts(
        "Product",
        Some("Catalog"),
        Some("Name:string"),
        Some(schema_path.to_str().unwrap()),
        true,
    );

    let result = run(&sandbox, opts);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("invalid schema content"),
        "Actual error: {}",
        err_msg
    );

    support::cleanup_sandbox_directory(&sandbox);
}
