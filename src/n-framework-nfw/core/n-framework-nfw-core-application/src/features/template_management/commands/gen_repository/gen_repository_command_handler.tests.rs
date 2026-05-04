#[cfg(test)]
mod tests {
    use crate::features::template_management::commands::gen_repository::{
        gen_repository_command::GenRepositoryCommand,
        gen_repository_command_handler::GenRepositoryCommandHandler,
    };
    use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
    use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
    use crate::features::template_management::services::artifact_generation_service::AddArtifactContext;
    use crate::features::template_management::services::template_engine::TemplateEngine;
    use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
    use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
    use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
    use n_framework_nfw_infrastructure_workspace_metadata::PreservedComments;
    use serde_yaml::Value as YamlValue;
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    #[derive(Clone)]
    struct MockWorkingDirectoryProvider {
        pub dir: PathBuf,
    }

    impl WorkingDirectoryProvider for MockWorkingDirectoryProvider {
        fn current_dir(&self) -> Result<PathBuf, String> {
            Ok(self.dir.clone())
        }
    }

    #[derive(Clone)]
    struct MockTemplateRootResolver;

    impl TemplateRootResolver for MockTemplateRootResolver {
        fn resolve(
            &self,
            _nfw_yaml: &YamlValue,
            _template_id: &str,
            _workspace_root: &Path,
        ) -> Result<PathBuf, String> {
            Ok(PathBuf::from("/mock/templates"))
        }
    }

    #[derive(Clone)]
    struct MockTemplateEngine;

    impl TemplateEngine for MockTemplateEngine {
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

    fn create_test_context(workspace_root: PathBuf) -> AddArtifactContext {
        let yaml_content = r#"
workspace:
  namespace: TestApp
services:
  TestService:
    modules:
      - persistence
"#;
        let nfw_yaml: YamlValue = serde_yaml::from_str(yaml_content).unwrap();

        AddArtifactContext {
            workspace_root,
            nfw_yaml,
            preserved_comments: PreservedComments::default(),
            template_root: PathBuf::from("/mock/templates"),
            config: TemplateConfig::new(Some("test".to_string()), vec![], vec![]).unwrap(),
            service_name: "TestService".to_string(),
            service_path: PathBuf::from("src/TestService"),
        }
    }

    #[test]
    fn test_entity_in_multiple_features() {
        let temp_dir = TempDir::new().unwrap();
        let service_dir = temp_dir.path().join("src").join("TestService");
        let features_dir = service_dir.join("src").join("Features");

        // Create Feature1 with Entity
        let f1_entities = features_dir.join("Feature1").join("Entities");
        fs::create_dir_all(&f1_entities).unwrap();
        fs::write(f1_entities.join("User.cs"), "public class User {}").unwrap();

        // Create Feature2 with Entity
        let f2_entities = features_dir.join("Feature2").join("Entities");
        fs::create_dir_all(&f2_entities).unwrap();
        fs::write(f2_entities.join("User.cs"), "public class User {}").unwrap();

        let provider = MockWorkingDirectoryProvider {
            dir: temp_dir.path().to_path_buf(),
        };
        let handler = GenRepositoryCommandHandler::new(
            provider,
            MockTemplateRootResolver,
            MockTemplateEngine,
        );

        let command = GenRepositoryCommand::new(
            "User".to_string(),
            None,
            create_test_context(temp_dir.path().to_path_buf()),
        );

        let result = handler.handle(&command);
        assert!(result.is_err());
        if let Err(AddArtifactError::InvalidIdentifier(msg)) = result {
            assert!(msg.contains("found in multiple features"));
        } else {
            panic!("Expected InvalidIdentifier error");
        }
    }

    #[test]
    fn test_invalid_entity_names() {
        let temp_dir = TempDir::new().unwrap();
        let provider = MockWorkingDirectoryProvider {
            dir: temp_dir.path().to_path_buf(),
        };
        let handler = GenRepositoryCommandHandler::new(
            provider,
            MockTemplateRootResolver,
            MockTemplateEngine,
        );

        let invalid_names = vec!["1User", "Invalid Entity", "User@Name"];

        for name in invalid_names {
            let command = GenRepositoryCommand::new(
                name.to_string(),
                None,
                create_test_context(temp_dir.path().to_path_buf()),
            );

            let result = handler.handle(&command);
            assert!(result.is_err());
            if let Err(AddArtifactError::InvalidIdentifier(msg)) = result {
                assert!(msg.contains("Invalid entity name"));
            } else {
                panic!("Expected InvalidIdentifier error for name: {}", name);
            }
        }
    }

    #[test]
    fn test_repository_already_exists() {
        let temp_dir = TempDir::new().unwrap();
        let service_dir = temp_dir.path().join("src").join("TestService");
        let features_dir = service_dir.join("src").join("Features");

        let target_feature = "UserManagement";
        let f_entities = features_dir.join(target_feature).join("Entities");
        fs::create_dir_all(&f_entities).unwrap();
        fs::write(f_entities.join("User.cs"), "public class User {}").unwrap();

        let repo_dir = features_dir
            .join(target_feature)
            .join("Persistence")
            .join("Repositories");
        fs::create_dir_all(&repo_dir).unwrap();
        fs::write(
            repo_dir.join("UserRepository.cs"),
            "public class UserRepository {}",
        )
        .unwrap();

        let provider = MockWorkingDirectoryProvider {
            dir: temp_dir.path().to_path_buf(),
        };
        let handler = GenRepositoryCommandHandler::new(
            provider,
            MockTemplateRootResolver,
            MockTemplateEngine,
        );

        let command = GenRepositoryCommand::new(
            "User".to_string(),
            Some(target_feature.to_string()),
            create_test_context(temp_dir.path().to_path_buf()),
        );

        let result = handler.handle(&command);
        assert!(result.is_err());
        match result {
            Err(AddArtifactError::ArtifactAlreadyExists(msg)) => {
                assert!(msg.contains("already exists"));
            }
            Err(e) => panic!("Expected ArtifactAlreadyExists, got {:?}", e),
            Ok(_) => panic!("Expected error, but succeeded"),
        }
    }
}
