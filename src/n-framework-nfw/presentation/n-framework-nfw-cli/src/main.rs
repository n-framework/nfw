use n_framework_nfw_cli::cli_error::CliError;
use n_framework_nfw_cli::runtime::nfw_cli_runtime::ParseExitCode;
use n_framework_nfw_cli::runtime::nfw_cli_runtime::build_nfw_cli_runtime;
use n_framework_nfw_cli::startup::cli_bootstrapper::CliBootstrapper;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

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
