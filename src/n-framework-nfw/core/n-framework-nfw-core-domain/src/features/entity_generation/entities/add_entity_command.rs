use std::path::PathBuf;

use super::super::value_objects::general_type::GeneralType;
use super::super::value_objects::property_definition::PropertyDefinition;

#[derive(Debug, Clone, Default)]
pub struct EntityGenerationOptions {
    pub service_name: Option<String>,
    pub feature: String,
    pub schema_only: bool,
    pub from_schema: Option<PathBuf>,
    pub non_interactive: bool,
}

#[derive(Debug, Clone)]
pub struct AddEntityCommand {
    entity_name: String,
    properties: Vec<PropertyDefinition>,
    id_type: GeneralType,
    entity_type: EntityType,
    options: EntityGenerationOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EntityType {
    #[default]
    Entity,
    AuditableEntity,
    SoftDeletableEntity,
}

impl EntityType {
    pub fn as_schema_value(&self) -> &str {
        match self {
            Self::Entity => "entity",
            Self::AuditableEntity => "auditable_entity",
            Self::SoftDeletableEntity => "soft_deletable_entity",
        }
    }

    pub fn from_str_value(s: &str) -> Option<Self> {
        match s {
            "entity" => Some(Self::Entity),
            "auditable_entity" => Some(Self::AuditableEntity),
            "soft_deletable_entity" => Some(Self::SoftDeletableEntity),
            _ => None,
        }
    }
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_schema_value())
    }
}

impl AddEntityCommand {
    pub fn new(
        entity_name: String,
        properties: Vec<PropertyDefinition>,
        id_type: GeneralType,
        entity_type: EntityType,
        options: EntityGenerationOptions,
    ) -> Self {
        Self {
            entity_name,
            properties,
            id_type,
            entity_type,
            options,
        }
    }

    pub fn entity_name(&self) -> &str {
        &self.entity_name
    }

    pub fn properties(&self) -> &[PropertyDefinition] {
        &self.properties
    }

    pub fn id_type(&self) -> &GeneralType {
        &self.id_type
    }

    pub fn entity_type(&self) -> &EntityType {
        &self.entity_type
    }

    pub fn service_name(&self) -> Option<&str> {
        self.options.service_name.as_deref()
    }

    pub fn feature(&self) -> &str {
        &self.options.feature
    }

    pub fn is_schema_only(&self) -> bool {
        self.options.schema_only
    }

    pub fn from_schema(&self) -> Option<&PathBuf> {
        self.options.from_schema.as_ref()
    }

    pub fn is_non_interactive(&self) -> bool {
        self.options.non_interactive
    }
}
