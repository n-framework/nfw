#[path = "../service_add/support.rs"]
mod support;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use n_framework_core_cli_abstractions::Command;
use n_framework_nfw_cli::commands::add::persistence::AddPersistenceCliCommand;
use n_framework_nfw_cli::startup::cli_service_collection_factory::CliServiceCollectionFactory;

static DIR_LOCK: Mutex<()> = Mutex::new(());

struct TestCommand {
    opts: HashMap<String, String>,
}

impl Command for TestCommand {
    fn name(&self) -> &str {
        "persistence"
    }
    fn args(&self) -> &[String] {
        &[]
    }
    fn option(&self, name: &str) -> Option<&str> {
        self.opts.get(name).map(|s| s.as_str())
    }
}

fn setup_persistence_workspace(sandbox: &Path) {
    fs::write(
        sandbox.join("nfw.yaml"),
        r#"
workspace:
  name: Test
  namespace: TestApp
services:
  TestService:
    path: src/TestService
    generator:
      id: dotnet-service
generator_sources:
  local: "generators"
"#,
    )
    .expect("failed to write nfw.yaml");

    let root_tpl_dir = sandbox.join("generators").join("dotnet-service");
    fs::create_dir_all(&root_tpl_dir).expect("failed to create root generator dir");
    fs::write(
        root_tpl_dir.join("nfw.generator.yaml"),
        "id: dotnet-service\nname: Dotnet Service\nversion: 1.0.0\ngenerators:\n  persistence: persistence\n  webapi: webapi\n",
    )
    .expect("failed to write root generator.yaml");

    // Scaffold a minimal webapi sub-generator so list_presentation_layers can derive the path.
    let webapi_tpl_dir = root_tpl_dir.join("webapi");
    fs::create_dir_all(&webapi_tpl_dir).expect("failed to create webapi generator dir");
    fs::write(
        webapi_tpl_dir.join("nfw.generator.yaml"),
        "id: dotnet-service/webapi\nsteps:\n  - action: render\n    source: \"Startup.cs.tera\"\n    destination: \"src/presentation/{{ Service }}.Presentation.WebApi/Startup.cs\"\n",
    )
    .expect("failed to write webapi generator.yaml");
    fs::write(webapi_tpl_dir.join("Startup.cs.tera"), "// startup")
        .expect("failed to write webapi tera");

    let tpl_dir = root_tpl_dir.join("persistence");
    fs::create_dir_all(&tpl_dir).expect("failed to create sub-generator dir");
    fs::write(
        tpl_dir.join("nfw.generator.yaml"),
        r#"
id: dotnet-service/persistence
steps:
  - action: render
    source: DbContext.cs.tera
    destination: "{{ Name }}DbContext.cs"
"#,
    )
    .expect("failed to write sub-generator generator.yaml");
    fs::write(
        tpl_dir.join("DbContext.cs.tera"),
        "// DbContext for {{ Name }}",
    )
    .expect("failed to write tera");

    fs::create_dir_all(
        sandbox.join("src/TestService/src/presentation/TestService.Presentation.WebApi"),
    )
    .unwrap();
}

#[test]
fn add_persistence_updates_nfw_yaml_and_renders_generator() {
    let sandbox = support::create_sandbox_directory("add-persistence-integration");
    setup_persistence_workspace(&sandbox);

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "TestService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddPersistenceCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "add persistence failed: {:?}", result.err());

    let nfw_yaml_content = fs::read_to_string(sandbox.join("nfw.yaml")).unwrap();
    let yaml: serde_yaml::Value = serde_yaml::from_str(&nfw_yaml_content).unwrap();

    let modules = yaml
        .get("services")
        .and_then(|s| s.get("TestService"))
        .and_then(|details| details.get("modules"))
        .and_then(|m| m.as_sequence())
        .expect("nfw.yaml should have modules sequence for TestService");

    assert!(
        modules.iter().any(|m| m.as_str() == Some("persistence")),
        "nfw.yaml modules should contain 'persistence'"
    );

    assert!(
        sandbox
            .join("src/TestService/TestServiceDbContext.cs")
            .exists()
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn add_persistence_rolls_back_yaml_if_generator_execution_fails() {
    let sandbox = support::create_sandbox_directory("add-persistence-rollback");
    setup_persistence_workspace(&sandbox);

    let tpl_yaml_path = sandbox.join("generators/dotnet-service/persistence/nfw.generator.yaml");
    fs::write(
        tpl_yaml_path,
        r#"
id: dotnet-service/persistence
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

    let result = AddPersistenceCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(
        result.is_err(),
        "Expected error due to missing generator source"
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
        "nfw.yaml should NOT have been updated with persistence module on failure"
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn add_persistence_fails_if_service_not_found() {
    let sandbox = support::create_sandbox_directory("add-persistence-fail");
    setup_persistence_workspace(&sandbox);

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "UnknownService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddPersistenceCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found in workspace"));

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn add_persistence_preserves_comments_in_nfw_yaml() {
    let sandbox = support::create_sandbox_directory("add-persistence-comments");
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
    generator:
      id: dotnet-service
generator_sources:
  local: "generators"
"#,
    )
    .expect("failed to write nfw.yaml");

    let root_tpl_dir = sandbox.join("generators").join("dotnet-service");
    fs::create_dir_all(&root_tpl_dir).unwrap();
    fs::write(
        root_tpl_dir.join("nfw.generator.yaml"),
        "id: dotnet-service\nname: Dotnet Service\nversion: 1.0.0\ngenerators:\n  persistence: persistence\n  webapi: webapi\n",
    )
    .unwrap();

    let webapi_tpl_dir = root_tpl_dir.join("webapi");
    fs::create_dir_all(&webapi_tpl_dir).unwrap();
    fs::write(
        webapi_tpl_dir.join("nfw.generator.yaml"),
        "id: dotnet-service/webapi\nsteps:\n  - action: render\n    source: \"Startup.cs.tera\"\n    destination: \"src/presentation/{{ Service }}.Presentation.WebApi/Startup.cs\"\n",
    )
    .unwrap();
    fs::write(webapi_tpl_dir.join("Startup.cs.tera"), "// startup").unwrap();

    let tpl_dir = root_tpl_dir.join("persistence");
    fs::create_dir_all(&tpl_dir).unwrap();
    fs::write(
        tpl_dir.join("nfw.generator.yaml"),
        "id: dotnet-service/persistence\nsteps:\n  - action: run_command\n    command: \"echo done\"\n",
    )
    .unwrap();
    fs::create_dir_all(
        sandbox.join("src/TestService/src/presentation/TestService.Presentation.WebApi"),
    )
    .unwrap();

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "TestService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let result = AddPersistenceCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "add persistence failed: {:?}", result.err());

    let content = fs::read_to_string(sandbox.join("nfw.yaml")).unwrap();

    assert!(
        content.contains("- persistence"),
        "persistence module not added"
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
fn add_persistence_detects_existing_persistence_module() {
    let sandbox = support::create_sandbox_directory("add-persistence-duplicate");
    fs::write(
        sandbox.join("nfw.yaml"),
        r#"
workspace:
  name: Test
  namespace: TestApp
services:
  TestService:
    path: src/TestService
    generator:
      id: dotnet-service
    modules:
      - persistence
generator_sources:
  local: "generators"
"#,
    )
    .expect("failed to write nfw.yaml");

    let root_tpl_dir = sandbox.join("generators").join("dotnet-service");
    fs::create_dir_all(&root_tpl_dir).unwrap();
    fs::write(
        root_tpl_dir.join("nfw.generator.yaml"),
        "id: dotnet-service\nname: Dotnet Service\nversion: 1.0.0\ngenerators:\n  persistence: persistence\n  webapi: webapi\n",
    )
    .unwrap();

    let webapi_tpl_dir = root_tpl_dir.join("webapi");
    fs::create_dir_all(&webapi_tpl_dir).unwrap();
    fs::write(
        webapi_tpl_dir.join("nfw.generator.yaml"),
        "id: dotnet-service/webapi\nsteps:\n  - action: render\n    source: \"Startup.cs.tera\"\n    destination: \"src/presentation/{{ Service }}.Presentation.WebApi/Startup.cs\"\n",
    )
    .unwrap();
    fs::write(webapi_tpl_dir.join("Startup.cs.tera"), "// startup").unwrap();

    let tpl_dir = root_tpl_dir.join("persistence");
    fs::create_dir_all(&tpl_dir).unwrap();
    fs::write(
        tpl_dir.join("nfw.generator.yaml"),
        r#"
id: dotnet-service/persistence
steps:
  - action: render
    source: DbContext.cs.tera
    destination: "{{ Name }}DbContext.cs"
"#,
    )
    .unwrap();
    fs::write(
        tpl_dir.join("DbContext.cs.tera"),
        "// DbContext for {{ Name }}",
    )
    .expect("failed to write tera");
    fs::create_dir_all(
        sandbox.join("src/TestService/src/presentation/TestService.Presentation.WebApi"),
    )
    .unwrap();

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let mut opts = HashMap::new();
    opts.insert("service".to_string(), "TestService".to_string());
    opts.insert("no-input".to_string(), "true".to_string());

    let _result = AddPersistenceCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).unwrap();

    // The command should succeed (or at least not crash) even when module already exists.
    // The ArtifactGenerationService.add_service_module handles duplicate detection internally.
    // Whether it reports success or an info message depends on the existing implementation.
    // Either way it should not panic or produce a corrupt state.
    let nfw_yaml_content = fs::read_to_string(sandbox.join("nfw.yaml")).unwrap();
    let yaml: serde_yaml::Value = serde_yaml::from_str(&nfw_yaml_content).unwrap();

    let modules = yaml
        .get("services")
        .and_then(|s| s.get("TestService"))
        .and_then(|details| details.get("modules"))
        .and_then(|m| m.as_sequence())
        .expect("modules should exist");

    let persistence_count = modules
        .iter()
        .filter(|m| m.as_str() == Some("persistence"))
        .count();

    assert!(
        persistence_count <= 1,
        "persistence module should not be duplicated, found {} entries",
        persistence_count
    );

    support::cleanup_sandbox_directory(&sandbox);
}
