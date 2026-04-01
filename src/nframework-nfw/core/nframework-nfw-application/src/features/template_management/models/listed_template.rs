use nframework_nfw_domain::features::template_management::language::Language;
use nframework_nfw_domain::features::versioning::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: Version,
    pub language: Language,
    pub source_name: String,
}
