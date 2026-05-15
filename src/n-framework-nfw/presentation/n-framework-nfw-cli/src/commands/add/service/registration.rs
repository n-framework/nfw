use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("service")
        .with_about("Generate a service from a service generator")
        .with_option(CliOptionSpec::positional("name", 1).with_help("Service name"))
        .with_option(
            CliOptionSpec::new("generator", "generator").with_help("Service generator identifier"),
        )
        .with_option(
            CliOptionSpec::new("no-input", "no-input")
                .with_help("Disable all interactive prompts")
                .flag(),
        )
}
