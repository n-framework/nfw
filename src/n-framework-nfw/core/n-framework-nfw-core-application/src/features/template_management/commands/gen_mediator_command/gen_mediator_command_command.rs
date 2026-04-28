use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GenMediatorCommandCommand {
    pub name: String,
    pub feature: Option<String>,
    pub params: Option<Value>,
}

impl GenMediatorCommandCommand {
    pub fn new(name: impl Into<String>, feature: Option<String>, params: Option<Value>) -> Self {
        Self {
            name: name.into(),
            feature,
            params,
        }
    }
}
