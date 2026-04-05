use std::cmp::Ordering;

use nframework_nfw_core_application::features::versioning::abstractions::version_comparator::VersionComparator;
use nframework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;

#[test]
fn compares_versions() {
    let comparator = SemverVersionComparator::new();
    let ordering = comparator
        .compare("1.0.0", "1.1.0")
        .expect("versions should be comparable");

    assert_eq!(ordering, Ordering::Less);
}

#[test]
fn validates_requirement() {
    let comparator = SemverVersionComparator::new();
    assert!(
        comparator
            .satisfies("1.2.3", ">=1.0.0, <2.0.0")
            .expect("constraint check should succeed")
    );
}
