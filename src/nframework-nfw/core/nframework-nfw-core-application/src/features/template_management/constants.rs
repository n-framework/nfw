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
