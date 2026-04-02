mod commands;
mod runtime;
mod startup;

use crate::runtime::nfw_cli_runtime::build_nfw_cli_runtime;
use crate::startup::cli_bootstrapper::CliBootstrapper;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let input = std::env::args().skip(1).collect::<Vec<_>>();
    let services = CliBootstrapper::bootstrap()?;
    let runtime = build_nfw_cli_runtime(services);

    runtime.run(&input)
}
