use std::cmp::Ordering;

use n_framework_nfw_core_domain::features::versioning::version::Version;

pub trait VersionProvider {
    fn compare(&self, left: &Version, right: &Version) -> Result<Ordering, String>;
    fn is_stable(&self, version: &Version) -> Result<bool, String>;
    fn satisfies(&self, version: &Version, requirement: &str) -> Result<bool, String>;
}
