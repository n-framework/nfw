use crate::features::template_management::services::artifact_generation_service::AddArtifactContext;

#[derive(Debug, Clone)]
pub struct GenRepositoryCommand {
    entity_name: String,
    feature: Option<String>,
    context: AddArtifactContext,
}

impl GenRepositoryCommand {
    pub fn new(entity_name: String, feature: Option<String>, context: AddArtifactContext) -> Self {
        assert!(!entity_name.is_empty(), "entity_name cannot be empty");
        if let Some(ref f) = feature {
            assert!(!f.is_empty(), "feature name cannot be empty if provided");
        }

        Self {
            entity_name,
            feature,
            context,
        }
    }

    pub fn entity_name(&self) -> &str {
        &self.entity_name
    }

    pub fn feature(&self) -> Option<&str> {
        self.feature.as_deref()
    }

    pub fn context(&self) -> &AddArtifactContext {
        &self.context
    }
}
