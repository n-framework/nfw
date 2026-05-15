#[cfg(test)]
mod tests {
    use crate::features::generator_management::commands::gen_repository::{
        gen_repository_command::GenRepositoryCommand,
        gen_repository_command_handler::GenRepositoryCommandHandler,
    };
    use crate::features::generator_management::models::errors::add_artifact_error::AddArtifactError;
    use crate::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
    use crate::features::generator_management::services::artifact_generation_service::AddArtifactContext;
    use crate::features::generator_management::services::generator_engine::GeneratorEngine;
    use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
    use n_framework_nfw_core_domain::features::generator_management::generator_config::GeneratorConfig;
    use n_framework_nfw_core_domain::features::generator_management::generator_parameters::GeneratorParameters;
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

    /// Generator root resolver that points to a caller-supplied base directory.
    #[derive(Clone)]
    struct FixedGeneratorRootResolver {
        base: PathBuf,
    }

    impl GeneratorRootResolver for FixedGeneratorRootResolver {
        fn resolve(
            &self,
            _nfw_yaml: &serde_yaml::Value,
            _generator_id: &str,
            _workspace_root: &std::path::Path,
        ) -> Result<PathBuf, String> {
            Ok(self.base.clone())
        }
    }

    #[derive(Clone)]
    struct MockGeneratorEngine;

    impl GeneratorEngine for MockGeneratorEngine {
        fn execute(
            &self,
            _config: &GeneratorConfig,
            _generator_root: &Path,
            _output_root: &Path,
            _parameters: &GeneratorParameters,
        ) -> Result<
            (),
            crate::features::generator_management::models::generator_error::GeneratorError,
        > {
            Ok(())
        }
    }

    /// Writes the minimal generator configs under `base/` so that
    /// `find_artifact_in_features` and `resolve_artifact_subdir` can derive
    /// feature paths from the generator step destinations.
    fn write_generator_configs(base: &Path) {
        // Root generator: declares generators
        fs::write(
            base.join("nfw.generator.yaml"),
            "id: test\nname: Test\nversion: 1.0.0\ngenerators:\n  entity: entity\n  repository: repository\n",
        )
        .unwrap();

        // entity sub-generator
        let entity_dir = base.join("entity");
        fs::create_dir_all(&entity_dir).unwrap();
        fs::write(
            entity_dir.join("nfw.workflow.yaml"),
            "id: test/entity\nname: Entity\nsteps:\n  - action: render\n    source: Entity.cs.tera\n    destination: 'src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Entities/{{ Name }}.cs'\n",
        )
        .unwrap();

        // repository sub-generator
        let repo_dir = base.join("repository");
        fs::create_dir_all(&repo_dir).unwrap();
        fs::write(
            repo_dir.join("nfw.workflow.yaml"),
            "id: test/repository\nname: Repository\nsteps:\n  - action: render\n    source: Repository.cs.tera\n    destination: 'src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Persistence/Repositories/{{ Name }}Repository.cs'\n",
        )
        .unwrap();
    }

    fn create_test_context(workspace_root: PathBuf, generator_root: PathBuf) -> AddArtifactContext {
        let yaml_content = r#"
workspace:
  namespace: TestApp
services:
  TestService:
    path: src/TestService
    generator:
      id: test
    modules:
      - persistence
"#;
        let nfw_yaml: YamlValue = serde_yaml::from_str(yaml_content).unwrap();

        AddArtifactContext {
            workspace: crate::features::generator_management::services::artifact_generation_service::WorkspaceContext::new(workspace_root.clone(), nfw_yaml.clone(), PreservedComments::default()).unwrap(),
            generator_root,
            config: GeneratorConfig::new(
                Some("test".to_string()),
                vec![n_framework_nfw_core_domain::features::generator_management::generator_config::GeneratorStepConfig {
                    condition: None,
                    action: n_framework_nfw_core_domain::features::generator_management::generator_config::GeneratorStepAction::RunCommand {
                        command: "echo done".to_string(),
                        working_directory: None,
                    },
                }],
                vec![],
            )
            .unwrap(),
            service_name: "TestService".to_string(),
            service_path: PathBuf::from("src/TestService"),
        }
    }

    // Generator-derived paths for TestService with namespace TestApp:
    // entity:     src/TestService/src/core/TestApp.Core.Application/Features/{Feature}/Entities/{Name}.cs
    // repository: src/TestService/src/core/TestApp.Core.Application/Features/{Feature}/Persistence/Repositories/{Name}Repository.cs
    // NOTE: The `{{ Service }}` placeholder is substituted with the service *name* ("TestService"),
    // and `{{ Namespace }}` with the workspace namespace. Our generator uses `{{ Service }}` so the
    // path prefix becomes `TestService.Core.Application`.

    fn entity_features_root(workspace_root: &Path) -> PathBuf {
        workspace_root.join("src/TestService/src/core/TestService.Core.Application/Features")
    }

    fn repo_dir_for(workspace_root: &Path, feature: &str) -> PathBuf {
        entity_features_root(workspace_root)
            .join(feature)
            .join("Persistence")
            .join("Repositories")
    }

    fn entity_dir_for(workspace_root: &Path, feature: &str) -> PathBuf {
        entity_features_root(workspace_root)
            .join(feature)
            .join("Entities")
    }

    #[test]
    fn test_entity_in_multiple_features() {
        let temp_dir = TempDir::new().unwrap();
        let tpl_base = temp_dir.path().join("generators").join("test");
        fs::create_dir_all(&tpl_base).unwrap();
        write_generator_configs(&tpl_base);

        // Create Feature1 with User entity
        let f1_entities = entity_dir_for(temp_dir.path(), "Feature1");
        fs::create_dir_all(&f1_entities).unwrap();
        fs::write(f1_entities.join("User.cs"), "public class User {}").unwrap();

        // Create Feature2 with the same User entity
        let f2_entities = entity_dir_for(temp_dir.path(), "Feature2");
        fs::create_dir_all(&f2_entities).unwrap();
        fs::write(f2_entities.join("User.cs"), "public class User {}").unwrap();

        let provider = MockWorkingDirectoryProvider {
            dir: temp_dir.path().to_path_buf(),
        };
        let handler = GenRepositoryCommandHandler::new(
            provider,
            FixedGeneratorRootResolver {
                base: tpl_base.clone(),
            },
            MockGeneratorEngine,
        );

        let command = GenRepositoryCommand::new(
            "User".to_string(),
            None,
            create_test_context(temp_dir.path().to_path_buf(), tpl_base),
        );

        let result = handler.handle(&command);
        assert!(result.is_err());
        if let Err(AddArtifactError::InvalidIdentifier(msg)) = result {
            assert!(
                msg.contains("found in multiple features"),
                "Actual error: {}",
                msg
            );
        } else {
            panic!("Expected InvalidIdentifier error, got: {:?}", result);
        }
    }

    #[test]
    fn test_invalid_entity_names() {
        let temp_dir = TempDir::new().unwrap();
        let tpl_base = temp_dir.path().join("generators").join("test");
        fs::create_dir_all(&tpl_base).unwrap();
        write_generator_configs(&tpl_base);

        let provider = MockWorkingDirectoryProvider {
            dir: temp_dir.path().to_path_buf(),
        };
        let handler = GenRepositoryCommandHandler::new(
            provider,
            FixedGeneratorRootResolver {
                base: tpl_base.clone(),
            },
            MockGeneratorEngine,
        );

        let invalid_names = vec!["1User", "Invalid Entity", "User@Name"];

        for name in invalid_names {
            let command = GenRepositoryCommand::new(
                name.to_string(),
                None,
                create_test_context(temp_dir.path().to_path_buf(), tpl_base.clone()),
            );

            let result = handler.handle(&command);
            assert!(result.is_err());
            if let Err(AddArtifactError::InvalidIdentifier(msg)) = result {
                assert!(
                    msg.contains("Invalid entity name"),
                    "Actual error for '{}': {}",
                    name,
                    msg
                );
            } else {
                panic!("Expected InvalidIdentifier error for name: {}", name);
            }
        }
    }

    #[test]
    fn test_repository_already_exists() {
        let temp_dir = TempDir::new().unwrap();
        let tpl_base = temp_dir.path().join("generators").join("test");
        fs::create_dir_all(&tpl_base).unwrap();
        write_generator_configs(&tpl_base);

        let target_feature = "UserManagement";

        // Entity must exist so the handler proceeds past entity validation.
        let f_entities = entity_dir_for(temp_dir.path(), target_feature);
        fs::create_dir_all(&f_entities).unwrap();
        fs::write(f_entities.join("User.cs"), "public class User {}").unwrap();

        // Repository file that triggers the "already exists" guard.
        let repo_dir = repo_dir_for(temp_dir.path(), target_feature);
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
            FixedGeneratorRootResolver {
                base: tpl_base.clone(),
            },
            MockGeneratorEngine,
        );

        let command = GenRepositoryCommand::new(
            "User".to_string(),
            Some(target_feature.to_string()),
            create_test_context(temp_dir.path().to_path_buf(), tpl_base),
        );

        let result = handler.handle(&command);
        assert!(result.is_err());
        match result {
            Err(AddArtifactError::ArtifactAlreadyExists(msg)) => {
                assert!(msg.contains("already exists"), "Actual msg: {}", msg);
            }
            Err(e) => panic!("Expected ArtifactAlreadyExists, got {:?}", e),
            Ok(_) => panic!("Expected error, but succeeded"),
        }
    }
}
