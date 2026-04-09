use std::io::{self, IsTerminal};

use nframework_core_cli_abstractions::{
    CliAppConfig, CliCommandSpec, CliOptionSpec, CliRuntime, CliSpec, Command,
};
use nframework_core_cli_clap::ClapCliRuntimeBuilder;
use nframework_nfw_core_application::features::cli::exit_codes::ExitCodes;
use nframework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;

use crate::cli_error::CliError;
use crate::commands::check::run_check::{RunCheckCliCommand, RunCheckError};
use crate::commands::service::add_service::AddServiceCliCommand;
use crate::commands::templates::add_source::AddSourceCliCommand;
use crate::commands::templates::list_templates::TemplatesCliCommand;
use crate::commands::templates::refresh::RefreshTemplatesCliCommand;
use crate::commands::templates::remove_source::RemoveSourceCliCommand;
use crate::commands::workspace::new_workspace::NewWorkspaceCliCommand;
use crate::startup::cli_service_collection_factory::CliServiceCollection;

const NFRAMEWORK_ASCII_BANNER: &str = r#"
   _  ______                                   __
  / |/ / __/______ ___ _  ___ _    _____  ____/ /__
 /    / _// __/ _ `/  ' \/ -_) |/|/ / _ \/ __/  '_/
/_/|_/_/ /_/  \_,_/_/_/_/\__/|__,__/\___/_/ /_/\_\ "#;

pub fn build_nfw_cli_app_config() -> CliAppConfig {
    CliAppConfig::new(
        CliSpec::new("nfw")
            .with_about("NFramework CLI")
            .require_command()
            .with_command(
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
                    ),
            )
            .with_command(
                CliCommandSpec::new("templates")
                    .with_about("Manage template sources and discovery")
                    .require_subcommand()
                    .with_subcommand(
                        CliCommandSpec::new("list").with_about("List discovered templates"),
                    )
                    .with_subcommand(
                        CliCommandSpec::new("add")
                            .with_about("Register a template source")
                            .with_option(
                                CliOptionSpec::new("name", "name")
                                    .with_help("Template source name")
                                    .required(),
                            )
                            .with_option(
                                CliOptionSpec::new("url", "url")
                                    .with_help("Template source git URL")
                                    .required(),
                            ),
                    )
                    .with_subcommand(
                        CliCommandSpec::new("remove")
                            .with_about("Unregister a template source")
                            .with_option(
                                CliOptionSpec::new("name", "name")
                                    .with_help("Template source name")
                                    .required(),
                            ),
                    )
                    .with_subcommand(
                        CliCommandSpec::new("refresh")
                            .with_about("Refresh template catalogs from sources"),
                    ),
            )
            .with_command(
                CliCommandSpec::new("check")
                    .with_about("Validate workspace architecture boundaries"),
            )
            .with_command(
                CliCommandSpec::new("add")
                    .with_about("Create workspace artifacts")
                    .require_subcommand()
                    .with_subcommand(
                        CliCommandSpec::new("service")
                            .with_about("Generate a service from a service template")
                            .with_option(
                                CliOptionSpec::positional("name", 1).with_help("Service name"),
                            )
                            .with_option(
                                CliOptionSpec::new("template", "template")
                                    .with_help("Service template identifier"),
                            )
                            .with_option(
                                CliOptionSpec::new("no-input", "no-input")
                                    .with_help("Disable all interactive prompts")
                                    .flag(),
                            ),
                    ),
            ),
    )
}

pub fn build_nfw_cli_runtime(services: CliServiceCollection) -> CliRuntime<CliServiceCollection> {
    println!("{NFRAMEWORK_ASCII_BANNER}");
    println!();

    ClapCliRuntimeBuilder::new(build_nfw_cli_app_config(), services)
        .register_handler("new", handle_workspace_new)
        .register_handler("check", handle_check)
        .register_handler("add/service", handle_add_service)
        .register_handler("templates/list", handle_templates_list)
        .register_handler("templates/add", handle_templates_add)
        .register_handler("templates/remove", handle_templates_remove)
        .register_handler("templates/refresh", handle_templates_refresh)
        .build()
}

fn required_option(command: &dyn Command, option_name: &str) -> Result<String, String> {
    command
        .option(option_name)
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("missing required option '--{option_name}'"))
}

fn handle_workspace_new(
    command: &dyn Command,
    context: &CliServiceCollection,
) -> Result<(), String> {
    let no_input = command.option("no-input").is_some();
    let is_interactive_terminal = io::stdin().is_terminal() && io::stdout().is_terminal();

    NewWorkspaceCliCommand::new(context.new_workspace_command_handler.clone())
        .execute(
            command.option("workspace-name"),
            command.option("template"),
            no_input,
            is_interactive_terminal,
        )
        .map_err(|error| {
            let exit_code = ExitCodes::from_workspace_new_error(&error) as i32;
            format!("[exit:{exit_code}] {error}")
        })
}

fn handle_add_service(command: &dyn Command, context: &CliServiceCollection) -> Result<(), String> {
    let no_input = command.option("no-input").is_some();
    let is_interactive_terminal = io::stdin().is_terminal() && io::stdout().is_terminal();

    AddServiceCliCommand::new(context.add_service_command_handler.clone())
        .execute(
            command.option("name"),
            command.option("template"),
            no_input,
            is_interactive_terminal,
        )
        .map_err(|error| {
            let exit_code = match error {
                AddServiceError::Interrupted => 130,
                _ => ExitCodes::from_add_service_error(&error) as i32,
            };
            format!("[exit:{exit_code}] {error}")
        })
}

fn handle_check(_: &dyn Command, context: &CliServiceCollection) -> Result<(), String> {
    RunCheckCliCommand::new(&context.check_command_handler)
        .execute()
        .map_err(|error| {
            let exit_code = match error {
                RunCheckError::ValidationFailed => ExitCodes::ValidationError as i32,
                RunCheckError::Interrupted => ExitCodes::Interrupted as i32,
                RunCheckError::CurrentDirectoryUnavailable(_) => ExitCodes::InternalError as i32,
                RunCheckError::CommandError(_) => ExitCodes::InternalError as i32,
            };

            format!("[exit:{exit_code}] {error}")
        })
}

fn handle_templates_list(_: &dyn Command, context: &CliServiceCollection) -> Result<(), String> {
    TemplatesCliCommand::new(context.list_templates_query_handler.clone()).execute()
}

fn handle_templates_add(
    command: &dyn Command,
    context: &CliServiceCollection,
) -> Result<(), String> {
    let name = required_option(command, "name")?;
    let url = required_option(command, "url")?;
    AddSourceCliCommand::new(context.add_template_source_command_handler.clone())
        .execute(&name, &url)
}

fn handle_templates_remove(
    command: &dyn Command,
    context: &CliServiceCollection,
) -> Result<(), String> {
    let name = required_option(command, "name")?;
    RemoveSourceCliCommand::new(context.remove_template_source_command_handler.clone())
        .execute(&name)
}

fn handle_templates_refresh(_: &dyn Command, context: &CliServiceCollection) -> Result<(), String> {
    RefreshTemplatesCliCommand::new(context.refresh_templates_command_handler.clone()).execute()
}

/// Extension trait to parse exit code from error string protocol.
/// This improves reliability of the exit code extraction with better validation.
pub trait ParseExitCode {
    fn parse_exit_code(&self) -> CliError;
}

impl ParseExitCode for String {
    fn parse_exit_code(&self) -> CliError {
        let Some(rest) = self.strip_prefix("[exit:") else {
            return CliError::internal(self.clone());
        };

        let Some((exit_code_text, message)) = rest.split_once(']') else {
            return CliError::internal(self.clone());
        };

        let Ok(exit_code) = exit_code_text.parse::<i32>() else {
            return CliError::internal(self.clone());
        };

        CliError::new(exit_code, message.trim_start().to_owned())
    }
}

impl ParseExitCode for &str {
    fn parse_exit_code(&self) -> CliError {
        // Directly parse the &str to avoid infinite recursion
        let Some(rest) = self.strip_prefix("[exit:") else {
            return CliError::internal(self.to_owned());
        };

        let Some((exit_code_text, message)) = rest.split_once(']') else {
            return CliError::internal(self.to_owned());
        };

        let Ok(exit_code) = exit_code_text.parse::<i32>() else {
            return CliError::internal(self.to_owned());
        };

        CliError::new(exit_code, message.trim_start().to_owned())
    }
}
