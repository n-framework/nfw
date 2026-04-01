use core_cli_rust::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliArgs {
    pub command: CliCommand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    Templates(TemplatesCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplatesCommand {
    List,
    Add { name: String, url: String },
    Remove { name: String },
    Refresh,
}

impl CliArgs {
    pub fn from_command(command: &dyn Command) -> Result<Self, String> {
        match command.name() {
            "templates/list" => Ok(Self {
                command: CliCommand::Templates(TemplatesCommand::List),
            }),
            "templates/add" => {
                let name = command
                    .option("name")
                    .ok_or_else(|| "missing required option '--name'".to_owned())?
                    .to_owned();
                let url = command
                    .option("url")
                    .ok_or_else(|| "missing required option '--url'".to_owned())?
                    .to_owned();

                Ok(Self {
                    command: CliCommand::Templates(TemplatesCommand::Add { name, url }),
                })
            }
            "templates/remove" => {
                let name = command
                    .option("name")
                    .ok_or_else(|| "missing required option '--name'".to_owned())?
                    .to_owned();

                Ok(Self {
                    command: CliCommand::Templates(TemplatesCommand::Remove { name }),
                })
            }
            "templates/refresh" => Ok(Self {
                command: CliCommand::Templates(TemplatesCommand::Refresh),
            }),
            unsupported => Err(format!("unsupported command: {unsupported}")),
        }
    }
}
