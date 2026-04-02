use crate::commands::templates::source_management_service::SourceManagementService;

#[derive(Debug, Clone)]
pub struct AddSourceCliCommand<S>
where
    S: SourceManagementService,
{
    source_management_service: S,
}

impl<S> AddSourceCliCommand<S>
where
    S: SourceManagementService,
{
    pub fn new(source_management_service: S) -> Self {
        Self {
            source_management_service,
        }
    }

    pub fn execute(&self, name: &str, url: &str) -> Result<(), String> {
        self.source_management_service.add_source(name, url)?;
        println!("Template source '{name}' added.");
        Ok(())
    }
}
