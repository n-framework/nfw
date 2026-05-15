use std::path::Path;

use crate::features::service_management::models::service_generator_provenance_record::ServiceGeneratorProvenanceRecord;

pub trait ServiceProvenanceStore {
    fn persist_service_provenance(
        &self,
        workspace_root: &Path,
        record: &ServiceGeneratorProvenanceRecord,
    ) -> Result<(), String>;
}
