/// Generator-related constants used across the generator management system
pub mod generator {
    /// Name of the generator metadata file
    pub const METADATA_FILE: &str = "nfw.generator.yaml";

    /// Name of the generator content directory
    pub const CONTENT_DIR: &str = "content";

    /// Name of the ignore file for generator discovery
    pub const IGNORE_FILE: &str = ".nfwignore";
}

/// Workspace metadata file constants
pub mod workspace {
    /// Name of the workspace metadata file
    pub const METADATA_FILE: &str = "nfw.yaml";
}

/// YAML key constants for workspace metadata structure
pub mod yaml_keys {
    /// Root workspace key
    pub const WORKSPACE: &str = "workspace";

    /// Services collection key
    pub const SERVICES: &str = "services";

    /// Modules collection key (under service)
    pub const MODULES: &str = "modules";

    /// Generator sources configuration key
    pub const GENERATOR_SOURCES: &str = "generator_sources";

    /// Service path key
    pub const PATH: &str = "path";

    /// Service generator configuration key
    pub const GENERATOR: &str = "generator";

    /// Generator identifier key
    pub const ID: &str = "id";

    /// Generator name key
    pub const NAME: &str = "name";

    /// Generator version key
    pub const VERSION: &str = "version";

    /// Generator steps collection key
    pub const STEPS: &str = "steps";

    /// Step action key
    pub const ACTION: &str = "action";

    /// Step source file key
    pub const SOURCE: &str = "source";

    /// Step destination file key
    pub const DESTINATION: &str = "destination";

    /// Step command key (for run_command action)
    pub const COMMAND: &str = "command";

    /// Step files collection key (for inject action)
    pub const FILES: &str = "files";

    /// Step target file key (for inject action)
    pub const TARGET: &str = "target";

    /// Step content key (for inject action)
    pub const CONTENT: &str = "content";

    /// Generators sub-generator mapping key
    pub const GENERATORS: &str = "generators";

    /// Tags key for generator metadata
    pub const TAGS: &str = "tags";

    /// Project GUID key (legacy, removed on write)
    pub const PROJECT_GUID: &str = "projectGuid";
}

/// Generator step action values
pub mod actions {
    /// Render a single template file
    pub const RENDER: &str = "render";

    /// Render an entire folder of templates
    pub const RENDER_FOLDER: &str = "render_folder";

    /// Inject generated content into an existing file
    pub const INJECT: &str = "inject";

    /// Execute a shell command
    pub const RUN_COMMAND: &str = "run_command";
}

/// Official generator source configuration
pub mod source {
    /// Name of the official nfw generator source
    pub const OFFICIAL_NAME: &str = "official";

    /// URL of the official nfw generator repository
    pub const OFFICIAL_URL: &str = "https://github.com/n-framework/nfw-generators";

    /// Directory under the source root where generators are primarily stored
    pub const GENERATORS_ROOT_DIR: &str = "src";
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
