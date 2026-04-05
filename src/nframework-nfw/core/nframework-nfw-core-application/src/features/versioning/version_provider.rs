use std::cmp::Ordering;

use nframework_nfw_core_domain::features::versioning::version::Version;

use crate::features::versioning::abstractions::version_comparator::VersionComparator;
use crate::features::versioning::abstractions::version_provider::VersionProvider as VersionProviderTrait;

#[derive(Debug, Clone)]
pub struct VersionProvider<C>
where
    C: VersionComparator,
{
    comparator: C,
}

impl<C> VersionProvider<C>
where
    C: VersionComparator,
{
    pub fn new(comparator: C) -> Self {
        Self { comparator }
    }
}

impl<C> VersionProviderTrait for VersionProvider<C>
where
    C: VersionComparator,
{
    fn compare(&self, left: &Version, right: &Version) -> Result<Ordering, String> {
        self.comparator
            .compare(&left.to_string(), &right.to_string())
    }

    fn is_stable(&self, version: &Version) -> Result<bool, String> {
        self.comparator.is_stable(&version.to_string())
    }

    fn satisfies(&self, version: &Version, requirement: &str) -> Result<bool, String> {
        self.comparator.satisfies(&version.to_string(), requirement)
    }
}
