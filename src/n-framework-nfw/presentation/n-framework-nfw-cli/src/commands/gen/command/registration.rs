use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("command")
        .with_about("Generate a mediator command")
        .with_option(
            CliOptionSpec::positional("name", 1).with_help("Command name (e.g. CreateProduct)"),
        )
        .with_option(
            CliOptionSpec::new("feature", "feature").with_help("The target feature or module"),
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
