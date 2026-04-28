use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("service")
        .with_about("Generate a service from a service template")
        .with_option(CliOptionSpec::positional("name", 1).with_help("Service name"))
        .with_option(
            CliOptionSpec::new("template", "template").with_help("Service template identifier"),
        )
        .with_option(
            CliOptionSpec::new("no-input", "no-input")
                .with_help("Disable all interactive prompts")
                .flag(),
        )
}
