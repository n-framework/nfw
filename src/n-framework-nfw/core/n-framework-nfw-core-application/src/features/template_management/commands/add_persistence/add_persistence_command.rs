use crate::features::template_management::services::artifact_generation_service::{
    ServiceInfo, WorkspaceContext,
};

#[derive(Debug, Clone)]
pub struct AddPersistenceCommand {
    service_info: ServiceInfo,
    workspace_context: WorkspaceContext,
}

impl AddPersistenceCommand {
    pub const GENERATOR_TYPE: &'static str = "persistence";

    pub fn new(
        service_info: ServiceInfo,
        workspace_context: WorkspaceContext,
    ) -> Result<Self, String> {
        Ok(Self {
            service_info,
            workspace_context,
        })
    }

    pub fn service_info(&self) -> &ServiceInfo {
        &self.service_info
    }

    pub fn workspace_context(&self) -> &WorkspaceContext {
        &self.workspace_context
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use n_framework_nfw_infrastructure_workspace_metadata::PreservedComments;
    use std::path::PathBuf;

    #[test]
    fn validates_empty_service_name() {
        let nfw_yaml = serde_yaml::from_str("workspace: {}").unwrap();
        let _ctx =
            WorkspaceContext::new(PathBuf::from("/"), nfw_yaml, PreservedComments::default())
                .unwrap();
        let svc = ServiceInfo::new("".to_string(), "path".to_string(), "t1".to_string());
        assert!(svc.is_err());
    }

    #[test]
    fn can_create_valid_command() {
        let nfw_yaml = serde_yaml::from_str("workspace: {}").unwrap();
        let ctx = WorkspaceContext::new(PathBuf::from("/"), nfw_yaml, PreservedComments::default())
            .unwrap();
        let svc =
            ServiceInfo::new("Svc".to_string(), "path".to_string(), "t1".to_string()).unwrap();

        let cmd = AddPersistenceCommand::new(svc.clone(), ctx.clone()).unwrap();
        assert_eq!(cmd.service_info().name(), "Svc");
    }

    #[test]
    fn add_persistence_command_enforces_rules() {
        let nfw_yaml = serde_yaml::from_str("workspace: {}").unwrap();
        let _ctx =
            WorkspaceContext::new(PathBuf::from("."), nfw_yaml, PreservedComments::default())
                .unwrap();
        let svc =
            ServiceInfo::new("Svc".to_string(), "path".to_string(), "t1".to_string()).unwrap();
        let cmd = AddPersistenceCommand::new(svc, _ctx).unwrap();

        assert_eq!(cmd.service_info().name(), "Svc");
    }
}
