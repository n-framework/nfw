#[cfg(test)]
mod tests {
    use super::super::gen_endpoint_command_handler::GenEndpointCommandHandler;
    use crate::features::template_management::commands::gen_endpoint::gen_endpoint_command::GenEndpointCommand;
    use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
    use crate::features::template_management::services::artifact_generation_service::AddArtifactContext;
    use n_framework_nfw_core_domain::features::template_management::{
        template_config::TemplateConfig, template_parameters::TemplateParameters,
    };
    use n_framework_nfw_infrastructure_workspace_metadata::PreservedComments;
    use std::path::{Path, PathBuf};

    // Need mocks since we're in core-application and infrastructure is not available
    struct MockWorkingDir {}
    impl crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider for MockWorkingDir {
        fn current_dir(&self) -> Result<PathBuf, String> {
            Ok(PathBuf::from(""))
        }
    }

    struct MockTemplateEngine {}
    impl crate::features::template_management::services::template_engine::TemplateEngine
        for MockTemplateEngine
    {
        fn execute(
            &self,
            _config: &TemplateConfig,
            _template_root: &Path,
            _output_root: &Path,
            _parameters: &TemplateParameters,
        ) -> Result<(), crate::features::template_management::models::template_error::TemplateError>
        {
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
    template:
      id: "test"
    modules: []
"#,
        )
        .unwrap()
    }

    /// Builds a TemplateConfig for the endpoint template that declares `mediator_sources` so the
    /// handler performs the mediator-artifact existence check and a single render step so the
    /// duplicate-endpoint check has a destination to probe.
    fn test_endpoint_config() -> TemplateConfig {
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

    /// A MockTemplateRoot that resolves to a given base directory.
    struct FixedMockTemplateRoot {
        base: PathBuf,
    }

    impl crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver
        for FixedMockTemplateRoot
    {
        fn resolve(&self, _nfw_yaml: &serde_yaml::Value, _template_id: &str, _workspace_root: &std::path::Path) -> Result<PathBuf, String> {
            Ok(self.base.clone())
        }
    }

    /// Writes minimal command and query template configs under `base/command/` and `base/query/`
    /// so `resolve_mediator_artifact_root` can parse their step destinations.
    fn write_mediator_template_configs(base: &Path) {
        let cmd_dir = base.join("command");
        std::fs::create_dir_all(&cmd_dir).unwrap();
        std::fs::write(
            cmd_dir.join("template.yaml"),
            "id: test/command\nname: Command\nsteps:\n  - action: render\n    source: Command.cs.tera\n    destination: 'src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Commands/{{ Name }}/{{ Name }}Command.cs'\n",
        )
        .unwrap();

        let qry_dir = base.join("query");
        std::fs::create_dir_all(&qry_dir).unwrap();
        std::fs::write(
            qry_dir.join("template.yaml"),
            "id: test/query\nname: Query\nsteps:\n  - action: render\n    source: Query.cs.tera\n    destination: 'src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Queries/{{ Name }}/{{ Name }}Query.cs'\n",
        )
        .unwrap();

        // Root template.yaml must declare the generators map
        std::fs::write(
            base.join("template.yaml"),
            "id: test\nname: Test\nversion: 1.0.0\ngenerators:\n  command: command\n  query: query\n",
        )
        .unwrap();
    }

    #[test]
    fn test_missing_feature_directory_returns_error() {
        let root = tempfile::tempdir().unwrap();
        let tpl_base = root.path().join("templates").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_template_configs(&tpl_base);

        let config = test_endpoint_config();
        let nfw_yaml = test_valid_yaml();

        let context = AddArtifactContext {
            template_root: root.path().to_path_buf(),
            service_path: std::path::PathBuf::from("src/TestService"),
            service_name: "TestService".to_string(),
            config,
            workspace: crate::features::template_management::services::artifact_generation_service::WorkspaceContext::new(root.path().to_path_buf(), nfw_yaml.clone(), PreservedComments::default()).unwrap(),
        };

        let command = GenEndpointCommand::new(
        "GetProduct".to_string(),
        Some("NonExistentFeature".to_string()),
        crate::features::template_management::commands::gen_endpoint::gen_endpoint_command::HttpMethod::Get,
        None,
        context,
        true,
    ).unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockTemplateRoot { base: tpl_base },
            MockTemplateEngine {},
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
        let tpl_base = root.path().join("templates").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_template_configs(&tpl_base);

        // Feature dir exists but no Command/Query file inside it
        let feature_dir = root.path().join(
            "src/TestService/src/core/TestService.Core.Application/Features/Inventory/Commands",
        );
        std::fs::create_dir_all(&feature_dir).unwrap();

        let config = test_endpoint_config();
        let nfw_yaml = test_valid_yaml();

        let context = AddArtifactContext {
            template_root: root.path().to_path_buf(),
            service_path: std::path::PathBuf::from("src/TestService"),
            service_name: "TestService".to_string(),
            config,
            workspace: crate::features::template_management::services::artifact_generation_service::WorkspaceContext::new(root.path().to_path_buf(), nfw_yaml.clone(), PreservedComments::default()).unwrap(),
        };

        let command = GenEndpointCommand::new(
        "GetProduct".to_string(),
        Some("Inventory".to_string()),
        crate::features::template_management::commands::gen_endpoint::gen_endpoint_command::HttpMethod::Get,
        None,
        context,
        true,
    ).unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockTemplateRoot { base: tpl_base },
            MockTemplateEngine {},
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
        let tpl_base = root.path().join("templates").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_template_configs(&tpl_base);

        // Create the GetProductQuery.cs in the nested template path:
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
            template_root: root.path().to_path_buf(),
            service_path: std::path::PathBuf::from("src/TestService"),
            service_name: "TestService".to_string(),
            config,
            workspace: crate::features::template_management::services::artifact_generation_service::WorkspaceContext::new(root.path().to_path_buf(), nfw_yaml.clone(), PreservedComments::default()).unwrap(),
        };

        let command = GenEndpointCommand::new(
        "GetProduct".to_string(),
        Some("Inventory".to_string()),
        crate::features::template_management::commands::gen_endpoint::gen_endpoint_command::HttpMethod::Get,
        None,
        context,
        true,
    ).unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockTemplateRoot { base: tpl_base },
            MockTemplateEngine {},
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
        let tpl_base = root.path().join("templates").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_template_configs(&tpl_base);

        let query_dir = root
            .path()
            .join("src/TestService/src/core/TestService.Core.Application/Features/Inventory/Queries/GetProduct");
        std::fs::create_dir_all(&query_dir).unwrap();
        std::fs::write(query_dir.join("GetProductQuery.cs"), "dummy").unwrap();

        let config = test_endpoint_config();
        let nfw_yaml = test_valid_yaml();

        let context = crate::features::template_management::services::artifact_generation_service::AddArtifactContext::new(
            crate::features::template_management::services::artifact_generation_service::WorkspaceContext::new(
                root.path().to_path_buf(),
                nfw_yaml.clone(),
                PreservedComments::default()
            ).unwrap(),
            root.path().to_path_buf(),
            config,
            "TestService".to_string(),
            std::path::PathBuf::from("src/TestService")
        ).unwrap();

        // Pass a JSON parameter containing `OperationType` to verify the system correctly overrides it
        let custom_params = serde_json::json!({
            "OperationType": "CUSTOM",
            "AttachToMediator": false,
            "CustomKey": "CustomValue"
        });

        let command = GenEndpointCommand::new(
            "GetProduct".to_string(),
            Some("Inventory".to_string()),
            crate::features::template_management::commands::gen_endpoint::gen_endpoint_command::HttpMethod::Get,
            Some(custom_params),
            context,
            true,
        ).unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockTemplateRoot { base: tpl_base },
            MockTemplateEngine {},
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
        let tpl_base = root.path().join("templates").join("test");
        std::fs::create_dir_all(&tpl_base).unwrap();
        write_mediator_template_configs(&tpl_base);

        // DO NOT create the query file. It should still succeed because attach_to_mediator = false

        let config = test_endpoint_config();
        let nfw_yaml = test_valid_yaml();

        let context = AddArtifactContext::new(
        crate::features::template_management::services::artifact_generation_service::WorkspaceContext::new(
            root.path().to_path_buf(),
            nfw_yaml.clone(),
            PreservedComments::default()
        ).unwrap(),
        root.path().to_path_buf(),
        config,
        "TestService".to_string(),
        std::path::PathBuf::from("src/TestService")
    ).unwrap();

        let command = GenEndpointCommand::new(
            "GetProduct".to_string(),
            Some("Inventory".to_string()),
            crate::features::template_management::commands::gen_endpoint::gen_endpoint_command::HttpMethod::Get,
            None,
            context,
            false, // Bypass check!
        ).unwrap();

        let handler = GenEndpointCommandHandler::new(
            MockWorkingDir {},
            FixedMockTemplateRoot { base: tpl_base },
            MockTemplateEngine {},
        );

        let result = handler.handle(command);
        assert!(
            result.is_ok(),
            "Expected success when attach_to_mediator is false, even if artifact is missing"
        );
    }
}
