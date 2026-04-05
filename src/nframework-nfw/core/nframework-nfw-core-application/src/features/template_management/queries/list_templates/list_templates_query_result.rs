use crate::features::template_management::models::listed_template::ListedTemplate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTemplatesQueryResult {
    pub templates: Vec<ListedTemplate>,
    pub warnings: Vec<String>,
}
