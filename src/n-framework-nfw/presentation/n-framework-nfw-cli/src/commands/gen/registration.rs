use n_framework_core_cli_abstractions::CliCommandSpec;

use crate::commands::r#gen::command::registration::register as command_register;
use crate::commands::r#gen::entity::registration::register as entity_register;
use crate::commands::r#gen::query::registration::register as query_register;
use crate::commands::r#gen::repository::registration::register as repository_register;

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("gen")
        .with_about("Generate workspace artifacts from templates")
        .require_subcommand()
        .with_subcommand(command_register())
        .with_subcommand(entity_register())
        .with_subcommand(query_register())
        .with_subcommand(repository_register())
}
