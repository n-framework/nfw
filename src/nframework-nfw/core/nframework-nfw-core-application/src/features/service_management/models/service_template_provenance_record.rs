#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceTemplateProvenanceRecord {
    pub service_name: String,
    pub template_id: String,
    pub template_version: String,
    pub generated_at_utc: String,
}
