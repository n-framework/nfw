use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("query")
        .with_about("Generate a mediator query")
        .with_option(
            CliOptionSpec::positional("name", 1).with_help("Query name (e.g. GetProductById)"),
        )
        .with_option(
            CliOptionSpec::new("feature", "feature")
                .with_help("Feature/module name (defaults to query name)"),
        )
        .with_option(CliOptionSpec::new("param", "param").with_help(
            "Comma-separated parameters for the template (e.g. Key=Value,OtherKey=OtherValue)",
        ))
        .with_option(
            CliOptionSpec::new("param-json", "param-json").with_help(
                "JSON string of parameters for the template (e.g. '{\"Key\": \"Value\"}')",
            ),
        )
        .with_option(
            CliOptionSpec::new("no-input", "no-input")
                .with_help("Disable all interactive prompts")
                .flag(),
        )
}
