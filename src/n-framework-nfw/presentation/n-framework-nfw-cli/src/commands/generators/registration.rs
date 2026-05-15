use n_framework_core_cli_abstractions::CliCommandSpec;

use crate::commands::generators::add::registration::register as add_register;
use crate::commands::generators::list::registration::register as list_register;
use crate::commands::generators::refresh::registration::register as refresh_register;
use crate::commands::generators::remove::registration::register as remove_register;

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("generators")
        .with_about("Manage generator sources and discovery")
        .require_subcommand()
        .with_subcommand(list_register())
        .with_subcommand(add_register())
        .with_subcommand(remove_register())
        .with_subcommand(refresh_register())
}
