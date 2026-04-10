use std::collections::BTreeMap;

use crate::features::check::models::architecture_layer::CheckLayer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckRuleSet {
    pub forbidden_project_references: BTreeMap<CheckLayer, Vec<CheckLayer>>,
    pub forbidden_namespace_prefixes: BTreeMap<CheckLayer, Vec<String>>,
    pub forbidden_direct_packages: BTreeMap<CheckLayer, Vec<String>>,
}
