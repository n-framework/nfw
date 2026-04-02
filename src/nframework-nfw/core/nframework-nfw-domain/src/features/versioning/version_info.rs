use crate::features::versioning::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionInfo {
    pub version: Version,
    pub stable: bool,
}

impl VersionInfo {
    pub fn new(version: Version) -> Self {
        let stable = version.is_stable();

        Self { version, stable }
    }
}
