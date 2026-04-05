use std::collections::BTreeMap;

use crate::features::architecture_validation::models::{
    ArchitectureLayer, ArchitectureRuleSet,
};
use crate::features::architecture_validation::services::abstractions::RuleSetLoader;

#[derive(Debug, Default, Clone, Copy)]
pub struct RuleSetLoaderService;

impl RuleSetLoaderService {
    pub fn new() -> Self {
        Self
    }
}

impl RuleSetLoader for RuleSetLoaderService {
    fn load(&self) -> ArchitectureRuleSet {
        let forbidden_project_references = BTreeMap::from([
            (
                ArchitectureLayer::Domain,
                vec![
                    ArchitectureLayer::Application,
                    ArchitectureLayer::Infrastructure,
                    ArchitectureLayer::Presentation,
                ],
            ),
            (
                ArchitectureLayer::Application,
                vec![ArchitectureLayer::Infrastructure, ArchitectureLayer::Presentation],
            ),
            (
                ArchitectureLayer::Infrastructure,
                vec![ArchitectureLayer::Presentation],
            ),
        ]);

        let forbidden_namespace_prefixes = BTreeMap::from([
            (
                ArchitectureLayer::Domain,
                vec![
                    "NFramework.Infrastructure".to_owned(),
                    "NFramework.Presentation".to_owned(),
                    "nframework::infrastructure".to_owned(),
                    "nframework::presentation".to_owned(),
                    "nframework.infrastructure".to_owned(),
                    "nframework.presentation".to_owned(),
                    "Microsoft.AspNetCore".to_owned(),
                ],
            ),
            (
                ArchitectureLayer::Application,
                vec![
                    "NFramework.Presentation".to_owned(),
                    "nframework::presentation".to_owned(),
                    "nframework.presentation".to_owned(),
                    "Microsoft.AspNetCore.Mvc".to_owned(),
                ],
            ),
            (
                ArchitectureLayer::Infrastructure,
                vec![
                    "NFramework.Presentation".to_owned(),
                    "nframework::presentation".to_owned(),
                    "nframework.presentation".to_owned(),
                ],
            ),
        ]);

        let forbidden_direct_packages = BTreeMap::from([
            (
                ArchitectureLayer::Domain,
                vec![
                    "Microsoft.AspNetCore.App".to_owned(),
                    "Serilog.AspNetCore".to_owned(),
                    "axum".to_owned(),
                    "express".to_owned(),
                ],
            ),
            (
                ArchitectureLayer::Application,
                vec![
                    "Microsoft.AspNetCore.App".to_owned(),
                    "express".to_owned(),
                ],
            ),
        ]);

        ArchitectureRuleSet {
            forbidden_project_references,
            forbidden_namespace_prefixes,
            forbidden_direct_packages,
        }
    }
}
