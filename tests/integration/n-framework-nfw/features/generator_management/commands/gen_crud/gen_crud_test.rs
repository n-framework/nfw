#[path = "../../../service_add/support.rs"]
mod support;

#[path = "support.rs"]
mod gen_support;

use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;

static DIR_LOCK: Mutex<()> = Mutex::new(());

fn setup_crud_workspace(sandbox: &Path, with_entity: bool, feature: &str) {
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
      - mediator
      - persistence
      - webapi
generator_sources:
  local: "generators"
"#,
    )
    .expect("failed to write nfw.yaml");

    let root_tpl_dir = sandbox.join("generators").join("dotnet-service");
    fs::create_dir_all(&root_tpl_dir).expect("failed to create root generator dir");
    fs::write(
        root_tpl_dir.join("nfw.generator.yaml"),
        "id: dotnet-service\nname: Dotnet Service\nversion: 1.0.0\ngenerators:\n  crud: ./crud/\n  entity: ./entity/\n",
    )
    .expect("failed to write root generator.yaml");

    // Entity sub-generator
    let entity_tpl_dir = root_tpl_dir.join("entity");
    fs::create_dir_all(entity_tpl_dir.join("content"))
        .expect("failed to create entity generator dir");
    fs::write(
        entity_tpl_dir.join("nfw.workflow.yaml"),
        r#"steps:
  - action: render
    source: "content/Entity.cs.tera"
    destination: "src/core/{{ Service }}.Core.Domain/Features/{{ Feature }}/Entities/{{ Name }}.cs"
"#,
    )
    .expect("failed to write entity workflow");
    fs::write(
        entity_tpl_dir.join("content/Entity.cs.tera"),
        "public class {{ Name }} {}",
    )
    .expect("failed to write entity template");

    // CRUD sub-generator (minimal — just commands + queries + repository interface)
    let crud_tpl_dir = root_tpl_dir.join("crud");
    fs::create_dir_all(crud_tpl_dir.join("content/commands"))
        .expect("failed to create crud commands dir");
    fs::create_dir_all(crud_tpl_dir.join("content/queries"))
        .expect("failed to create crud queries dir");
    fs::create_dir_all(crud_tpl_dir.join("content/persistence"))
        .expect("failed to create crud persistence dir");

    fs::write(
        crud_tpl_dir.join("nfw.workflow.yaml"),
        r#"required_modules:
  - mediator
  - persistence
  - webapi
steps:
  - action: render
    source: "content/commands/CreateCommand.cs.tera"
    destination: "src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Commands/Create{{ Name }}/Create{{ Name }}Command.cs"
  - action: render
    source: "content/queries/GetByIdQuery.cs.tera"
    destination: "src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Queries/Get{{ Name }}ById/Get{{ Name }}ByIdQuery.cs"
  - action: render
    source: "content/persistence/Repository.cs.tera"
    destination: "src/core/{{ Service }}.Core.Application/Contracts/Persistence/I{{ Name }}Repository.cs"
"#,
    )
    .expect("failed to write crud workflow");

    fs::write(
        crud_tpl_dir.join("content/commands/CreateCommand.cs.tera"),
        "public record Create{{ Name }}Command();",
    )
    .unwrap();
    fs::write(
        crud_tpl_dir.join("content/queries/GetByIdQuery.cs.tera"),
        "public record Get{{ Name }}ByIdQuery(Guid Id);",
    )
    .unwrap();
    fs::write(
        crud_tpl_dir.join("content/persistence/Repository.cs.tera"),
        "public interface I{{ Name }}Repository {}",
    )
    .unwrap();

    // Create entity if needed
    if with_entity {
        let entities_dir = sandbox
            .join("src/TestService/src/core/TestApp.Core.Domain/Features")
            .join(feature)
            .join("Entities");
        fs::create_dir_all(&entities_dir).expect("failed to create entities dir");
        fs::write(entities_dir.join("Product.cs"), "public class Product {}")
            .expect("failed to write entity file");

        // Entity schema
        let schema_dir = sandbox.join("src/TestService/.nfw/entities");
        fs::create_dir_all(&schema_dir).expect("failed to create schema dir");
        fs::write(
            schema_dir.join("Product.yaml"),
            format!(
                r#"name: Product
feature: {feature}
id_type: uuid
properties:
  - name: Name
    type: string
    nullable: false
"#
            ),
        )
        .expect("failed to write entity schema");
    } else {
        let features_root = sandbox.join("src/TestService/src/core/TestApp.Core.Domain/Features");
        fs::create_dir_all(&features_root).expect("failed to create features root");
    }
}

fn run_test_in_sandbox<F>(sandbox_name: &str, with_entity: bool, feature: &str, test_fn: F)
where
    F: FnOnce(&Path, Instant) -> Result<(), String>,
{
    let sandbox = support::create_sandbox_directory(sandbox_name);
    setup_crud_workspace(&sandbox, with_entity, feature);

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let start = Instant::now();
    let test_result = test_fn(&sandbox, start);

    std::env::set_current_dir(&original_dir).unwrap();

    if let Err(e) = test_result {
        panic!("Test failed in sandbox {}: {}", sandbox_name, e);
    }

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn generates_crud_successfully_with_valid_entity() {
    run_test_in_sandbox("gen-crud-success", true, "Products", |sandbox, start| {
        let result =
            gen_support::execute_non_interactive_gen_crud(sandbox, "Product", "Products", None);
        let duration = start.elapsed();

        if let Err(e) = result {
            return Err(format!("gen crud failed: {:?}", e));
        }

        if duration.as_secs_f64() >= 2.0 {
            return Err(format!(
                "Command took too long: {:.2}s (must be < 2s)",
                duration.as_secs_f64()
            ));
        }

        let service_dir = sandbox.join("src/TestService");

        let create_cmd = service_dir.join(
                "src/core/TestService.Core.Application/Features/Products/Commands/CreateProduct/CreateProductCommand.cs",
            );
        if !create_cmd.exists() {
            return Err("CreateProductCommand.cs was not generated".to_string());
        }

        let query = service_dir.join(
                "src/core/TestService.Core.Application/Features/Products/Queries/GetProductById/GetProductByIdQuery.cs",
            );
        if !query.exists() {
            return Err("GetProductByIdQuery.cs was not generated".to_string());
        }

        let repo = service_dir.join(
            "src/core/TestService.Core.Application/Contracts/Persistence/IProductRepository.cs",
        );
        if !repo.exists() {
            return Err("IProductRepository.cs was not generated".to_string());
        }

        let repo_content = fs::read_to_string(repo).unwrap();
        if !repo_content.contains("IProductRepository") {
            return Err("Repository content mismatch".to_string());
        }

        Ok(())
    });
}

// Entity missing validation is enforced at CLI handler level (GenCrudCliCommand),
// not at application handler level. Application handler generates templates
// regardless of entity existence. CLI-level test would require a mock prompt.

#[test]
fn gen_crud_fails_when_required_modules_missing() {
    let sandbox = support::create_sandbox_directory("gen-crud-no-modules");

    // Workspace without required modules
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
    .unwrap();

    let root_tpl_dir = sandbox.join("generators").join("dotnet-service");
    fs::create_dir_all(&root_tpl_dir).unwrap();
    fs::write(
        root_tpl_dir.join("nfw.generator.yaml"),
        "id: dotnet-service\nname: Dotnet Service\nversion: 1.0.0\ngenerators:\n  crud: ./crud/\n",
    )
    .unwrap();

    let crud_tpl_dir = root_tpl_dir.join("crud");
    fs::create_dir_all(crud_tpl_dir.join("content")).unwrap();
    fs::write(
        crud_tpl_dir.join("nfw.workflow.yaml"),
        "required_modules:\n  - mediator\n  - persistence\nsteps: []\n",
    )
    .unwrap();

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let result =
        gen_support::execute_non_interactive_gen_crud(&sandbox, "Product", "Products", None);

    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_err(), "Expected error for missing modules");
    let err_str = format!("{:?}", result.err().unwrap());
    assert!(
        err_str.contains("module")
            || err_str.contains("mediator")
            || err_str.contains("persistence"),
        "Error should mention missing modules: {}",
        err_str
    );

    support::cleanup_sandbox_directory(&sandbox);
}
