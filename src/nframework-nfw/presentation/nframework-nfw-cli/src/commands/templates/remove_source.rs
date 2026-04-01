use crate::commands::templates::source_management_service::SourceManagementService;

#[derive(Debug, Clone)]
pub struct RemoveSourceCliCommand<S>
where
    S: SourceManagementService,
{
    source_management_service: S,
}

impl<S> RemoveSourceCliCommand<S>
where
    S: SourceManagementService,
{
    pub fn new(source_management_service: S) -> Self {
        Self {
            source_management_service,
        }
    }

    pub fn execute(&self, name: &str) -> Result<(), String> {
        self.source_management_service.remove_source(name)?;
        println!("Template source '{name}' removed.");
        Ok(())
    }
}
