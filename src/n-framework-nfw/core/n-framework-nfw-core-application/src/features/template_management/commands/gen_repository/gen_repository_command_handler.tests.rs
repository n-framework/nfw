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

    /// Template root resolver that points to a caller-supplied base directory.
    #[derive(Clone)]
    struct FixedTemplateRootResolver {
        base: PathBuf,
    }

    impl TemplateRootResolver for FixedTemplateRootResolver {
        fn resolve(
            &self,
            _nfw_yaml: &serde_yaml::Value,
            _template_id: &str,
            _workspace_root: &std::path::Path,
        ) -> Result<PathBuf, String> {
            Ok(self.base.clone())
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

    /// Writes the minimal template configs under `base/` so that
    /// `find_artifact_in_features` and `resolve_artifact_subdir` can derive
    /// feature paths from the template step destinations.
    fn write_template_configs(base: &Path) {
        // Root template: declares generators
        fs::write(
            base.join("template.yaml"),
            "id: test\nname: Test\nversion: 1.0.0\ngenerators:\n  entity: entity\n  repository: repository\n",
        )
        .unwrap();

        // entity sub-template
        let entity_dir = base.join("entity");
        fs::create_dir_all(&entity_dir).unwrap();
        fs::write(
            entity_dir.join("template.yaml"),
            "id: test/entity\nname: Entity\nsteps:\n  - action: render\n    source: Entity.cs.tera\n    destination: 'src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Entities/{{ Name }}.cs'\n",
        )
        .unwrap();

        // repository sub-template
        let repo_dir = base.join("repository");
        fs::create_dir_all(&repo_dir).unwrap();
        fs::write(
            repo_dir.join("template.yaml"),
            "id: test/repository\nname: Repository\nsteps:\n  - action: render\n    source: Repository.cs.tera\n    destination: 'src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Persistence/Repositories/{{ Name }}Repository.cs'\n",
        )
        .unwrap();
    }

    fn create_test_context(workspace_root: PathBuf, template_root: PathBuf) -> AddArtifactContext {
        let yaml_content = r#"
workspace:
  namespace: TestApp
services:
  TestService:
    path: src/TestService
    template:
      id: test
    modules:
      - persistence
"#;
        let nfw_yaml: YamlValue = serde_yaml::from_str(yaml_content).unwrap();

        AddArtifactContext {
            workspace: crate::features::template_management::services::artifact_generation_service::WorkspaceContext::new(workspace_root.clone(), nfw_yaml.clone(), PreservedComments::default()).unwrap(),
            template_root,
            config: TemplateConfig::new(
                Some("test".to_string()),
                vec![n_framework_nfw_core_domain::features::template_management::template_config::TemplateStepConfig {
                    condition: None,
                    action: n_framework_nfw_core_domain::features::template_management::template_config::TemplateStepAction::RunCommand {
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

    // Template-derived paths for TestService with namespace TestApp:
    // entity:     src/TestService/src/core/TestApp.Core.Application/Features/{Feature}/Entities/{Name}.cs
    // repository: src/TestService/src/core/TestApp.Core.Application/Features/{Feature}/Persistence/Repositories/{Name}Repository.cs
    // NOTE: The `{{ Service }}` placeholder is substituted with the service *name* ("TestService"),
    // and `{{ Namespace }}` with the workspace namespace. Our template uses `{{ Service }}` so the
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
        let tpl_base = temp_dir.path().join("templates").join("test");
        fs::create_dir_all(&tpl_base).unwrap();
        write_template_configs(&tpl_base);

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
            FixedTemplateRootResolver {
                base: tpl_base.clone(),
            },
            MockTemplateEngine,
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
        let tpl_base = temp_dir.path().join("templates").join("test");
        fs::create_dir_all(&tpl_base).unwrap();
        write_template_configs(&tpl_base);

        let provider = MockWorkingDirectoryProvider {
            dir: temp_dir.path().to_path_buf(),
        };
        let handler = GenRepositoryCommandHandler::new(
            provider,
            FixedTemplateRootResolver {
                base: tpl_base.clone(),
            },
            MockTemplateEngine,
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
        let tpl_base = temp_dir.path().join("templates").join("test");
        fs::create_dir_all(&tpl_base).unwrap();
        write_template_configs(&tpl_base);

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
            FixedTemplateRootResolver {
                base: tpl_base.clone(),
            },
            MockTemplateEngine,
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
