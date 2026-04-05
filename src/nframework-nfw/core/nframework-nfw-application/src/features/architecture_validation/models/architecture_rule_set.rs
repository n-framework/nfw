use std::collections::BTreeMap;

use crate::features::architecture_validation::models::architecture_layer::ArchitectureLayer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchitectureRuleSet {
    pub forbidden_project_references: BTreeMap<ArchitectureLayer, Vec<ArchitectureLayer>>,
    pub forbidden_namespace_prefixes: BTreeMap<ArchitectureLayer, Vec<String>>,
    pub forbidden_direct_packages: BTreeMap<ArchitectureLayer, Vec<String>>,
}
