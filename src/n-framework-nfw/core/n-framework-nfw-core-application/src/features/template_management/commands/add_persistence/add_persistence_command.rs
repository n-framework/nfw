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
#[path = "add_persistence_command.tests.rs"]
mod tests;
