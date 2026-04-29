use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::artifact_generation_service::{
    ArtifactGenerationService, ServiceInfo, WorkspaceContext,
};
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;

use super::add_persistence_command::AddPersistenceCommand;

#[derive(Debug, Clone)]
pub struct AddPersistenceCommandHandler<W, R, E> {
    service: ArtifactGenerationService<W, R, E>,
}

impl<W, R, E> AddPersistenceCommandHandler<W, R, E>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
{
    pub fn new(working_dir_provider: W, root_resolver: R, engine: E) -> Self {
        Self {
            service: ArtifactGenerationService::new(working_dir_provider, root_resolver, engine),
        }
    }

    /// Handles the `add persistence` command workflow.
    ///
    /// ## Workflow Context
    /// 1. Extracts variables required for rendering template content and names, including identifying target service properties.
    /// 2. Performs a robust template resolution algorithm to locate the appropriate templates on disk or fallback paths.
    /// 3. Validates naming rules matching NFramework identifiers against CLI payload properties.
    /// 4. Executes code generation using the templating engine.
    pub fn handle(&self, cmd: &AddPersistenceCommand) -> Result<(), AddArtifactError> {
        let workspace = cmd.workspace_context();
        let context = self.service.load_template_context(
            workspace.clone(),
            cmd.service_info(),
            AddPersistenceCommand::GENERATOR_TYPE,
        )?;

        let namespace = self.service.extract_namespace(workspace.nfw_yaml())?;

        let parameters =
            n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters::new()
                .with_name(cmd.service_info().name())
                .map_err(AddArtifactError::InvalidParameter)?
                .with_namespace(namespace)
                .map_err(AddArtifactError::InvalidParameter)?
                .with_service(cmd.service_info().name())
                .map_err(AddArtifactError::InvalidParameter)?;

        let output_root = workspace.workspace_root().join(cmd.service_info().path());

        self.service
            .engine()
            .execute(
                &context.config,
                &context.template_root,
                &output_root,
                &parameters,
            )
            .map_err(|e| AddArtifactError::ExecutionFailed(Box::new(e)))?;

        self.service.add_service_module(
            workspace.workspace_root(),
            cmd.service_info().name(),
            AddPersistenceCommand::GENERATOR_TYPE,
        )?;

        Ok(())
    }

    pub fn get_workspace_context(&self) -> Result<WorkspaceContext, AddArtifactError> {
        self.service.get_workspace_context()
    }

    pub fn extract_services(
        &self,
        workspace: &WorkspaceContext,
    ) -> Result<Vec<ServiceInfo>, AddArtifactError> {
        self.service.extract_services(workspace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
    use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
    use std::path::{Path, PathBuf};
    use tempfile;

    struct MockWorkingDir;
    impl WorkingDirectoryProvider for MockWorkingDir {
        fn current_dir(&self) -> Result<PathBuf, String> {
            Ok(PathBuf::from("/mock/workspace"))
        }
    }

    struct MockEngine {
        fail_execution: bool,
    }
    impl TemplateEngine for MockEngine {
        fn execute(
            &self,
            _config: &TemplateConfig,
            _root: &Path,
            _output: &Path,
            _params: &TemplateParameters,
        ) -> Result<(), crate::features::template_management::models::template_error::TemplateError>
        {
            if self.fail_execution {
                Err(
                    crate::features::template_management::models::template_error::TemplateError::io(
                        "mock error",
                        PathBuf::from("mock"),
                    ),
                )
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn handle_returns_error_when_engine_fails() {
        let sandbox = tempfile::tempdir().unwrap();
        let template_dir = sandbox.path().join("my-template");
        let sub_template_dir = template_dir.join("persistence");
        std::fs::create_dir_all(&sub_template_dir).unwrap();

        let template_yaml = r#"
id: my-template
generators:
  persistence: "persistence"
"#;
        std::fs::write(template_dir.join("template.yaml"), template_yaml).unwrap();
        std::fs::write(sub_template_dir.join("template.yaml"), template_yaml).unwrap();

        struct LocalMockResolver(PathBuf);
        impl TemplateRootResolver for LocalMockResolver {
            fn resolve(
                &self,
                _yaml: &serde_yaml::Value,
                _id: &str,
                _root: &Path,
            ) -> Result<PathBuf, String> {
                Ok(self.0.clone())
            }
        }

        let handler = AddPersistenceCommandHandler::new(
            MockWorkingDir,
            LocalMockResolver(template_dir),
            MockEngine {
                fail_execution: true,
            },
        );

        let nfw_yaml = serde_yaml::from_str("workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1").unwrap();
        let cmd = AddPersistenceCommand::new(
            ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
            WorkspaceContext::new(
                PathBuf::from("/mock/workspace"),
                nfw_yaml,
                n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
            )
            .unwrap(),
        )
        .unwrap();

        let result = handler.handle(&cmd);
        assert!(matches!(result, Err(AddArtifactError::ExecutionFailed(_))));
    }
}
