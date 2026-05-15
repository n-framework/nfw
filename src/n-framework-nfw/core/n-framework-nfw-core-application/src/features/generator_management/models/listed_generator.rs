use n_framework_nfw_core_domain::features::generator_management::language::Language;
use n_framework_nfw_core_domain::features::versioning::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedGenerator {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: Version,
    pub language: Language,
    pub source_name: String,
}
