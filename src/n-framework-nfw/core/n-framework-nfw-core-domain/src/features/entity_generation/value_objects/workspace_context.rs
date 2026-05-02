use std::path::PathBuf;

use super::service_info::ServiceInfo;

#[derive(Debug, Clone)]
pub struct WorkspaceContext {
    root: PathBuf,
    services: Vec<ServiceInfo>,
}

impl WorkspaceContext {
    pub fn new(root: PathBuf, services: Vec<ServiceInfo>) -> Self {
        Self { root, services }
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn services(&self) -> &[ServiceInfo] {
        &self.services
    }

    pub fn default_service(&self) -> Option<&ServiceInfo> {
        if self.services.len() == 1 {
            self.services.first()
        } else {
            None
        }
    }

    pub fn find_service(&self, name: &str) -> Option<&ServiceInfo> {
        self.services.iter().find(|s| s.name() == name)
    }
}
