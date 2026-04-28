#[derive(Debug, Clone)]
pub struct AddMediatorCommand {
    pub service_name: String,
}

impl AddMediatorCommand {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }
}
