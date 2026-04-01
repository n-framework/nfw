use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::features::versioning::errors::VersionParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
    pub build: Option<String>,
}

impl Version {
    pub fn new(
        major: u64,
        minor: u64,
        patch: u64,
        pre: Option<String>,
        build: Option<String>,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
            pre,
            build,
        }
    }

    pub fn is_stable(&self) -> bool {
        self.pre.as_deref().is_none_or(str::is_empty)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;

        if let Some(pre) = self.pre.as_deref().filter(|value| !value.is_empty()) {
            write!(f, "-{pre}")?;
        }

        if let Some(build) = self.build.as_deref().filter(|value| !value.is_empty()) {
            write!(f, "+{build}")?;
        }

        Ok(())
    }
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.trim().is_empty() {
            return Err(VersionParseError::new("version cannot be empty"));
        }

        let (core_and_pre, build) = match value.split_once('+') {
            Some((left, right)) if !right.trim().is_empty() => {
                (left, Some(right.trim().to_owned()))
            }
            Some((_left, _right)) => return Err(VersionParseError::new("build metadata is empty")),
            None => (value, None),
        };

        let (core, pre) = match core_and_pre.split_once('-') {
            Some((left, right)) if !right.trim().is_empty() => {
                (left, Some(right.trim().to_owned()))
            }
            Some((_left, _right)) => return Err(VersionParseError::new("pre-release is empty")),
            None => (core_and_pre, None),
        };

        let mut components = core.split('.');
        let major = parse_component(components.next(), "major")?;
        let minor = parse_component(components.next(), "minor")?;
        let patch = parse_component(components.next(), "patch")?;

        if components.next().is_some() {
            return Err(VersionParseError::new("too many version components"));
        }

        Ok(Self::new(major, minor, patch, pre, build))
    }
}

fn parse_component(component: Option<&str>, label: &str) -> Result<u64, VersionParseError> {
    let value = component.ok_or_else(|| VersionParseError::new(&format!("missing {label}")))?;

    value
        .parse::<u64>()
        .map_err(|_| VersionParseError::new(&format!("invalid {label}: {value}")))
}
