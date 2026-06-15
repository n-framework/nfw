use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("crud")
        .with_about("Generate complete CRUD scaffolding for an entity")
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
