use n_framework_core_cli_abstractions::CliCommandSpec;

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("add")
        .with_about("Create workspace artifacts")
        .require_subcommand()
        .with_subcommand(crate::commands::add::service::registration::register())
        .with_subcommand(crate::commands::add::mediator::registration::register())
}
