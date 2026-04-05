use crate::features::check::models::CheckRuleSet;

pub trait RuleSetLoader {
    fn load(&self) -> CheckRuleSet;
}
