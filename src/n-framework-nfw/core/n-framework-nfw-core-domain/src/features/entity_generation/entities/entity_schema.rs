use serde::{Deserialize, Serialize};

use super::super::value_objects::general_type::GeneralType;
use super::add_entity_command::EntityType;

/// Language-agnostic entity schema stored as YAML.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct EntitySchema {
    pub entity: String,
    pub id_type: GeneralType,
    pub entity_type: String,
    pub properties: Vec<SchemaProperty>,
}

/// A single property within an entity schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct SchemaProperty {
    pub name: String,
    #[serde(rename = "type")]
    pub general_type: GeneralType,
    pub nullable: bool,
}

impl EntitySchema {
    pub fn new(
        entity_name: String,
        id_type: GeneralType,
        entity_type: &EntityType,
        properties: Vec<SchemaProperty>,
    ) -> Self {
        Self {
            entity: entity_name,
            id_type,
            entity_type: entity_type.as_schema_value().to_owned(),
            properties,
        }
    }

    pub fn entity_type_parsed(&self) -> Option<EntityType> {
        EntityType::from_str_value(&self.entity_type)
    }
}

#[cfg(test)]
#[path = "entity_schema.tests.rs"]
mod tests;
