use n_framework_nfw_core_domain::features::template_management::template_descriptor::TemplateDescriptor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateSelectionResult {
    pub source_name: String,
    pub template: TemplateDescriptor,
    pub warnings: Vec<String>,
}
