use std::cmp::Ordering;

use n_framework_nfw_core_domain::features::versioning::version::Version;
use n_framework_nfw_core_domain::features::versioning::version_info::VersionInfo;

use crate::features::versioning::abstractions::version_provider::VersionProvider;

#[derive(Debug, Clone)]
pub struct VersionResolver<P>
where
    P: VersionProvider,
{
    version_provider: P,
}

impl<P> VersionResolver<P>
where
    P: VersionProvider,
{
    pub fn new(version_provider: P) -> Self {
        Self { version_provider }
    }

    pub fn resolve_latest_stable<'a>(
        &self,
        versions: &'a [VersionInfo],
    ) -> Result<Option<&'a VersionInfo>, String> {
        let mut latest_stable: Option<&VersionInfo> = None;

        for candidate in versions {
            if !self.version_provider.is_stable(&candidate.version)? {
                continue;
            }

            latest_stable = match latest_stable {
                None => Some(candidate),
                Some(current_latest) => {
                    let ordering = self
                        .version_provider
                        .compare(&candidate.version, &current_latest.version)?;

                    if ordering == Ordering::Greater {
                        Some(candidate)
                    } else {
                        Some(current_latest)
                    }
                }
            };
        }

        Ok(latest_stable)
    }

    pub fn resolve_requested<'a>(
        &self,
        versions: &'a [VersionInfo],
        requested_version: &str,
    ) -> Result<Option<&'a VersionInfo>, String> {
        let requested = requested_version.trim();
        if requested.is_empty() {
            return Ok(None);
        }

        let requested_version = requested
            .parse::<Version>()
            .map_err(|error| error.to_string())?;

        for candidate in versions {
            if self
                .version_provider
                .compare(&candidate.version, &requested_version)?
                == Ordering::Equal
            {
                return Ok(Some(candidate));
            }
        }

        Ok(None)
    }

    pub fn min_cli_warning(
        &self,
        template_min_cli_version: Option<&Version>,
        current_cli_version: &Version,
    ) -> Result<Option<String>, String> {
        let Some(template_min_cli_version) = template_min_cli_version else {
            return Ok(None);
        };

        let ordering = self
            .version_provider
            .compare(current_cli_version, template_min_cli_version)?;

        if ordering == Ordering::Less {
            return Ok(Some(format!(
                "template requires CLI version {} but current CLI version is {}; consider upgrading",
                template_min_cli_version, current_cli_version
            )));
        }

        Ok(None)
    }
}
