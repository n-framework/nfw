use n_framework_nfw_core_domain::features::generator_management::generator_descriptor::GeneratorDescriptor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorSelectionResult {
    pub source_name: String,
    pub generator: GeneratorDescriptor,
    pub warnings: Vec<String>,
}
