use crate::cli_error::CliError;
use crate::startup::cli_service_collection_factory::CliServiceCollection;

pub struct GenCrudRequest<'a> {
    pub entity_name: Option<&'a str>,
    pub no_api: bool,
    pub secured: bool,
    pub cached: bool,
    pub force: bool,
    pub no_input: bool,
    pub is_interactive_terminal: bool,
}

pub fn handle(
    command: &dyn n_framework_core_cli_abstractions::Command,
    _context: &CliServiceCollection,
) -> Result<(), String> {
    use std::io::{self, IsTerminal};
    let is_interactive_terminal = io::stdin().is_terminal() && io::stdout().is_terminal();

    let _request = GenCrudRequest {
        entity_name: command.option("entity-name"),
        no_api: command.option("no-api").is_some(),
        secured: command.option("secured").is_some(),
        cached: command.option("cached").is_some(),
        force: command.option("force").is_some(),
        no_input: command.option("no-input").is_some(),
        is_interactive_terminal,
    };

    // TODO: Phase 2 - Foundational Validation
    // TODO: Phase 3 - Orchestration
    // TODO: Phase 4 - Interactive Prompts

    Err(CliError::internal("CRUD generation is not yet implemented").to_string())
}
