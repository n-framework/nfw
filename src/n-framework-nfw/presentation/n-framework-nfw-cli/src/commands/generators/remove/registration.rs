use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("remove")
        .with_about("Unregister a generator source")
        .with_option(CliOptionSpec::positional("name", 1).with_help("Generator source name"))
}
