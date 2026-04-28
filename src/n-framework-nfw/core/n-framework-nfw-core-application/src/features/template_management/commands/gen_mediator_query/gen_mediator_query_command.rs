use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GenMediatorQueryCommand {
    pub name: String,
    pub feature: Option<String>,
    pub params: Option<Value>,
}

impl GenMediatorQueryCommand {
    pub fn new(name: impl Into<String>, feature: Option<String>, params: Option<Value>) -> Self {
        Self {
            name: name.into(),
            feature,
            params,
        }
    }
}
