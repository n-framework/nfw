/// Template-related constants used across the template management system
pub mod template {
    /// Name of the template metadata file
    pub const METADATA_FILE: &str = "template.yaml";

    /// Name of the template content directory
    pub const CONTENT_DIR: &str = "content";

    /// Name of the ignore file for template discovery
    pub const IGNORE_FILE: &str = ".nfwignore";
}

/// Official template source configuration
pub mod source {
    /// Name of the official nfw template source
    pub const OFFICIAL_NAME: &str = "official";

    /// URL of the official nfw template repository
    pub const OFFICIAL_URL: &str = "https://github.com/n-framework/nfw-templates";

    /// Directory under the source root where templates are primarily stored
    pub const TEMPLATES_ROOT_DIR: &str = "src";
}

/// Constants related to generation features
pub mod generation {
    /// Default feature name when none is provided
    pub const DEFAULT_FEATURE_NAME: &str = "Common";

    /// Identifier for the PresentationLayer feature
    pub const PRESENTATION_LAYER: &str = "PresentationLayer";

    /// Standard error messages for transactional generation
    pub mod errors {
        pub const ERR_INIT_TRACKER: &str = "Failed to initialize file tracking";
        pub const ERR_YAML_BACKUP: &str = "Secondary failure during rollback (yaml restore)";
        pub const ERR_FILE_CLEANUP: &str = "Secondary failure during rollback (cleanup)";
        pub const ERR_MODULE_EXISTS: &str = "module already exists for service";
    }
}
