use std::cmp::Ordering;

pub trait VersionComparator {
    fn parse(&self, version: &str) -> Result<(), String>;
    fn compare(&self, left: &str, right: &str) -> Result<Ordering, String>;
    fn is_stable(&self, version: &str) -> Result<bool, String>;
    fn satisfies(&self, version: &str, requirement: &str) -> Result<bool, String>;
}
