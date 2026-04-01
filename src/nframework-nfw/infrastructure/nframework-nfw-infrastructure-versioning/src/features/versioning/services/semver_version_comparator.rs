use std::cmp::Ordering;

use nframework_nfw_application::features::versioning::abstraction::version_comparator::VersionComparator;
use semver::{Version, VersionReq};

#[derive(Debug, Default, Clone, Copy)]
pub struct SemverVersionComparator;

impl SemverVersionComparator {
    pub fn new() -> Self {
        Self
    }
}

impl VersionComparator for SemverVersionComparator {
    fn parse(&self, version: &str) -> Result<(), String> {
        Version::parse(version)
            .map(|_| ())
            .map_err(|error| format!("invalid semver '{version}': {error}"))
    }

    fn compare(&self, left: &str, right: &str) -> Result<Ordering, String> {
        let left_version =
            Version::parse(left).map_err(|error| format!("invalid semver '{left}': {error}"))?;
        let right_version =
            Version::parse(right).map_err(|error| format!("invalid semver '{right}': {error}"))?;

        Ok(left_version.cmp(&right_version))
    }

    fn is_stable(&self, version: &str) -> Result<bool, String> {
        let parsed = Version::parse(version)
            .map_err(|error| format!("invalid semver '{version}': {error}"))?;

        Ok(parsed.pre.is_empty())
    }

    fn satisfies(&self, version: &str, requirement: &str) -> Result<bool, String> {
        let parsed_version = Version::parse(version)
            .map_err(|error| format!("invalid semver '{version}': {error}"))?;
        let parsed_requirement = VersionReq::parse(requirement)
            .map_err(|error| format!("invalid version requirement '{requirement}': {error}"))?;

        Ok(parsed_requirement.matches(&parsed_version))
    }
}
