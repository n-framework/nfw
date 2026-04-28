use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("remove")
        .with_about("Unregister a template source")
        .with_option(
            CliOptionSpec::new("name", "name")
                .with_help("Template source name")
                .required(),
        )
}
