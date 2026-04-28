use n_framework_core_cli_abstractions::CliCommandSpec;

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("templates")
        .with_about("Manage template sources and discovery")
        .require_subcommand()
        .with_subcommand(crate::commands::templates::list::registration::register())
        .with_subcommand(crate::commands::templates::add::registration::register())
        .with_subcommand(crate::commands::templates::remove::registration::register())
        .with_subcommand(crate::commands::templates::refresh::registration::register())
}
