use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("repository")
        .with_about("Generate a new repository")
        .with_option(
            CliOptionSpec::positional("name", 1)
                .with_help("Name of the entity to generate a repository for"),
        )
        .with_option(CliOptionSpec::new("feature", "feature").with_help("Target feature folder"))
        .with_option(
            CliOptionSpec::new("service", "service")
                .with_help("Target service (if multiple exist)"),
        )
        .with_option(
            CliOptionSpec::new("no-input", "no-input")
                .with_help("Disable interactive prompts")
                .flag(),
        )
}
