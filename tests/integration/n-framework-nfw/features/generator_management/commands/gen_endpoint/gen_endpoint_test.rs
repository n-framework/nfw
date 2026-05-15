#[path = "../../../service_add/support.rs"]
mod support;

#[path = "support.rs"]
mod gen_support;

use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;

static DIR_LOCK: Mutex<()> = Mutex::new(());

fn setup_endpoint_workspace(
    sandbox: &Path,
    with_webapi: bool,
    with_feature: bool,
    with_command: bool,
    with_existing_endpoint: bool,
    feature: &str,
) {
    let modules_str = if with_webapi {
        r#"
    modules:
      - webapi"#
    } else {
        ""
    };

    fs::write(
        sandbox.join("nfw.yaml"),
        format!(
            r#"
workspace:
  name: Test
  namespace: TestApp
services:
  TestService:
    path: src/TestService
    generator:
      id: dotnet-service{}
generator_sources:
  local: "generators"
"#,
            modules_str
        ),
    )
    .expect("failed to write nfw.yaml");

    // Scaffold the generator configuration
    let root_tpl_dir = sandbox.join("generators").join("dotnet-service");
    fs::create_dir_all(&root_tpl_dir).expect("failed to create root generator dir");
    fs::write(
         root_tpl_dir.join("nfw.generator.yaml"),
         "id: dotnet-service\nname: Dotnet Service\nversion: 1.0.0\ngenerators:\n  endpoint: ./endpoint/\n  command: ./command/\n  query: ./query/\nsteps:\n  - action: run_command\n    command: 'echo root'\n",
     )
     .expect("failed to write root generator.yaml");

    let tpl_dir = root_tpl_dir.join("endpoint");
    fs::create_dir_all(&tpl_dir).expect("failed to create sub-generator dir");

    fs::write(
         tpl_dir.join("nfw.generator.yaml"),
         r#"
id: dotnet-service/endpoint
required_modules: ["webapi"]
mediator_sources: ["command", "query"]
steps:
  - action: render
    source: "Endpoint.cs.tera"
    destination: "src/presentation/{{ Service }}.Presentation.WebApi/Endpoints/{{ Feature }}/{{ Name }}Endpoint.cs"
"#,
     ).expect("failed to write sub-generator generator.yaml");

    fs::write(
        tpl_dir.join("Endpoint.cs.tera"),
        "public class {{ Name }}Endpoint",
    )
    .unwrap();

    // Add command and query generators for mediator validation
    let cmd_tpl_dir = root_tpl_dir.join("command");
    fs::create_dir_all(&cmd_tpl_dir).expect("failed to create command generator dir");
    fs::write(
        cmd_tpl_dir.join("nfw.generator.yaml"),
        r#"
id: dotnet-service/command
required_modules: ["mediator"]
steps:
  - action: render
    source: "Command.cs.tera"
    destination: 'src/core/{{ Namespace }}.Core.Application/Features/{{ Feature }}/Commands/{{ Name }}/{{ Name }}Command.cs'
"#,
    ).expect("failed to write command generator.yaml");

    let query_tpl_dir = root_tpl_dir.join("query");
    fs::create_dir_all(&query_tpl_dir).expect("failed to create query generator dir");
    fs::write(
        query_tpl_dir.join("nfw.generator.yaml"),
        r#"
id: dotnet-service/query
required_modules: ["mediator"]
steps:
  - action: render
    source: "Query.cs.tera"
    destination: 'src/core/{{ Namespace }}.Core.Application/Features/{{ Feature }}/Queries/{{ Name }}/{{ Name }}Query.cs'
"#,
    ).expect("failed to write query generator.yaml");

    let service_dir = sandbox.join("src/TestService");

    if with_feature {
        let feature_dir = service_dir
            .join("src/core/TestApp.Core.Application/Features")
            .join(feature);
        fs::create_dir_all(&feature_dir).expect("failed to create feature dir");

        if with_command {
            let command_dir = feature_dir.join("Commands").join("Test");
            fs::create_dir_all(&command_dir).expect("failed to create command dir");
            fs::write(
                command_dir.join("TestCommand.cs"),
                "public class TestCommand {}",
            )
            .expect("failed to write command file");
        }
    }

    if with_existing_endpoint {
        let endpoint_dir = service_dir
            .join("src/presentation/TestService.Presentation.WebApi/Endpoints")
            .join(feature);
        fs::create_dir_all(&endpoint_dir).expect("failed to create endpoint dir");
        fs::write(
            endpoint_dir.join("TestEndpoint.cs"),
            "public class TestEndpoint {}",
        )
        .expect("failed to write existing endpoint");
    }
}

fn run_test_in_sandbox<F>(test_name: &str, params: (bool, bool, bool, bool, &str), action: F)
where
    F: FnOnce(&Path, Instant) -> Result<(), String> + std::panic::UnwindSafe,
{
    let _lock = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let sandbox_name = format!("gen-endpoint-{}", test_name);
    let sandbox = support::create_sandbox_directory(&sandbox_name);

    if sandbox.exists() {
        fs::remove_dir_all(&sandbox).unwrap();
    }
    fs::create_dir_all(&sandbox).unwrap();

    setup_endpoint_workspace(&sandbox, params.0, params.1, params.2, params.3, params.4);

    let start = Instant::now();
    let current_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let result = std::panic::catch_unwind(|| action(&sandbox, start));

    std::env::set_current_dir(current_dir).unwrap();

    if let Err(e) = result {
        std::panic::resume_unwind(e);
    } else if let Ok(Err(e)) = result {
        panic!("{}", e);
    }
}

#[test]
fn gen_endpoint_success() {
    run_test_in_sandbox(
        "success",
        (true, true, true, false, "Catalog"),
        |sandbox, _| {
            let result = gen_support::execute_non_interactive_gen_endpoint(
                &sandbox, "Test", "Catalog", "POST",
            );

            if let Err(e) = result {
                return Err(format!("Expected success, but got error: {:?}", e));
            }

            let endpoint_file = sandbox.join("src/TestService/src/presentation/TestService.Presentation.WebApi/Endpoints/Catalog/TestEndpoint.cs");
            if !endpoint_file.exists() {
                return Err("Endpoint file was not created".to_string());
            }

            let content = fs::read_to_string(endpoint_file).unwrap();
            if !content.contains("public class TestEndpoint") {
                return Err("Endpoint file content is incorrect".to_string());
            }

            Ok(())
        },
    );
}

#[test]
fn gen_endpoint_fails_if_feature_not_found() {
    run_test_in_sandbox(
        "no-feature",
        (true, false, false, false, "Catalog"),
        |sandbox, _| {
            let result = gen_support::execute_non_interactive_gen_endpoint(
                &sandbox, "Test", "Catalog", "POST",
            );

            if result.is_ok() {
                return Err("Expected error, but got success".to_string());
            }

            let err_str = format!("{:?}", result.err().unwrap());
            if !err_str.contains("No Command or Query artifact found") {
                return Err(format!("Unexpected error message: {}", err_str));
            }

            Ok(())
        },
    );
}

#[test]
fn gen_endpoint_fails_if_command_not_found() {
    run_test_in_sandbox(
        "no-command",
        (true, true, false, false, "Catalog"),
        |sandbox, _| {
            let result = gen_support::execute_non_interactive_gen_endpoint(
                &sandbox, "Test", "Catalog", "POST",
            );

            if result.is_ok() {
                return Err("Expected error, but got success".to_string());
            }

            let err_str = format!("{:?}", result.err().unwrap());
            if !err_str.contains("No Command or Query artifact found") {
                return Err(format!("Unexpected error message: {}", err_str));
            }

            Ok(())
        },
    );
}

#[test]
fn gen_endpoint_fails_if_endpoint_already_exists() {
    run_test_in_sandbox(
        "existing-endpoint",
        (true, true, true, true, "Catalog"),
        |sandbox, _| {
            let result = gen_support::execute_non_interactive_gen_endpoint(
                &sandbox, "Test", "Catalog", "POST",
            );

            if result.is_ok() {
                return Err("Expected error, but got success".to_string());
            }

            let err_str = format!("{:?}", result.err().unwrap());
            if !err_str.contains("Target endpoint file already exists") {
                return Err(format!("Unexpected error message: {}", err_str));
            }

            Ok(())
        },
    );
}

#[test]
fn gen_endpoint_fails_if_webapi_missing() {
    run_test_in_sandbox(
        "no-webapi",
        (false, true, true, false, "Catalog"),
        |sandbox, _| {
            let result = gen_support::execute_non_interactive_gen_endpoint(
                &sandbox, "Test", "Catalog", "POST",
            );

            if result.is_ok() {
                return Err("Expected error, but got success".to_string());
            }

            let err_str = format!("{:?}", result.err().unwrap());
            if !err_str.contains("Required module 'webapi' is missing")
                && !err_str.contains("module 'webapi' is required but not installed")
            {
                return Err(format!("Unexpected error message: {}", err_str));
            }

            Ok(())
        },
    );
}

#[test]
fn gen_endpoint_fails_if_generator_resolution_fails() {
    run_test_in_sandbox(
        "invalid-generator",
        (true, true, false, false, "Catalog"),
        |sandbox, _| {
            // Corrupt the generator by removing the steps
            let tpl_dir = sandbox.join("generators/dotnet-service/endpoint");
            fs::write(
                tpl_dir.join("nfw.generator.yaml"),
                "id: dotnet-service/endpoint\nrequired_modules: [\"webapi\"]\nmediator_sources: [\"command\", \"query\"]\nsteps: [{ action: render, source: 't', destination: '{{ Feature }}/d' }]",
            )
            .unwrap();

            let result = gen_support::execute_non_interactive_gen_endpoint(
                &sandbox, "Test", "Catalog", "POST",
            );

            if result.is_ok() {
                return Err("Expected error for empty generator steps, but got success".to_string());
            }

            let err_str = format!("{:?}", result.err().unwrap());
            // Since there are no steps, it won't find the features root.
            if !err_str.contains("No Command or Query artifact found") {
                return Err(format!("Unexpected error message: {}", err_str));
            }

            Ok(())
        },
    );
}

#[test]
fn test_get_mediator_items() {
    let _lock = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let sandbox = support::create_sandbox_directory("get-mediator-items");
    if sandbox.exists() {
        fs::remove_dir_all(&sandbox).unwrap();
    }
    fs::create_dir_all(&sandbox).unwrap();

    setup_endpoint_workspace(&sandbox, true, true, true, false, "Catalog");

    let current_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::standard_working_directory_provider::StandardWorkingDirectoryProvider;
    use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::file_system_generator_root_resolver::FileSystemGeneratorRootResolver;
    use n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine;
    use n_framework_nfw_core_application::features::generator_management::commands::gen_endpoint::gen_endpoint_command_handler::GenEndpointCommandHandler;

    let handler = GenEndpointCommandHandler::new(
        StandardWorkingDirectoryProvider::new(),
        FileSystemGeneratorRootResolver::new(),
        FileSystemGeneratorEngine::new(),
    );

    let workspace_context = handler.get_workspace_context().unwrap();
    let services = handler.extract_services(&workspace_context).unwrap();
    let service = services[0].clone();

    // Test get_mediator_items for commands
    let commands = handler
        .get_mediator_items(&workspace_context, &service, "Catalog", false)
        .unwrap();
    assert!(commands.contains(&"Test".to_string()));

    // Test get_mediator_items for queries (should be empty)
    let queries = handler
        .get_mediator_items(&workspace_context, &service, "Catalog", true)
        .unwrap();
    assert!(queries.is_empty());

    std::env::set_current_dir(current_dir).unwrap();
}

#[test]
fn test_has_mediator_sources() {
    let _lock = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let sandbox = support::create_sandbox_directory("has-mediator-sources");
    if sandbox.exists() {
        fs::remove_dir_all(&sandbox).unwrap();
    }
    fs::create_dir_all(&sandbox).unwrap();

    // Case 1: Service has webapi but no mediator module - has_mediator_sources should be false
    // because command/query generators require 'mediator' module.
    setup_endpoint_workspace(&sandbox, true, false, false, false, "Catalog");

    let current_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::standard_working_directory_provider::StandardWorkingDirectoryProvider;
    use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::file_system_generator_root_resolver::FileSystemGeneratorRootResolver;
    use n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine;
    use n_framework_nfw_core_application::features::generator_management::commands::gen_endpoint::gen_endpoint_command_handler::GenEndpointCommandHandler;

    let handler = GenEndpointCommandHandler::new(
        StandardWorkingDirectoryProvider::new(),
        FileSystemGeneratorRootResolver::new(),
        FileSystemGeneratorEngine::new(),
    );

    let workspace_context = handler.get_workspace_context().unwrap();
    let services = handler.extract_services(&workspace_context).unwrap();
    let service = services[0].clone();

    let mediator_sources = vec!["command".to_string(), "query".to_string()];

    let has_sources = handler.has_mediator_sources(&workspace_context, &service, &mediator_sources);
    assert!(
        !has_sources,
        "Should NOT have mediator sources when mediator module is missing"
    );

    // Case 2: Add mediator module to nfw.yaml
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
      - webapi
      - mediator
generator_sources:
  local: "generators"
"#,
    )
    .unwrap();

    // Reload context
    let workspace_context = handler.get_workspace_context().unwrap();
    let services = handler.extract_services(&workspace_context).unwrap();
    let service = services[0].clone();

    let has_sources = handler.has_mediator_sources(&workspace_context, &service, &mediator_sources);
    assert!(
        has_sources,
        "Should HAVE mediator sources when mediator module is present"
    );

    std::env::set_current_dir(current_dir).unwrap();
}
