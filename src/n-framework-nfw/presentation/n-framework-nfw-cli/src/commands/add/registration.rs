use n_framework_core_cli_abstractions::CliCommandSpec;

use crate::commands::add::mediator::registration::register as mediator_register;
use crate::commands::add::persistence::registration::register as persistence_register;
use crate::commands::add::service::registration::register as service_register;
use crate::commands::add::webapi::registration::register as webapi_register;

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("add")
        .with_about("Create workspace artifacts")
        .require_subcommand()
        .with_subcommand(service_register())
        .with_subcommand(mediator_register())
        .with_subcommand(persistence_register())
        .with_subcommand(webapi_register())
}
