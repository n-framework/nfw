use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("webapi")
        .with_about("Add webapi module to a service")
        .with_option(
            CliOptionSpec::new("service", "service").with_help("Service name to add webapi to"),
        )
        .with_option(
            CliOptionSpec::new("no-input", "no-input")
                .with_help("Disable all interactive prompts")
                .flag(),
        )
}
