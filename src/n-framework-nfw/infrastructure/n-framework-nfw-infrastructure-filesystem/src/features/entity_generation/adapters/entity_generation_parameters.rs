use n_framework_nfw_core_domain::features::entity_generation::value_objects::general_type::GeneralType;
use std::path::{Path, PathBuf};

/// Parameters for an entity generation request.
///
/// This serves as a data-carrying object between infrastructure adapters
/// and the core application logic, ensuring all necessary context is available
/// for the generation engine while maintaining proper encapsulation.
#[derive(Debug, Clone, Default)]
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
    /// Returns a new builder for EntityGenerationParameters.
    pub fn builder() -> EntityGenerationParametersBuilder {
        EntityGenerationParametersBuilder::default()
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

/// Builder for EntityGenerationParameters to handle complex construction
/// with many arguments.
#[derive(Debug, Clone, Default)]
pub struct EntityGenerationParametersBuilder {
    entity_name: String,
    namespace: String,
    id_type: GeneralType,
    id_type_cli: String,
    properties: Vec<PropertyTemplate>,
    base_class: String,
    service_name: String,
    service_path: PathBuf,
}

impl EntityGenerationParametersBuilder {
    pub fn entity_name(mut self, value: String) -> Self {
        self.entity_name = value;
        self
    }

    pub fn namespace(mut self, value: String) -> Self {
        self.namespace = value;
        self
    }

    pub fn id_type(mut self, value: GeneralType) -> Self {
        self.id_type = value;
        self
    }

    pub fn id_type_cli(mut self, value: String) -> Self {
        self.id_type_cli = value;
        self
    }

    pub fn properties(mut self, value: Vec<PropertyTemplate>) -> Self {
        self.properties = value;
        self
    }

    pub fn base_class(mut self, value: String) -> Self {
        self.base_class = value;
        self
    }

    pub fn service_name(mut self, value: String) -> Self {
        self.service_name = value;
        self
    }

    pub fn service_path(mut self, value: PathBuf) -> Self {
        self.service_path = value;
        self
    }

    pub fn build(self) -> EntityGenerationParameters {
        EntityGenerationParameters {
            entity_name: self.entity_name,
            namespace: self.namespace,
            id_type: self.id_type,
            id_type_cli: self.id_type_cli,
            properties: self.properties,
            base_class: self.base_class,
            service_name: self.service_name,
            service_path: self.service_path,
        }
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
