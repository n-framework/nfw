use n_framework_core_cli_abstractions::CliCommandSpec;

pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("gen")
        .with_about("Generate workspace artifacts from templates")
        .require_subcommand()
        .with_subcommand(crate::commands::r#gen::command::registration::register())
        .with_subcommand(crate::commands::r#gen::query::registration::register())
}
