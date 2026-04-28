use crate::features::template_management::services::artifact_generation_service::{
    ServiceInfo, WorkspaceContext,
};

#[derive(Debug, Clone)]
pub struct AddMediatorCommand {
    pub service_info: ServiceInfo,
    pub workspace_context: WorkspaceContext,
}

impl AddMediatorCommand {
    pub fn new(service_info: ServiceInfo, workspace_context: WorkspaceContext) -> Self {
        Self {
            service_info,
            workspace_context,
        }
    }
}
