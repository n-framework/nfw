use n_framework_core_cli_abstractions::CliCommandSpec;

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("refresh").with_about("Refresh template catalogs from sources")
}
