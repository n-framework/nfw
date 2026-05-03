#[path = "../service_add/support.rs"]
mod support;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use n_framework_core_cli_abstractions::Command;
use n_framework_nfw_cli::commands::r#gen::entity::handler::GenEntityCliCommand;
use n_framework_nfw_cli::startup::cli_service_collection_factory::CliServiceCollectionFactory;

// Tests in this file must run sequentially because they temporarily mutate
// `std::env::current_dir()`, which is shared global process state.
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
    opts
}

fn run(sandbox: &Path, opts: HashMap<String, String>) -> Result<(), String> {
    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().expect("should have current dir");
    std::env::set_current_dir(sandbox).expect("should set current dir");
    let result = GenEntityCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).expect("should restore current dir");

    // Convert CliError to String if Err
    result.map_err(|e| e.to_string())
}

#[test]
fn generates_entity_successfully() {
    let sandbox = support::create_sandbox_directory("gen-entity-success");

    // Write a dummy workspace
    fs::create_dir_all(sandbox.join("src/Application/Features/Catalog")).unwrap();
    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\nservices:\n  Application:\n    path: src/Application\n    template:\n      id: mock-entity-template\n    modules:\n      - persistence\ntemplate_sources:\n  local: \"templates\"\n",
    ).expect("failed to write nfw.yaml");

    let root_tpl_dir = sandbox.join("templates").join("mock-entity-template");
    fs::create_dir_all(&root_tpl_dir).expect("failed to create root template dir");
    fs::write(
        root_tpl_dir.join("template.yaml"),
        "id: mock-entity-template\nname: Mock Entity\nversion: 1.0.0\ngenerators:\n  entity: entity\n",
    )
    .expect("failed to write root template.yaml");

    let tpl_dir = root_tpl_dir.join("entity");
    fs::create_dir_all(&tpl_dir).expect("failed to create entity template dir");
    fs::write(
        tpl_dir.join("template.yaml"),
        "id: mock-entity-template/entity\nname: Entity\nsteps:\n  - action: render\n    source: 'Entity.Nfw.g.cs.tera'\n    destination: 'src/{{ Service }}.Core.Domain/Features/{{ Feature }}/Entities/{{ Name }}.Nfw.g.cs'\n  - action: render\n    source: 'Entity.cs.tera'\n    destination: 'src/{{ Service }}.Core.Domain/Features/{{ Feature }}/Entities/{{ Name }}.cs'\n",
    ).expect("failed to write template.yaml");

    fs::write(
        tpl_dir.join("Entity.Nfw.g.cs.tera"),
        "public partial class {{ Name }} {}",
    )
    .expect("failed to write Tera file");

    fs::write(
        tpl_dir.join("Entity.cs.tera"),
        "public partial class {{ Name }} {}",
    )
    .expect("failed to write Tera file");

    let opts = make_opts(
        "Product",
        Some("Catalog"),
        Some("Name:string,Price:decimal"),
        true,
    );

    let result = run(&sandbox, opts);
    assert!(result.is_ok(), "Generation failed: {:?}", result.err());

    let schema_file = sandbox.join("src/Application/specs/features/Catalog/entities/Product.yaml");
    assert!(
        schema_file.exists(),
        "Schema file not found at {:?}",
        schema_file
    );

    let content = fs::read_to_string(&schema_file).unwrap();
    assert!(content.contains("entity: Product"));
    assert!(content.contains("entity_type: entity"));
    assert!(content.contains("id_type: uuid"));
    assert!(content.contains("properties:"));
    assert!(content.contains("name: Name"));
    assert!(content.contains("type: string"));
    assert!(content.contains("name: Price"));
    assert!(content.contains("type: decimal"));

    support::cleanup_sandbox_directory(&sandbox);
}
