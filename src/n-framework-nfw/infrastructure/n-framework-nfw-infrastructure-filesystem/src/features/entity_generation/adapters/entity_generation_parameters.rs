use n_framework_nfw_core_domain::features::entity_generation::value_objects::general_type::GeneralType;
use std::path::{Path, PathBuf};

/// Parameters for an entity generation request.
///
/// This serves as a data-carrying object between infrastructure adapters
/// and the core application logic, ensuring all necessary context is available
/// for the generation engine while maintaining proper encapsulation.
#[derive(Debug, Clone)]
pub struct EntityGenerationParameters {
    entity_name: String,
    namespace: String,
    id_type: GeneralType,
    id_type_cli: String,
    properties: Vec<PropertyTemplate>,
    base_class: String,
    service_name: String,
    service_path: PathBuf,
}

impl EntityGenerationParameters {
    /// Creates a new set of generation parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        entity_name: String,
        namespace: String,
        id_type: GeneralType,
        id_type_cli: String,
        properties: Vec<PropertyTemplate>,
        base_class: String,
        service_name: String,
        service_path: PathBuf,
    ) -> Self {
        Self {
            entity_name,
            namespace,
            id_type,
            id_type_cli,
            properties,
            base_class,
            service_name,
            service_path,
        }
    }

    pub fn entity_name(&self) -> &str {
        &self.entity_name
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn id_type(&self) -> &GeneralType {
        &self.id_type
    }

    pub fn id_type_cli(&self) -> &str {
        &self.id_type_cli
    }

    pub fn properties(&self) -> &[PropertyTemplate] {
        &self.properties
    }

    pub fn base_class(&self) -> &str {
        &self.base_class
    }

    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub fn service_path(&self) -> &Path {
        &self.service_path
    }
}

/// Represents a single property within a generation request.
#[derive(Debug, Clone)]
pub struct PropertyTemplate {
    name: String,
    general_type: GeneralType,
    nullable: bool,
}

impl PropertyTemplate {
    /// Creates a new property template.
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
