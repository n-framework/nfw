use nframework_core_cli_abstraction::{
    CliAdapter, CliAdapterError, CliCommandSpec, CliOptionSpec, CliSpec, Command,
};
use nframework_core_cli_clap::ClapAdapter;

const NFRAMEWORK_ASCII_BANNER: &str = r#"
   _  ______                                   __
  / |/ / __/______ ___ _  ___ _    _____  ____/ /__
 /    / _// __/ _ `/  ' \/ -_) |/|/ / _ \/ __/  '_/
/_/|_/_/ /_/  \_,_/_/_/_/\__/|__,__/\___/_/ /_/\_\ "#;

#[derive(Debug, Clone)]
pub struct NfwCliAdapter {
    adapter: ClapAdapter,
}

impl NfwCliAdapter {
    pub fn new() -> Self {
        let spec = CliSpec::new("nfw")
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
            );

        Self {
            adapter: ClapAdapter::from_spec(&spec),
        }
    }
}

impl CliAdapter for NfwCliAdapter {
    fn parse(&self, input: &[String]) -> Result<Box<dyn Command>, CliAdapterError> {
        self.adapter.parse(input)
    }
}
