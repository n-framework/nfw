use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AddArtifactCommand {
    pub generator_type: String,
    pub name: String,
    pub feature: Option<String>,
    pub params: Option<Value>,
}

impl AddArtifactCommand {
    pub fn new(
        generator_type: impl Into<String>,
        name: impl Into<String>,
        feature: Option<String>,
        params: Option<Value>,
    ) -> Self {
        Self {
            generator_type: generator_type.into(),
            name: name.into(),
            feature,
            params,
        }
    }
}
