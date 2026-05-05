use crate::features::template_management::services::artifact_generation_service::{
    ServiceInfo, WorkspaceContext,
};

#[derive(Debug, Clone)]
pub struct AddPersistenceCommand {
    service_info: ServiceInfo,
    workspace_context: WorkspaceContext,
    presentation_layer: String,
}

impl AddPersistenceCommand {
    pub const GENERATOR_TYPE: &'static str = "persistence";

    pub fn new(
        service_info: ServiceInfo,
        workspace_context: WorkspaceContext,
        presentation_layer: String,
    ) -> Result<Self, String> {
        if presentation_layer.is_empty() {
            return Err("Presentation layer cannot be empty".to_string());
        }
        Ok(Self {
            service_info,
            workspace_context,
            presentation_layer,
        })
    }

    pub fn service_info(&self) -> &ServiceInfo {
        &self.service_info
    }

    pub fn workspace_context(&self) -> &WorkspaceContext {
        &self.workspace_context
    }

    pub fn presentation_layer(&self) -> &str {
        &self.presentation_layer
    }
}

#[cfg(test)]
#[path = "add_persistence_command.tests.rs"]
mod tests;
