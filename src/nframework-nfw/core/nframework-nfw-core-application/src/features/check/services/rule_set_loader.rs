use std::collections::BTreeMap;

use crate::features::check::models::{CheckLayer, CheckRuleSet};
use crate::features::check::services::abstractions::RuleSetLoader;

#[derive(Debug, Default, Clone, Copy)]
pub struct RuleSetLoaderService;

impl RuleSetLoaderService {
    pub fn new() -> Self {
        Self
    }
}

impl RuleSetLoader for RuleSetLoaderService {
    fn load(&self) -> CheckRuleSet {
        let forbidden_project_references = BTreeMap::from([
            (
                CheckLayer::Domain,
                vec![
                    CheckLayer::Application,
                    CheckLayer::Infrastructure,
                    CheckLayer::Presentation,
                ],
            ),
            (
                CheckLayer::Application,
                vec![CheckLayer::Infrastructure, CheckLayer::Presentation],
            ),
            (CheckLayer::Infrastructure, vec![CheckLayer::Presentation]),
        ]);

        let forbidden_namespace_prefixes = BTreeMap::from([
            (
                CheckLayer::Domain,
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
                CheckLayer::Application,
                vec![
                    "NFramework.Presentation".to_owned(),
                    "nframework::presentation".to_owned(),
                    "nframework.presentation".to_owned(),
                    "Microsoft.AspNetCore.Mvc".to_owned(),
                ],
            ),
            (
                CheckLayer::Infrastructure,
                vec![
                    "NFramework.Presentation".to_owned(),
                    "nframework::presentation".to_owned(),
                    "nframework.presentation".to_owned(),
                ],
            ),
        ]);

        let forbidden_direct_packages = BTreeMap::from([
            (
                CheckLayer::Domain,
                vec![
                    "Microsoft.AspNetCore.App".to_owned(),
                    "Serilog.AspNetCore".to_owned(),
                    "axum".to_owned(),
                    "express".to_owned(),
                ],
            ),
            (
                CheckLayer::Application,
                vec!["Microsoft.AspNetCore.App".to_owned(), "express".to_owned()],
            ),
        ]);

        CheckRuleSet {
            forbidden_project_references,
            forbidden_namespace_prefixes,
            forbidden_direct_packages,
        }
    }
}
