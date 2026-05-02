use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntityGenerationError {
    #[error("invalid entity name '{name}': {reason}")]
    InvalidEntityName { name: String, reason: String },

    #[error("unsupported property type '{cli_type}'. Supported types: {supported}")]
    UnsupportedPropertyType { cli_type: String, supported: String },

    #[error("invalid property syntax '{input}': expected format 'Name:Type' or 'Name:Type?'")]
    InvalidPropertySyntax { input: String },

    #[error("duplicate property name '{name}'")]
    DuplicatePropertyName { name: String },

    #[error("at least one property must be defined")]
    EmptyProperties,

    #[error("no services found in workspace. Add a service first with: nfw add service")]
    NoServicesInWorkspace,

    #[error("service '{name}' not found in workspace")]
    ServiceNotFound { name: String },

    #[error("feature '{feature}' not found in service")]
    FeatureNotFound { feature: String },

    #[error(
        "service '{service_name}' does not have the persistence module. \
         Add it with: nfw add persistence --service {service_name}"
    )]
    MissingPersistenceModule { service_name: String },

    #[error("unsupported ID type '{id_type}'. Supported types: integer, uuid, string")]
    UnsupportedIdType { id_type: String },

    #[error("schema file already exists at '{path}'. Use --from-schema to generate from it")]
    SchemaFileConflict { path: PathBuf },

    #[error("schema file not found at '{path}'")]
    SchemaFileNotFound { path: PathBuf },

    #[error("failed to read schema file '{path}': {reason}")]
    SchemaReadError { path: PathBuf, reason: String },

    #[error("failed to write schema file '{path}': {reason}")]
    SchemaWriteError { path: PathBuf, reason: String },

    #[error("invalid schema content in '{path}': {reason}")]
    InvalidSchemaContent { path: PathBuf, reason: String },

    #[error("failed to create entity specs directory '{path}': {reason}")]
    DirectoryCreationError { path: PathBuf, reason: String },

    #[error("entity specs path '{path}' exists but is not a directory")]
    SpecsPathNotDirectory { path: PathBuf },

    #[error("template execution failed: {reason}")]
    TemplateExecutionError { reason: String },

    #[error("interactive prompt failed: {reason}")]
    PromptError { reason: String },

    #[error("workspace error: {reason}")]
    WorkspaceError { reason: String },

    #[error("configuration error: {reason}")]
    ConfigError { reason: String },

    #[error("{0}")]
    Internal(String),
}
