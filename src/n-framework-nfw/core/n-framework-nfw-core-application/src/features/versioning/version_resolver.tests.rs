use super::*;
use crate::features::versioning::abstractions::version_comparator::VersionComparator;
use crate::features::versioning::version_provider::VersionProvider;
use n_framework_nfw_core_domain::features::versioning::version::Version;
use n_framework_nfw_core_domain::features::versioning::version_info::VersionInfo;
use std::cmp::Ordering;
use std::str::FromStr;

struct MockComparator;

impl VersionComparator for MockComparator {
    fn parse(&self, _version: &str) -> Result<(), String> {
        Ok(())
    }
    fn compare(&self, left: &str, right: &str) -> Result<Ordering, String> {
        let l = semver::Version::parse(left).map_err(|e| e.to_string())?;
        let r = semver::Version::parse(right).map_err(|e| e.to_string())?;
        Ok(l.cmp(&r))
    }
    fn is_stable(&self, version: &str) -> Result<bool, String> {
        let v = semver::Version::parse(version).map_err(|e| e.to_string())?;
        Ok(v.pre.is_empty())
    }
    fn satisfies(&self, version: &str, requirement: &str) -> Result<bool, String> {
        let v = semver::Version::parse(version).map_err(|e| e.to_string())?;
        let req = semver::VersionReq::parse(requirement).map_err(|e| e.to_string())?;
        Ok(req.matches(&v))
    }
}

#[test]
fn resolves_latest_stable_version() {
    let resolver = create_resolver();
    let versions = vec![
        version_info("1.0.0-alpha.1"),
        version_info("1.0.0"),
        version_info("1.2.0-beta.1"),
        version_info("1.1.5"),
    ];

    let resolved = resolver
        .resolve_latest_stable(&versions)
        .expect("version resolution should succeed")
        .expect("a stable version should be found");

    assert_eq!(resolved.version.to_string(), "1.1.5");
}

#[test]
fn excludes_pre_release_when_resolving_latest_stable() {
    let resolver = create_resolver();
    let versions = vec![version_info("1.0.0-alpha.1"), version_info("2.0.0-beta.2")];

    let resolved = resolver
        .resolve_latest_stable(&versions)
        .expect("version resolution should succeed");

    assert!(resolved.is_none());
}

#[test]
fn resolves_requested_version_including_pre_release() {
    let resolver = create_resolver();
    let versions = vec![version_info("1.0.0"), version_info("2.0.0-beta.2")];

    let resolved = resolver
        .resolve_requested(&versions, "2.0.0-beta.2")
        .expect("version resolution should succeed")
        .expect("requested version should be found");

    assert_eq!(resolved.version.to_string(), "2.0.0-beta.2");
}

#[test]
fn warns_when_current_cli_is_older_than_template_minimum() {
    let resolver = create_resolver();
    let template_min_cli = Version::from_str("2.0.0").expect("version should parse");
    let current_cli = Version::from_str("1.5.0").expect("version should parse");

    let warning = resolver
        .min_cli_warning(Some(&template_min_cli), &current_cli)
        .expect("compatibility check should succeed");

    assert!(warning.is_some());
    assert!(
        warning
            .expect("warning should exist")
            .contains("template requires CLI version 2.0.0")
    );
}

fn create_resolver() -> VersionResolver<VersionProvider<MockComparator>> {
    VersionResolver::new(VersionProvider::new(MockComparator))
}

fn version_info(value: &str) -> VersionInfo {
    VersionInfo::new(Version::from_str(value).expect("version should parse"))
}
