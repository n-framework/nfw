mod cli_error;
mod commands;
mod runtime;
mod startup;

use crate::cli_error::CliError;
use crate::runtime::nfw_cli_runtime::ParseExitCode;
use crate::runtime::nfw_cli_runtime::build_nfw_cli_runtime;
use crate::startup::cli_bootstrapper::CliBootstrapper;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(error.exit_code);
    }
}

fn run() -> Result<(), CliError> {
    let input = std::env::args().skip(1).collect::<Vec<_>>();
    let services = CliBootstrapper::bootstrap()?;
    let runtime = build_nfw_cli_runtime(services);

    runtime
        .run(&input)
        .map_err(|error_string| error_string.parse_exit_code())
}
