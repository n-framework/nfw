use crate::features::architecture_validation::models::ArchitectureRuleSet;

pub trait RuleSetLoader {
    fn load(&self) -> ArchitectureRuleSet;
}
