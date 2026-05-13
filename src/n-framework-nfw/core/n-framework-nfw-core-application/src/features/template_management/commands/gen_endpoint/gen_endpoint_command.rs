use crate::features::template_management::services::artifact_generation_service::AddArtifactContext;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GenEndpointCommand {
    pub name: String,
    pub feature: Option<String>,
    pub operation_type: String,
    pub params: Option<Value>,
    pub context: AddArtifactContext,
    /// When true the handler verifies that the named Command/Query artifact already
    /// exists in the Application layer before generating the endpoint.  Set to false
    /// when the user explicitly chose NOT to attach to an existing mediator artifact
    /// (free-form endpoint name) so that the existence check is skipped.
    pub attach_to_mediator: bool,
}

impl GenEndpointCommand {
    pub fn new(
        name: impl Into<String>,
        feature: Option<String>,
        operation_type: impl Into<String>,
        params: Option<Value>,
        context: AddArtifactContext,
        attach_to_mediator: bool,
    ) -> Self {
        Self {
            name: name.into(),
            feature,
            operation_type: operation_type.into(),
            params,
            context,
            attach_to_mediator,
        }
    }
}
