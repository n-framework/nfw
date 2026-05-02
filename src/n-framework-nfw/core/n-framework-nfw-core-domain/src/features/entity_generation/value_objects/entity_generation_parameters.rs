use std::path::PathBuf;

use super::general_type::GeneralType;

#[derive(Debug, Clone)]
pub struct EntityGenerationParameters {
    pub entity_name: String,
    pub namespace: String,
    pub id_type: GeneralType,
    pub id_type_cli: String,
    pub properties: Vec<PropertyTemplate>,
    pub base_class: String,
    pub service_name: String,
    pub service_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PropertyTemplate {
    pub name: String,
    pub general_type: GeneralType,
    pub nullable: bool,
}
