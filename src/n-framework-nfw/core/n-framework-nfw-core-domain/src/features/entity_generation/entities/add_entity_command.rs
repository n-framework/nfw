use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

use super::super::errors::entity_generation_error::EntityGenerationError;
use super::super::value_objects::general_type::GeneralType;
use super::super::value_objects::global_constants::GlobalConstants;
use super::super::value_objects::property_definition::PropertyDefinition;
use super::super::value_objects::validation_utils::ValidationUtils;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityGenerationOptions {
    service_name: Option<String>,
    feature: String,
    schema_only: bool,
    from_schema: Option<PathBuf>,
    non_interactive: bool,
}

impl EntityGenerationOptions {
    pub fn new(
        service_name: Option<String>,
        feature: String,
        schema_only: bool,
        from_schema: Option<PathBuf>,
        non_interactive: bool,
    ) -> Self {
        Self {
            service_name,
            feature,
            schema_only,
            from_schema,
            non_interactive,
        }
    }

    pub fn service_name(&self) -> Option<&str> {
        self.service_name.as_deref()
    }

    pub fn feature(&self) -> &str {
        &self.feature
    }

    pub fn schema_only(&self) -> bool {
        self.schema_only
    }

    pub fn from_schema(&self) -> Option<&PathBuf> {
        self.from_schema.as_ref()
    }

    pub fn non_interactive(&self) -> bool {
        self.non_interactive
    }
}

/// Domain command for adding a new entity to a service.
///
/// This command contains all the necessary data to generate both the
/// persistence schema file and the actual entity source code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddEntityCommand {
    entity_name: String,
    properties: Vec<PropertyDefinition>,
    id_type: GeneralType,
    entity_type: EntityType,
    options: EntityGenerationOptions,
}

/// Represents the type of entity being generated.
///
/// Different entity types trigger different templates and add specific
/// metadata or base classes (e.g., Auditable, SoftDeletable).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    /// A standard domain entity.
    #[default]
    Entity,
    /// An entity that tracks creation and modification timestamps/users.
    AuditableEntity,
    /// An entity that supports logical deletion instead of physical deletion.
    SoftDeletableEntity,
}

impl EntityType {
    /// Returns the string representation used in schema files.
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
    /// Attempts to create a new AddEntityCommand with validation.
    ///
    /// # Errors
    ///
    /// Returns `EntityGenerationError::InvalidEntityName` if the name violates
    /// naming conventions (PascalCase, alphanumeric).
    pub fn try_new(
        entity_name: String,
        properties: Vec<PropertyDefinition>,
        id_type: GeneralType,
        entity_type: EntityType,
        options: EntityGenerationOptions,
    ) -> Result<Self, EntityGenerationError> {
        ValidationUtils::validate_pascal_case(&entity_name, GlobalConstants::ENTITY_LABEL)?;

        if properties.is_empty() && options.from_schema().is_none() {
            return Err(EntityGenerationError::EmptyProperties);
        }

        let mut names = HashSet::new();
        for prop in &properties {
            if !names.insert(prop.name().to_string()) {
                return Err(EntityGenerationError::DuplicatePropertyName {
                    name: prop.name().to_string(),
                });
            }
        }

        Ok(Self {
            entity_name,
            properties,
            id_type,
            entity_type,
            options,
        })
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
        self.options.service_name()
    }

    pub fn feature(&self) -> &str {
        self.options.feature()
    }

    pub fn is_schema_only(&self) -> bool {
        self.options.schema_only()
    }

    pub fn from_schema(&self) -> Option<&PathBuf> {
        self.options.from_schema()
    }

    pub fn is_non_interactive(&self) -> bool {
        self.options.non_interactive()
    }
}

#[cfg(test)]
#[path = "add_entity_command.tests.rs"]
mod tests;
