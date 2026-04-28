use n_framework_core_cli_abstractions::CliCommandSpec;

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("check").with_about("Validate workspace architecture boundaries")
}
