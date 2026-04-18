#[derive(Debug, Clone)]
pub struct GenerateCommand {
    pub generator_type: String,
    pub name: String,
    pub feature: Option<String>,
    pub params: Option<String>,
}

impl GenerateCommand {
    pub fn new(
        generator_type: impl Into<String>,
        name: impl Into<String>,
        feature: Option<String>,
        params: Option<String>,
    ) -> Self {
        Self {
            generator_type: generator_type.into(),
            name: name.into(),
            feature,
            params,
        }
    }
}
