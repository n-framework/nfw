use std::path::Path;

use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_generator_provenance_record::ServiceGeneratorProvenanceRecord;
use crate::features::service_management::services::abstractions::service_provenance_store::ServiceProvenanceStore;

#[derive(Debug, Clone)]
pub struct ServiceGeneratorProvenanceService<S>
where
    S: ServiceProvenanceStore,
{
    store: S,
}

impl<S> ServiceGeneratorProvenanceService<S>
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
        generator_id: &str,
        generator_version: &str,
    ) -> Result<(), AddServiceError> {
        let record = ServiceGeneratorProvenanceRecord {
            service_name: service_name.to_owned(),
            generator_id: generator_id.to_owned(),
            generator_version: generator_version.to_owned(),
        };

        self.store
            .persist_service_provenance(workspace_root, &record)
            .map_err(AddServiceError::ProvenanceWriteFailed)
    }
}
