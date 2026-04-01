use std::str::FromStr;

use nframework_nfw_domain::features::versioning::version::Version;

#[test]
fn parses_basic_version() {
    let version = Version::from_str("1.2.3").expect("version should parse");

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert!(version.pre.is_none());
    assert!(version.build.is_none());
    assert!(version.is_stable());
}

#[test]
fn parses_pre_release_and_build() {
    let version = Version::from_str("1.2.3-alpha.1+build.42").expect("version should parse");

    assert_eq!(version.pre.as_deref(), Some("alpha.1"));
    assert_eq!(version.build.as_deref(), Some("build.42"));
    assert!(!version.is_stable());
    assert_eq!(version.to_string(), "1.2.3-alpha.1+build.42");
}
