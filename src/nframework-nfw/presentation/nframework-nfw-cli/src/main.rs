mod commands;
mod runtime;
mod startup;

use crate::runtime::nfw_cli_runtime::build_nfw_cli_runtime;
use crate::startup::cli_bootstrapper::CliBootstrapper;

fn main() {
    if let Err(error) = run() {
        let (exit_code, message) = parse_exit_code_and_message(&error);
        eprintln!("error: {message}");
        std::process::exit(exit_code);
    }
}

fn run() -> Result<(), String> {
    let input = std::env::args().skip(1).collect::<Vec<_>>();
    let services = CliBootstrapper::bootstrap()?;
    let runtime = build_nfw_cli_runtime(services);

    runtime.run(&input)
}

fn parse_exit_code_and_message(error: &str) -> (i32, String) {
    let Some(rest) = error.strip_prefix("[exit:") else {
        return (1, error.to_owned());
    };

    let Some((exit_code_text, message)) = rest.split_once(']') else {
        return (1, error.to_owned());
    };

    let Ok(exit_code) = exit_code_text.parse::<i32>() else {
        return (1, error.to_owned());
    };

    (exit_code, message.trim_start().to_owned())
}
