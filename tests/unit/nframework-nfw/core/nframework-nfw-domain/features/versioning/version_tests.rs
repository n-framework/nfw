use std::str::FromStr;

use nframework_nfw_core_domain::features::versioning::version::Version;

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

#[test]
fn rejects_empty_version() {
    let result = Version::from_str("");
    assert!(result.is_err(), "empty version should be rejected");
}

#[test]
fn rejects_malformed_version() {
    let result = Version::from_str("not.a.version");
    assert!(result.is_err(), "malformed version should be rejected");
}

#[test]
fn rejects_missing_components() {
    assert!(
        Version::from_str("1.2").is_err(),
        "missing patch should be rejected"
    );
    assert!(
        Version::from_str("1").is_err(),
        "missing minor and patch should be rejected"
    );
}

#[test]
fn rejects_too_many_components() {
    let result = Version::from_str("1.2.3.4.5");
    assert!(result.is_err(), "too many components should be rejected");
}

#[test]
fn handles_zero_components() {
    let version = Version::from_str("0.0.0").expect("zero version should parse");
    assert_eq!(version.major, 0);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
}

#[test]
fn handles_large_version_numbers() {
    let version = Version::from_str("999.999.999").expect("large version should parse");
    assert_eq!(version.major, 999);
    assert_eq!(version.minor, 999);
    assert_eq!(version.patch, 999);
}

#[test]
fn rejects_empty_pre_release() {
    let result = Version::from_str("1.2.3-");
    assert!(result.is_err(), "empty pre-release should be rejected");
}

#[test]
fn rejects_empty_build() {
    let result = Version::from_str("1.2.3+");
    assert!(result.is_err(), "empty build should be rejected");
}

#[test]
fn builder_creates_version() {
    let version = Version::builder()
        .major(1)
        .minor(2)
        .patch(3)
        .pre("alpha".to_owned())
        .build("test".to_owned())
        .build_version();

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(version.pre.as_deref(), Some("alpha"));
    assert_eq!(version.build.as_deref(), Some("test"));
}
