use crate::features::template_management::services::artifact_generation_service::{
    ServiceInfo, WorkspaceContext,
};

#[derive(Debug, Clone)]
pub struct AddWebApiCommand {
    service_info: ServiceInfo,
    workspace_context: WorkspaceContext,
}

impl AddWebApiCommand {
    pub const GENERATOR_TYPE: &'static str = "webapi";

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
