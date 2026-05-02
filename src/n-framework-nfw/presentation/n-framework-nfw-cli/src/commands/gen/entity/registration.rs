use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("entity")
        .with_about("Generate an entity with schema and template files")
        .with_option(
            CliOptionSpec::positional("name", 1)
                .with_help("Entity name in PascalCase (e.g. Product, OrderItem)"),
        )
        .with_option(
            CliOptionSpec::new("properties", "properties")
                .with_help("Comma-separated properties in Name:Type format (e.g. Name:string,Price:decimal?,Active:bool)"),
        )
        .with_option(
            CliOptionSpec::new("id-type", "id-type")
                .with_help("Type for the entity primary key (default: Guid). Supported: int, long, Guid, string"),
        )
        .with_option(
            CliOptionSpec::new("entity-type", "entity-type")
                .with_help("Base entity type (default: entity). Options: entity, auditable_entity, soft_deletable_entity"),
        )
        .with_option(
            CliOptionSpec::new("from-schema", "from-schema")
                .with_help("Path to an existing entity schema YAML file to generate from"),
        )
        .with_option(
            CliOptionSpec::new("schema-only", "schema-only")
                .with_help("Only generate the schema YAML file, skip template execution")
                .flag(),
        )
        .with_option(
            CliOptionSpec::new("service", "service")
                .with_help("Service name to generate the entity for"),
        )
        .with_option(
            CliOptionSpec::new("feature", "feature")
                .with_help("Feature within the service to generate the entity for"),
        )
        .with_option(
            CliOptionSpec::new("no-input", "no-input")
                .with_help("Disable all interactive prompts")
                .flag(),
        )
}
