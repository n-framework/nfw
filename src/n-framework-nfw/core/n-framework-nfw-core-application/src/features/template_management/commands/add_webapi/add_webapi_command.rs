use crate::features::template_management::services::artifact_generation_service::{
    ServiceInfo, WorkspaceContext,
};

/// Configuration options for WebAPI generation features.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WebApiConfig {
    pub use_openapi: bool,
    pub use_health_checks: bool,
    pub use_cors: bool,
    pub use_problem_details: bool,
}

impl Default for WebApiConfig {
    fn default() -> Self {
        Self {
            use_openapi: true,
            use_health_checks: true,
            use_cors: true,
            use_problem_details: true,
        }
    }
}

impl WebApiConfig {
    pub fn new(
        use_openapi: bool,
        use_health_checks: bool,
        use_cors: bool,
        use_problem_details: bool,
    ) -> Self {
        Self {
            use_openapi,
            use_health_checks,
            use_cors,
            use_problem_details,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddWebApiCommand {
    service_info: ServiceInfo,
    workspace_context: WorkspaceContext,
    config: WebApiConfig,
}

impl AddWebApiCommand {
    pub const GENERATOR_TYPE: &'static str = "webapi";

    pub fn new(
        service_info: ServiceInfo,
        workspace_context: WorkspaceContext,
        config: WebApiConfig,
    ) -> Self {
        Self {
            service_info,
            workspace_context,
            config,
        }
    }

    pub fn service_info(&self) -> &ServiceInfo {
        &self.service_info
    }

    pub fn workspace_context(&self) -> &WorkspaceContext {
        &self.workspace_context
    }

    pub fn config(&self) -> WebApiConfig {
        self.config
    }
}
