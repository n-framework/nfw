//! Global constants used across the N-Framework CLI for consistent directory naming
//! and validation labels.
//!
//! These constants define the directory structure conventions used for identifying
//! features, specifications, and entities within a service.

pub struct GlobalConstants;

impl GlobalConstants {
    pub const FEATURES_DIR: &'static str = "features";
    pub const SPECS_DIR: &'static str = "specs";
    pub const ENTITIES_DIR: &'static str = "entities";
    pub const ENTITY_LABEL: &'static str = "entity";
    pub const SERVICE_LABEL: &'static str = "service";
    pub const PROPERTY_LABEL: &'static str = "property";
    pub const ENTITY_SCHEMA_PATH: &'static str =
        "https://raw.githubusercontent.com/n-framework/nfw/main/src/nfw/schemas/entity.schema.json";
}
