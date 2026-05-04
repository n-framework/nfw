use crate::features::template_management::services::artifact_generation_service::AddArtifactContext;

#[derive(Debug, Clone)]
pub struct GenRepositoryCommand {
    pub entity_name: String,
    pub feature: Option<String>,
    pub context: AddArtifactContext,
}

impl GenRepositoryCommand {
    pub fn new(entity_name: String, feature: Option<String>, context: AddArtifactContext) -> Self {
        Self {
            entity_name,
            feature,
            context,
        }
    }
}
