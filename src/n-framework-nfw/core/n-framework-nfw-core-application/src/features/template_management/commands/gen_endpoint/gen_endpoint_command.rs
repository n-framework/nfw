use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::artifact_generation_service::AddArtifactContext;
use serde_json::Value;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl FromStr for HttpMethod {
    type Err = AddArtifactError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "PATCH" => Ok(HttpMethod::Patch),
            _ => Err(AddArtifactError::InvalidParameter(format!(
                "Invalid HTTP method '{}'. Must be GET, POST, PUT, DELETE, or PATCH.",
                s
            ))),
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "Get"),
            HttpMethod::Post => write!(f, "Post"),
            HttpMethod::Put => write!(f, "Put"),
            HttpMethod::Delete => write!(f, "Delete"),
            HttpMethod::Patch => write!(f, "Patch"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenEndpointCommand {
    name: String,
    feature: Option<String>,
    operation_type: HttpMethod,
    params: Option<Value>,
    context: AddArtifactContext,
    /// When true the handler verifies that the named Command/Query artifact already
    /// exists in the Application layer before generating the endpoint.  Set to false
    /// when the user explicitly chose NOT to attach to an existing mediator artifact
    /// (free-form endpoint name) so that the existence check is skipped.
    attach_to_mediator: bool,
}

impl GenEndpointCommand {
    pub fn new(
        name: impl Into<String>,
        feature: Option<String>,
        operation_type: HttpMethod,
        params: Option<Value>,
        context: AddArtifactContext,
        attach_to_mediator: bool,
    ) -> Result<Self, AddArtifactError> {
        let name_str = name.into();
        if name_str.trim().is_empty() {
            return Err(AddArtifactError::InvalidIdentifier(
                "Endpoint name cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            name: name_str,
            feature,
            operation_type,
            params,
            context,
            attach_to_mediator,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn feature(&self) -> Option<&str> {
        self.feature.as_deref()
    }

    pub fn operation_type(&self) -> &HttpMethod {
        &self.operation_type
    }

    pub fn params(&self) -> Option<&Value> {
        self.params.as_ref()
    }

    pub fn into_params(self) -> Option<Value> {
        self.params
    }

    pub fn context(&self) -> &AddArtifactContext {
        &self.context
    }

    pub fn attach_to_mediator(&self) -> bool {
        self.attach_to_mediator
    }
}
