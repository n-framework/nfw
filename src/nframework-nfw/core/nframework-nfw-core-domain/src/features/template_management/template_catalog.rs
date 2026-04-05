use crate::features::template_management::template_descriptor::TemplateDescriptor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateCatalog {
    pub source_name: String,
    pub templates: Vec<TemplateDescriptor>,
}

impl TemplateCatalog {
    pub fn new(source_name: String, templates: Vec<TemplateDescriptor>) -> Self {
        Self {
            source_name,
            templates,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    pub fn len(&self) -> usize {
        self.templates.len()
    }
}
