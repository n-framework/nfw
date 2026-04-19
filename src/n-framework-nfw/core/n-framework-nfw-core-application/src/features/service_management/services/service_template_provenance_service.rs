use std::path::Path;

use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_template_provenance_record::ServiceTemplateProvenanceRecord;
use crate::features::service_management::services::abstractions::service_provenance_store::ServiceProvenanceStore;

#[derive(Debug, Clone)]
pub struct ServiceTemplateProvenanceService<S>
where
    S: ServiceProvenanceStore,
{
    store: S,
}

impl<S> ServiceTemplateProvenanceService<S>
where
    S: ServiceProvenanceStore,
{
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub fn persist(
        &self,
        workspace_root: &Path,
        service_name: &str,
        template_id: &str,
        template_version: &str,
    ) -> Result<(), AddServiceError> {
        let record = ServiceTemplateProvenanceRecord {
            service_name: service_name.to_owned(),
            template_id: template_id.to_owned(),
            template_version: template_version.to_owned(),
        };

        self.store
            .persist_service_provenance(workspace_root, &record)
            .map_err(AddServiceError::ProvenanceWriteFailed)
    }
}
