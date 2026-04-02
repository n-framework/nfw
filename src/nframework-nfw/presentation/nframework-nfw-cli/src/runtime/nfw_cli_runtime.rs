use nframework_core_cli_abstraction::{
    CliAppConfig, CliCommandSpec, CliOptionSpec, CliRuntime, CliSpec, Command,
};
use nframework_core_cli_clap::ClapCliRuntimeBuilder;

use crate::commands::templates::add_source::AddSourceCliCommand;
use crate::commands::templates::list_templates::TemplatesCliCommand;
use crate::commands::templates::refresh::RefreshTemplatesCliCommand;
use crate::commands::templates::remove_source::RemoveSourceCliCommand;
use crate::startup::cli_service_collection_factory::CliServiceCollection;

const NFRAMEWORK_ASCII_BANNER: &str = r#"
   _  ______                                   __
  / |/ / __/______ ___ _  ___ _    _____  ____/ /__
 /    / _// __/ _ `/  ' \/ -_) |/|/ / _ \/ __/  '_/
/_/|_/_/ /_/  \_,_/_/_/_/\__/|__,__/\___/_/ /_/\_\ "#;

pub fn build_nfw_cli_app_config() -> CliAppConfig {
    CliAppConfig::new(
        CliSpec::new("nfw")
            .with_banner(NFRAMEWORK_ASCII_BANNER)
            .with_about("NFramework CLI")
            .require_command()
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
            ),
    )
}

pub fn build_nfw_cli_runtime(services: CliServiceCollection) -> CliRuntime<CliServiceCollection> {
    ClapCliRuntimeBuilder::new(build_nfw_cli_app_config(), services)
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

fn handle_templates_list(_: &dyn Command, context: &CliServiceCollection) -> Result<(), String> {
    TemplatesCliCommand::new(context.list_templates_query_handler.clone()).execute()
}

fn handle_templates_add(
    command: &dyn Command,
    context: &CliServiceCollection,
) -> Result<(), String> {
    let name = required_option(command, "name")?;
    let url = required_option(command, "url")?;
    AddSourceCliCommand::new(context.templates_service.clone()).execute(&name, &url)
}

fn handle_templates_remove(
    command: &dyn Command,
    context: &CliServiceCollection,
) -> Result<(), String> {
    let name = required_option(command, "name")?;
    RemoveSourceCliCommand::new(context.templates_service.clone()).execute(&name)
}

fn handle_templates_refresh(_: &dyn Command, context: &CliServiceCollection) -> Result<(), String> {
    RefreshTemplatesCliCommand::new(context.templates_service.clone()).execute()
}
