use std::path::Path;

use crate::features::service_management::models::service_template_provenance_record::ServiceTemplateProvenanceRecord;

pub trait ServiceProvenanceStore {
    fn persist_service_provenance(
        &self,
        workspace_root: &Path,
        record: &ServiceTemplateProvenanceRecord,
    ) -> Result<(), String>;
}
