use n_framework_core_cli_abstractions::{CliCommandSpec, CliOptionSpec};

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("new")
        .with_about("Create a new workspace")
        .with_option(
            CliOptionSpec::positional("workspace-name", 1)
                .with_help("Workspace name (required in non-interactive mode)"),
        )
        .with_option(
            CliOptionSpec::new("template", "template")
                .with_help("Template identifier (qualified or unqualified)"),
        )
        .with_option(
            CliOptionSpec::new("no-input", "no-input")
                .with_help("Disable all interactive prompts")
                .flag(),
        )
}
