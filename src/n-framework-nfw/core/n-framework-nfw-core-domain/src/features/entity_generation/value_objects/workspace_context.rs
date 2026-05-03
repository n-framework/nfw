use super::service_info::ServiceInfo;
use crate::features::entity_generation::errors::entity_generation_error::EntityGenerationError;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct WorkspaceContext {
    root: PathBuf,
    services: Vec<ServiceInfo>,
}

impl WorkspaceContext {
    pub fn new(root: PathBuf, services: Vec<ServiceInfo>) -> Self {
        Self { root, services }
    }

    pub fn try_new(
        root: PathBuf,
        services: Vec<ServiceInfo>,
    ) -> Result<Self, EntityGenerationError> {
        if !root.exists() {
            return Err(EntityGenerationError::WorkspaceError {
                reason: format!(
                    "Workspace root directory does not exist: {}",
                    root.display()
                ),
            });
        }

        if services.is_empty() {
            return Err(EntityGenerationError::NoServicesInWorkspace);
        }

        let mut names = HashSet::new();
        for service in &services {
            if !names.insert(service.name().to_string()) {
                return Err(EntityGenerationError::InvalidEntityName {
                    name: service.name().to_string(),
                    reason: "duplicate service name in workspace".to_string(),
                });
            }
        }
        Ok(Self::new(root, services))
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
