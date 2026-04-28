use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("mediator")
        .with_about("Add mediator module to a service")
        .with_option(
            CliOptionSpec::new("service", "service").with_help("Service name to add mediator to"),
        )
        .with_option(
            CliOptionSpec::new("no-input", "no-input")
                .with_help("Disable all interactive prompts")
                .flag(),
        )
}
