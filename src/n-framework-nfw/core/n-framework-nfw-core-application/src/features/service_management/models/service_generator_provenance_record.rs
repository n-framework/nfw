#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceGeneratorProvenanceRecord {
    pub service_name: String,
    pub generator_id: String,
    pub generator_version: String,
}
