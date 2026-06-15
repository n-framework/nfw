use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("crud")
        .with_about("Generate complete CRUD scaffolding for an entity")
        .with_long_about(
            "Generate complete CRUD scaffolding for an entity.\n\n\
             Creates Commands (Create, Update, Delete), Queries (GetById, List),\n\
             Handlers, DTOs, Repository contract, concrete EF Core repository,\n\
             DI registration, API Endpoints, and Specifications.\n\n\
             In interactive mode (default), prompts guide you through entity selection,\n\
             feature placement, and optional flags. Use --no-input for CI/scripting.",
        )
        .with_after_help(
            "EXAMPLES:\n  \
             nfw gen crud Product --feature Products\n    \
               Generate CRUD for Product entity in the Products feature\n\n  \
             nfw gen crud Order --feature Orders --param secured=true,cached=true\n    \
               Generate CRUD with security and caching markers\n\n  \
             nfw gen crud Invoice --param no-api=true\n    \
               Generate CRUD without API endpoints\n\n  \
             nfw gen crud Product --param force=true\n    \
               Overwrite existing CRUD files\n\n  \
             nfw gen crud Product --no-input --feature Products\n    \
               Non-interactive mode for CI pipelines\n\n\
             SUPPORTED PARAMETERS:\n  \
             secured=true    Include security/authorization markers\n  \
             cached=true     Include caching markers\n  \
             no-api=true     Skip API endpoint generation\n  \
             force=true      Overwrite existing files without prompting",
        )
        .with_option(
            CliOptionSpec::positional("name", 1)
                .with_help("The name of the target entity (e.g., Product)"),
        )
        .with_option(
            CliOptionSpec::new("feature", "feature").with_help("The target feature or module"),
        )
        .with_option(CliOptionSpec::new("param", "param").with_help(
            "Comma-separated parameters for the generator (e.g. secured=true,cached=true,no-api=true)",
        ))
        .with_option(
            CliOptionSpec::new("param-json", "param-json").with_help(
                "JSON string of parameters for the generator (e.g. '{\"secured\": true, \"cached\": true}')",
            ),
        )
        .with_option(
            CliOptionSpec::new("no-input", "no-input")
                .with_help("Disable all interactive prompts")
                .flag(),
        )
}
