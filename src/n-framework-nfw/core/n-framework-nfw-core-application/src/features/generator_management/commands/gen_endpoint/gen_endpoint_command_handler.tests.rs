#[cfg(test)]
mod tests {
    use super::super::gen_endpoint_command_handler::GenEndpointCommandHandler;
    use crate::features::generator_management::commands::gen_endpoint::gen_endpoint_command::{
        GenEndpointCommand, HttpMethod,
    };
    use crate::features::generator_management::models::errors::add_artifact_error::AddArtifactError;
    use crate::features::generator_management::models::generator_error::GeneratorError;
    use crate::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
    use crate::features::generator_management::services::artifact_generation_service::{
        AddArtifactContext, WorkspaceContext,
    };
    use crate::features::generator_management::services::generator_engine::GeneratorEngine;
    use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
    use n_framework_nfw_core_domain::features::generator_management::{
        generator_config::GeneratorConfig, generator_parameters::GeneratorParameters,
    };
    use n_framework_nfw_infrastructure_workspace_metadata::PreservedComments;
    use std::path::{Path, PathBuf};

    // Need mocks since we're in core-application and infrastructure is not available
    struct MockWorkingDir {}
    impl WorkingDirectoryProvider for MockWorkingDir {
        fn current_dir(&self) -> Result<PathBuf, String> {
            Ok(PathBuf::from(""))
        }
    }

    struct MockGeneratorEngine {}
    impl GeneratorEngine for MockGeneratorEngine {
        fn execute(
            &self,
            _config: &GeneratorConfig,
            _generator_root: &Path,
            _output_root: &Path,
            _parameters: &GeneratorParameters,
        ) -> Result<(), GeneratorError> {
            Ok(())
        }
    }

    fn test_valid_yaml() -> serde_yaml::Value {
        serde_yaml::from_str(
            r#"
workspace:
  namespace: SmokeTest
workspaces:
  local:
    namespace: SmokeTest
    type: custom
services:
  TestService:
    description: A test service
    path: src/TestService
    generator:
      id: "test"
    modules: []
"#,
        )
        .unwrap()
    }

    /// Builds a GeneratorConfig for the endpoint generator that declares `mediator_sources` so the
    /// handler performs the mediator-artifact existence check and a single render step so the
    /// duplicate-endpoint check has a destination to probe.
    fn test_endpoint_config() -> GeneratorConfig {
        serde_yaml::from_str(
            r#"
id: test-endpoint
mediator_sources:
  - command
  - query
steps:
  - action: render
    source: Endpoint.cs.tera
    destination: 'src/presentation/{{ Service }}.Presentation.WebApi/Endpoints/{{ Feature }}/{{ Name }}Endpoint.cs'
"#,
        )
        .unwrap()
    }

    /// A MockGeneratorRoot that resolves to a given base directory.
    struct FixedMockGeneratorRoot {
        base: PathBuf,
    }

    impl GeneratorRootResolver for FixedMockGeneratorRoot {
        fn resolve(
            &self,
            _nfw_yaml: &serde_yaml::Value,
            _generator_id: &str,
            _workspace_root: &std::path::Path,
        ) -> Result<PathBuf, String> {
            Ok(self.base.clone())
        }
    }

    /// Writes minimal command and query generator configs under `base/command/` and `base/query/`
    /// so `resolve_mediator_artifact_root` can parse their step destinations.
    fn write_mediator_generator_configs(base: &Path) {
        let cmd_dir = base.join("command");
        std::fs::create_dir_all(&cmd_dir).unwrap();
        std::fs::write(
            cmd_dir.join("nfw.workflow.yaml"),
            "id: test/command\nname: Command\nsteps:\n  - action: render\n    source: Command.cs.tera\n    destination: 'src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Commands/{{ Name }}/{{ Name }}Command.cs'\n",
        )
        .unwrap();

        let qry_dir = base.join("query");
        std::fs::create_dir_all(&qry_dir).unwrap();
        std::fs::write(
            qry_dir.join("nfw.workflow.yaml"),
            "id: test/query\nname: Query\nsteps:\n  - action: render\n    source: Query.cs.tera\n    destination: 'src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Queries/{{ Name }}/{{ Name }}Query.cs'\n",
        )
        .unwrap();

        // Root generator.yaml must declare the generators map
        std::fs::write(
            base.join("nfw.generator.yaml"),
            "id: test\nname: Test\nversion: 1.0.0\ngenerators:\n  command: command\n  query: query\n",
        )
        .unwrap();
    }

    #[test]
    fn test_missing_feature_directory_returns_error() {
        let root = tempfile::tempdir().unwrap();
        let tpl_base = root.path().join("generators").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_generator_configs(&tpl_base);

        let config = test_endpoint_config();
        let nfw_yaml = test_valid_yaml();

        let context = AddArtifactContext {
            generator_root: root.path().to_path_buf(),
            service_path: std::path::PathBuf::from("src/TestService"),
            service_name: "TestService".to_string(),
            config,
            workspace: WorkspaceContext::new(
                root.path().to_path_buf(),
                nfw_yaml.clone(),
                PreservedComments::default(),
            )
            .unwrap(),
        };

        let command = GenEndpointCommand::new(
            "GetProduct".to_string(),
            Some("NonExistentFeature".to_string()),
            HttpMethod::Get,
            None,
            context,
            true,
        )
        .unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockGeneratorRoot { base: tpl_base },
            MockGeneratorEngine {},
        );

        let result = handler.handle(command);

        assert!(result.is_err());
        match result.unwrap_err() {
            AddArtifactError::ExecutionFailed(err) => {
                let msg = err.to_string();
                if !msg.contains("No Command or Query artifact found") {
                    panic!("Actual error: {}", msg);
                }
            }
            e => panic!("Expected ExecutionFailed, got: {:?}", e),
        }
    }

    #[test]
    fn test_missing_command_query_file_returns_error() {
        let root = tempfile::tempdir().unwrap();
        let tpl_base = root.path().join("generators").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_generator_configs(&tpl_base);

        // Feature dir exists but no Command/Query file inside it
        let feature_dir = root.path().join(
            "src/TestService/src/core/TestService.Core.Application/Features/Inventory/Commands",
        );
        std::fs::create_dir_all(&feature_dir).unwrap();

        let config = test_endpoint_config();
        let nfw_yaml = test_valid_yaml();

        let context = AddArtifactContext {
            generator_root: root.path().to_path_buf(),
            service_path: std::path::PathBuf::from("src/TestService"),
            service_name: "TestService".to_string(),
            config,
            workspace: WorkspaceContext::new(
                root.path().to_path_buf(),
                nfw_yaml.clone(),
                PreservedComments::default(),
            )
            .unwrap(),
        };

        let command = GenEndpointCommand::new(
            "GetProduct".to_string(),
            Some("Inventory".to_string()),
            HttpMethod::Get,
            None,
            context,
            true,
        )
        .unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockGeneratorRoot { base: tpl_base },
            MockGeneratorEngine {},
        );

        let result = handler.handle(command);

        assert!(result.is_err());
        match result.unwrap_err() {
            AddArtifactError::ExecutionFailed(err) => {
                let msg = err.to_string();
                if !msg.contains("No Command or Query artifact found") {
                    panic!("Actual error: {}", msg);
                }
            }
            e => panic!("Expected ExecutionFailed, got: {:?}", e),
        }
    }

    #[test]
    fn test_existing_endpoint_file_returns_error() {
        let root = tempfile::tempdir().unwrap();
        let tpl_base = root.path().join("generators").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_generator_configs(&tpl_base);

        // Create the GetProductQuery.cs in the nested generator path:
        // features_root / feature / Commands / GetProduct / GetProductQuery.cs
        let query_dir = root
            .path()
            .join("src/TestService/src/core/TestService.Core.Application/Features/Inventory/Queries/GetProduct");
        std::fs::create_dir_all(&query_dir).unwrap();
        std::fs::write(query_dir.join("GetProductQuery.cs"), "dummy").unwrap();

        // Create the endpoint file that should trigger the duplicate check
        let endpoint_dir = root.path().join(
            "src/TestService/src/presentation/TestService.Presentation.WebApi/Endpoints/Inventory",
        );
        std::fs::create_dir_all(&endpoint_dir).unwrap();
        std::fs::write(endpoint_dir.join("GetProductEndpoint.cs"), "dummy").unwrap();

        let config = test_endpoint_config();
        let nfw_yaml = test_valid_yaml();

        let context = AddArtifactContext {
            generator_root: root.path().to_path_buf(),
            service_path: std::path::PathBuf::from("src/TestService"),
            service_name: "TestService".to_string(),
            config,
            workspace: WorkspaceContext::new(
                root.path().to_path_buf(),
                nfw_yaml.clone(),
                PreservedComments::default(),
            )
            .unwrap(),
        };

        let command = GenEndpointCommand::new(
            "GetProduct".to_string(),
            Some("Inventory".to_string()),
            HttpMethod::Get,
            None,
            context,
            true,
        )
        .unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockGeneratorRoot { base: tpl_base },
            MockGeneratorEngine {},
        );

        let result = handler.handle(command);

        assert!(result.is_err());
        match result.unwrap_err() {
            AddArtifactError::ExecutionFailed(err) => {
                let msg = err.to_string();
                if !msg.contains("Target endpoint file already exists") {
                    panic!("Actual error: {}", msg);
                }
            }
            e => panic!("Expected ExecutionFailed, got: {:?}", e),
        }
    }

    #[test]
    fn test_parameter_injection_collision() {
        let root = tempfile::tempdir().unwrap();
        let tpl_base = root.path().join("generators").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_generator_configs(&tpl_base);

        let query_dir = root
            .path()
            .join("src/TestService/src/core/TestService.Core.Application/Features/Inventory/Queries/GetProduct");
        std::fs::create_dir_all(&query_dir).unwrap();
        std::fs::write(query_dir.join("GetProductQuery.cs"), "dummy").unwrap();

        let config = test_endpoint_config();
        let nfw_yaml = test_valid_yaml();

        let context = AddArtifactContext::new(
            WorkspaceContext::new(
                root.path().to_path_buf(),
                nfw_yaml.clone(),
                PreservedComments::default(),
            )
            .unwrap(),
            root.path().to_path_buf(),
            config,
            "TestService".to_string(),
            std::path::PathBuf::from("src/TestService"),
        )
        .unwrap();

        // Pass a JSON parameter containing `OperationType` to verify the system correctly overrides it
        let custom_params = serde_json::json!({
            "OperationType": "CUSTOM",
            "AttachToMediator": false,
            "CustomKey": "CustomValue"
        });

        let command = GenEndpointCommand::new(
            "GetProduct".to_string(),
            Some("Inventory".to_string()),
            HttpMethod::Get,
            Some(custom_params),
            context,
            true,
        )
        .unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockGeneratorRoot { base: tpl_base },
            MockGeneratorEngine {},
        );

        let result = handler.handle(command);
        assert!(
            result.is_ok(),
            "Expected success with overridden parameter injection"
        );
    }

    #[test]
    fn test_attach_to_mediator_false_bypasses_check() {
        let root = tempfile::tempdir().unwrap();
        let tpl_base = root.path().join("generators").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_generator_configs(&tpl_base);

        // DO NOT create the query file. It should still succeed because attach_to_mediator = false

        let config = test_endpoint_config();
        let nfw_yaml = test_valid_yaml();

        let context = AddArtifactContext::new(
            WorkspaceContext::new(
                root.path().to_path_buf(),
                nfw_yaml.clone(),
                PreservedComments::default(),
            )
            .unwrap(),
            root.path().to_path_buf(),
            config,
            "TestService".to_string(),
            std::path::PathBuf::from("src/TestService"),
        )
        .unwrap();

        let command = GenEndpointCommand::new(
            "GetProduct".to_string(),
            Some("Inventory".to_string()),
            HttpMethod::Get,
            None,
            context,
            false, // Bypass check!
        )
        .unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockGeneratorRoot { base: tpl_base },
            MockGeneratorEngine {},
        );

        let result = handler.handle(command);
        assert!(
            result.is_ok(),
            "Expected success when attach_to_mediator is false, even if artifact is missing"
        );
    }

    #[test]
    fn test_get_mediator_items_success() {
        let root = tempfile::tempdir().unwrap();
        let tpl_base = root.path().join("generators").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_generator_configs(&tpl_base);

        // Create some mediator commands
        let feature_dir = root.path().join(
            "src/TestService/src/core/TestService.Core.Application/Features/Inventory/Commands",
        );

        let cmd1_dir = feature_dir.join("CreateProduct");
        std::fs::create_dir_all(&cmd1_dir).unwrap();
        std::fs::write(cmd1_dir.join("CreateProductCommand.cs"), "dummy").unwrap();

        let cmd2_dir = feature_dir.join("UpdateProduct");
        std::fs::create_dir_all(&cmd2_dir).unwrap();
        std::fs::write(cmd2_dir.join("UpdateProductCommand.cs"), "dummy").unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockGeneratorRoot { base: tpl_base },
            MockGeneratorEngine {},
        );

        let workspace_context = WorkspaceContext::new(
            root.path().to_path_buf(),
            test_valid_yaml(),
            PreservedComments::default(),
        )
        .unwrap();

        let service_info = handler
            .service
            .extract_services(&workspace_context)
            .unwrap()
            .into_iter()
            .find(|s| s.name() == "TestService")
            .unwrap();

        let items = handler
            .get_mediator_items(&workspace_context, &service_info, "Inventory", false)
            .unwrap();

        assert_eq!(items.len(), 2);
        assert!(items.contains(&"CreateProduct".to_string()));
        assert!(items.contains(&"UpdateProduct".to_string()));
    }

    #[test]
    fn test_has_mediator_sources_success() {
        let root = tempfile::tempdir().unwrap();
        let tpl_base = root.path().join("generators").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_generator_configs(&tpl_base);

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockGeneratorRoot { base: tpl_base },
            MockGeneratorEngine {},
        );

        let nfw_yaml = r#"
services:
  TestService:
    path: src/TestService
    generator:
      id: "test"
    modules: ["mediator"]
"#;
        let workspace_context = WorkspaceContext::new(
            root.path().to_path_buf(),
            serde_yaml::from_str(nfw_yaml).unwrap(),
            PreservedComments::default(),
        )
        .unwrap();

        let service_info = handler
            .service
            .extract_services(&workspace_context)
            .unwrap()
            .into_iter()
            .find(|s| s.name() == "TestService")
            .unwrap();

        let sources = vec!["command".to_string(), "query".to_string()];

        // Mocking generator context load: we need the command/query generators to NOT have required_modules
        // or have modules that the service has. Our write_mediator_generator_configs creates them without required_modules.

        assert!(handler.has_mediator_sources(&workspace_context, &service_info, &sources));
    }

    #[test]
    fn test_has_mediator_sources_fails_on_missing_modules() {
        let root = tempfile::tempdir().unwrap();
        let tpl_base = root.path().join("generators").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();

        // Write generators that REQUIRE "mediator" module
        let cmd_dir = tpl_base.join("command");
        std::fs::create_dir_all(&cmd_dir).unwrap();
        std::fs::write(
            cmd_dir.join("nfw.workflow.yaml"),
            "id: test/command\nrequired_modules: [\"mediator\"]\nsteps: []\n",
        )
        .unwrap();

        std::fs::write(
            tpl_base.join("nfw.generator.yaml"),
            "id: test\ngenerators:\n  command: command\n",
        )
        .unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockGeneratorRoot { base: tpl_base },
            MockGeneratorEngine {},
        );

        // Service with NO modules
        let nfw_yaml = r#"
services:
  TestService:
    path: src/TestService
    generator:
      id: "test"
    modules: []
"#;
        let workspace_context = WorkspaceContext::new(
            root.path().to_path_buf(),
            serde_yaml::from_str(nfw_yaml).unwrap(),
            PreservedComments::default(),
        )
        .unwrap();

        let service_info = handler
            .service
            .extract_services(&workspace_context)
            .unwrap()
            .into_iter()
            .find(|s| s.name() == "TestService")
            .unwrap();

        let sources = vec!["command".to_string()];

        assert!(!handler.has_mediator_sources(&workspace_context, &service_info, &sources));
    }
}
