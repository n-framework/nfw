use n_framework_core_cli_abstractions::CliCommandSpec;

use crate::commands::templates::add::registration::register as add_register;
use crate::commands::templates::list::registration::register as list_register;
use crate::commands::templates::refresh::registration::register as refresh_register;
use crate::commands::templates::remove::registration::register as remove_register;

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("templates")
        .with_about("Manage template sources and discovery")
        .require_subcommand()
        .with_subcommand(list_register())
        .with_subcommand(add_register())
        .with_subcommand(remove_register())
        .with_subcommand(refresh_register())
}
