use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("add")
        .with_about("Register a generator source")
        .with_option(CliOptionSpec::positional("name", 1).with_help("Generator source name"))
        .with_option(CliOptionSpec::positional("url", 2).with_help("Generator source git URL"))
}
