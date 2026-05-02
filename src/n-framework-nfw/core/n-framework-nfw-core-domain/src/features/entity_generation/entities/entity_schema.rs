use serde::{Deserialize, Serialize};

use super::super::value_objects::general_type::GeneralType;
use super::add_entity_command::{AddEntityCommand, EntityType};

/// Represents the persistence schema for an entity.
///
/// This is typically serialized to a YAML file in the service's `specs` directory
/// and serves as the source of truth for code generation when using `--from-schema`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitySchema {
    entity: String,
    id_type: GeneralType,
    entity_type: EntityType,
    properties: Vec<SchemaProperty>,
}

/// A property definition within an `EntitySchema`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaProperty {
    name: String,
    #[serde(rename = "type")]
    general_type: GeneralType,
    nullable: bool,
}

impl SchemaProperty {
    pub fn new(name: String, general_type: GeneralType, nullable: bool) -> Self {
        Self {
            name,
            general_type,
            nullable,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn general_type(&self) -> &GeneralType {
        &self.general_type
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }
}

impl EntitySchema {
    pub fn new(
        entity_name: String,
        id_type: GeneralType,
        entity_type: EntityType,
        properties: Vec<SchemaProperty>,
    ) -> Self {
        Self {
            entity: entity_name,
            id_type,
            entity_type,
            properties,
        }
    }

    pub fn from_command(command: &AddEntityCommand) -> Self {
        Self::new(
            command.entity_name().to_string(),
            command.id_type().clone(),
            command.entity_type().clone(),
            command
                .properties()
                .iter()
                .map(|p| {
                    SchemaProperty::new(
                        p.name().to_string(),
                        p.general_type().clone(),
                        p.nullable(),
                    )
                })
                .collect(),
        )
    }

    pub fn entity(&self) -> &str {
        &self.entity
    }

    pub fn id_type(&self) -> &GeneralType {
        &self.id_type
    }

    pub fn entity_type(&self) -> &EntityType {
        &self.entity_type
    }

    pub fn properties(&self) -> &[SchemaProperty] {
        &self.properties
    }
}

#[cfg(test)]
#[path = "entity_schema.tests.rs"]
mod tests;
