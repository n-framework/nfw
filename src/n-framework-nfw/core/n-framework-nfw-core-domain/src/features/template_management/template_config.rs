use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemplateConfig {
    pub id: Option<String>,
    #[serde(default)]
    pub steps: Vec<TemplateStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum TemplateStep {
    Render {
        source: String,
        destination: String,
    },
    RenderFolder {
        source: String,
        destination: String,
    },
    Inject {
        source: String,
        destination: String,
        injection_target: InjectionTarget,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InjectionTarget {
    AtEnd,
    Class(String),
    Function(String),
    Region(String),
}
