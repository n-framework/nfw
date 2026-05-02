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
    no_input: bool,
    schema_only: bool,
    from_schema: Option<&str>,
) -> HashMap<String, String> {
    let mut opts = HashMap::new();
    opts.insert("name".to_string(), name.to_string());
    if let Some(f) = feature {
        opts.insert("feature".to_string(), f.to_string());
    }
    if let Some(p) = properties {
        opts.insert("properties".to_string(), p.to_string());
    }
    if no_input {
        opts.insert("no-input".to_string(), "true".to_string());
    }
    if schema_only {
        opts.insert("schema-only".to_string(), "true".to_string());
    }
    if let Some(s) = from_schema {
        opts.insert("from-schema".to_string(), s.to_string());
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
fn schema_conflict_detection_test() {
    let sandbox = support::create_sandbox_directory("gen-entity-schema-conflict");

    fs::create_dir_all(sandbox.join("src/Application")).unwrap();
    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\nservices:\n  Application:\n    path: src/Application\n    template:\n      id: basic-api\n    modules:\n      - persistence\n",
    ).unwrap();

    let specs_dir = sandbox.join("src/Application/specs/features/Catalog/entities");
    fs::create_dir_all(&specs_dir).unwrap();
    fs::write(specs_dir.join("Product.yaml"), "entity: Product\nid_type: uuid\nentity_type: entity\nproperties:\n  - name: Dummy\n    type: string\n    nullable: false\n").unwrap();

    let opts = make_opts(
        "Product",
        Some("Catalog"),
        Some("Name:string"),
        true,
        false,
        None,
    );

    let result = run(&sandbox, opts);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("schema file already exists"),
        "Actual error: {}",
        err_msg
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn schema_only_generation_test() {
    let sandbox = support::create_sandbox_directory("gen-entity-schema-only");

    fs::create_dir_all(sandbox.join("src/Application")).unwrap();
    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\nservices:\n  Application:\n    path: src/Application\n    template:\n      id: basic-api\n    modules:\n      - persistence\n",
    ).unwrap();

    let opts = make_opts(
        "Product",
        Some("Catalog"),
        Some("Name:string"),
        true,
        true,
        None,
    );

    let result = run(&sandbox, opts);
    assert!(result.is_ok(), "Generation failed: {:?}", result.err());

    let schema_file = sandbox.join("src/Application/specs/features/Catalog/entities/Product.yaml");
    assert!(schema_file.exists());

    let content = fs::read_to_string(&schema_file).unwrap();
    assert!(content.contains("entity: Product"));

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn from_schema_generation_test() {
    let sandbox = support::create_sandbox_directory("gen-entity-from-schema");

    fs::create_dir_all(sandbox.join("src/Application")).unwrap();
    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\nservices:\n  Application:\n    path: src/Application\n    template:\n      id: basic-api\n    modules:\n      - persistence\n",
    ).unwrap();

    let specs_dir = sandbox.join("src/Application/specs/features/Catalog/entities");
    fs::create_dir_all(&specs_dir).unwrap();
    let schema_path = specs_dir.join("Product.yaml");
    fs::write(&schema_path, "entity: Product\nid_type: uuid\nentity_type: entity\nproperties:\n  - name: Title\n    type: string\n    nullable: false\n").unwrap();

    // Do not pass --properties, pass --from-schema
    let opts = make_opts(
        "Product",
        Some("Catalog"),
        None,
        true,
        false,
        Some(schema_path.to_str().unwrap()),
    );

    let result = run(&sandbox, opts);
    assert!(result.is_ok(), "Generation failed: {:?}", result.err());

    // Should read the schema ok without doing validation errors about missing properties
    support::cleanup_sandbox_directory(&sandbox);
}
