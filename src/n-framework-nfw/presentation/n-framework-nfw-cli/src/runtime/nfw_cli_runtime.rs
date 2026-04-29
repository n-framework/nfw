use n_framework_core_cli_abstractions::{CliAppConfig, CliRuntime, CliSpec};
use n_framework_core_cli_clap::ClapCliRuntimeBuilder;

use crate::cli_error::CliError;
use crate::commands::add::mediator::AddMediatorCliCommand;
use crate::commands::add::persistence::AddPersistenceCliCommand;
use crate::commands::add::service::AddServiceCliCommand;
use crate::commands::check::RunCheckCliCommand;
use crate::commands::r#gen::command::GenMediatorCommandCliCommand;
use crate::commands::r#gen::query::GenMediatorQueryCliCommand;
use crate::commands::new::NewWorkspaceCliCommand;
use crate::commands::templates::add::AddSourceCliCommand;
use crate::commands::templates::list::TemplatesCliCommand;
use crate::commands::templates::refresh::RefreshTemplatesCliCommand;
use crate::commands::templates::remove::RemoveSourceCliCommand;
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
            .with_command(crate::commands::new::registration::register())
            .with_command(crate::commands::templates::registration::register())
            .with_command(crate::commands::check::registration::register())
            .with_command(crate::commands::add::registration::register())
            .with_command(crate::commands::r#gen::registration::register()),
    )
}

pub fn build_nfw_cli_runtime(services: CliServiceCollection) -> CliRuntime<CliServiceCollection> {
    println!("{NFRAMEWORK_ASCII_BANNER}");
    println!();

    ClapCliRuntimeBuilder::new(build_nfw_cli_app_config(), services)
        .register_handler("new", NewWorkspaceCliCommand::handle)
        .register_handler("check", RunCheckCliCommand::handle)
        .register_handler("add/service", AddServiceCliCommand::handle)
        .register_handler("add/mediator", AddMediatorCliCommand::handle)
        .register_handler("add/persistence", AddPersistenceCliCommand::handle)
        .register_handler("gen/command", GenMediatorCommandCliCommand::handle)
        .register_handler("gen/query", GenMediatorQueryCliCommand::handle)
        .register_handler("templates/list", TemplatesCliCommand::handle)
        .register_handler("templates/add", AddSourceCliCommand::handle)
        .register_handler("templates/remove", RemoveSourceCliCommand::handle)
        .register_handler("templates/refresh", RefreshTemplatesCliCommand::handle)
        .build()
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

        if let Some((code, "silent")) = exit_code_text.split_once(':')
            && let Ok(exit_code) = code.parse::<i32>()
        {
            return CliError::silent(exit_code, message.trim_start().to_owned());
        }

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

        if let Some((code, "silent")) = exit_code_text.split_once(':')
            && let Ok(exit_code) = code.parse::<i32>()
        {
            return CliError::silent(exit_code, message.trim_start().to_owned());
        }

        let Ok(exit_code) = exit_code_text.parse::<i32>() else {
            return CliError::internal(self.to_owned());
        };

        CliError::new(exit_code, message.trim_start().to_owned())
    }
}

#[cfg(test)]
#[path = "nfw_cli_runtime.tests.rs"]
mod tests;
