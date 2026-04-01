mod adapters;
mod args;
mod commands;
mod startup;

use core_cli_rust::CliAdapter;

use crate::adapters::nfw_cli_adapter::NfwCliAdapter;
use crate::args::{CliArgs, CliCommand, TemplatesCommand};
use crate::commands::templates::add_source::AddSourceCliCommand;
use crate::commands::templates::list_templates::TemplatesCliCommand;
use crate::commands::templates::refresh::RefreshTemplatesCliCommand;
use crate::commands::templates::remove_source::RemoveSourceCliCommand;
use crate::startup::cli_bootstrapper::CliBootstrapper;
use crate::startup::cli_service_collection_factory::CliServiceCollection;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let input = std::env::args().skip(1).collect::<Vec<_>>();
    let parsed_command = match NfwCliAdapter::new().parse(&input) {
        Ok(command) => command,
        Err(error) if error.is_help() => {
            print!("{}", error.message());
            return Ok(());
        }
        Err(error) => return Err(error.to_string()),
    };
    let cli_args = CliArgs::from_command(parsed_command.as_ref())?;
    let services = CliBootstrapper::bootstrap()?;

    match cli_args.command {
        CliCommand::Templates(command) => handle_templates_command(command, &services),
    }
}

fn handle_templates_command(
    command: TemplatesCommand,
    services: &CliServiceCollection,
) -> Result<(), String> {
    match command {
        TemplatesCommand::List => {
            TemplatesCliCommand::new(services.list_templates_query_handler.clone()).execute()
        }
        TemplatesCommand::Add { name, url } => {
            AddSourceCliCommand::new(services.templates_service.clone()).execute(&name, &url)
        }
        TemplatesCommand::Remove { name } => {
            RemoveSourceCliCommand::new(services.templates_service.clone()).execute(&name)
        }
        TemplatesCommand::Refresh => {
            RefreshTemplatesCliCommand::new(services.templates_service.clone()).execute()
        }
    }
}
