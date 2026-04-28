use crate::features::template_management::services::artifact_generation_service::AddArtifactContext;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GenMediatorQueryCommand {
    pub name: String,
    pub feature: Option<String>,
    pub params: Option<Value>,
    pub context: AddArtifactContext,
}

impl GenMediatorQueryCommand {
    pub fn new(
        name: impl Into<String>,
        feature: Option<String>,
        params: Option<Value>,
        context: AddArtifactContext,
    ) -> Self {
        Self {
            name: name.into(),
            feature,
            params,
            context,
        }
    }
}
