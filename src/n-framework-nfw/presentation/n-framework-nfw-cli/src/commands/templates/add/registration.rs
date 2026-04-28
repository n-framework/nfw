use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("add")
        .with_about("Register a template source")
        .with_option(
            CliOptionSpec::new("name", "name")
                .with_help("Template source name")
                .required(),
        )
        .with_option(
            CliOptionSpec::new("url", "url")
                .with_help("Template source git URL")
                .required(),
        )
}
