use std::collections::{BTreeMap, BTreeSet};

const DOMAIN_LAYER: &str = "Domain";
const APPLICATION_LAYER: &str = "Application";
const INFRASTRUCTURE_LAYER: &str = "Infrastructure";
const API_LAYER: &str = "Api";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerDependencyMatrix {
    allowed: BTreeMap<String, BTreeSet<String>>,
}

impl LayerDependencyMatrix {
    pub fn standard() -> Self {
        let mut allowed = BTreeMap::<String, BTreeSet<String>>::new();
        allowed.insert(DOMAIN_LAYER.to_owned(), BTreeSet::new());
        allowed.insert(
            APPLICATION_LAYER.to_owned(),
            BTreeSet::from([DOMAIN_LAYER.to_owned()]),
        );
        allowed.insert(
            INFRASTRUCTURE_LAYER.to_owned(),
            BTreeSet::from([DOMAIN_LAYER.to_owned(), APPLICATION_LAYER.to_owned()]),
        );
        allowed.insert(
            API_LAYER.to_owned(),
            BTreeSet::from([
                APPLICATION_LAYER.to_owned(),
                INFRASTRUCTURE_LAYER.to_owned(),
            ]),
        );

        Self { allowed }
    }

    pub fn layer_names(&self) -> Vec<String> {
        self.allowed.keys().cloned().collect()
    }

    pub fn is_allowed_reference(&self, source_layer: &str, target_layer: &str) -> bool {
        self.allowed
            .get(source_layer)
            .is_some_and(|allowed_targets| allowed_targets.contains(target_layer))
    }

    pub fn validate_edges(&self, edges: &[(String, String)]) -> Vec<String> {
        edges
            .iter()
            .filter_map(|(source_layer, target_layer)| {
                if self.is_allowed_reference(source_layer, target_layer) {
                    return None;
                }

                Some(format!(
                    "forbidden dependency: {source_layer} -> {target_layer}"
                ))
            })
            .collect()
    }
}
