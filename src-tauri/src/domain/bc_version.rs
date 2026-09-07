use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct BcVersion {
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub revision: u32,
}

impl FromStr for BcVersion {
    type Err = BCVersionError;

    fn from_str(version: &str) -> Result<Self, BCVersionError> {
        if version.is_empty() {
            return Err(BCVersionError::ParseVersion(version.to_string()));
        }
        let mut parts = version.split('.');

        // TODO might be a bit nitpicky but I would like to only need
        // parse on v and 0 should be an int and not need to be converted.
        // Also I do not actuall need map_or on major
        {
            let major = parts.next().map_or("0", |v| v).parse::<u32>()?;
            let minor = parts.next().map_or("0", |v| v).parse::<u32>()?;
            let build = parts.next().map_or("0", |v| v).parse::<u32>()?;
            let revision = parts.next().map_or("0", |v| v).parse::<u32>()?;

            if parts.next().is_some() {
                return Err(BCVersionError::ParseVersion(version.to_string()));
            }

            Ok(Self {
                major,
                minor,
                build,
                revision,
            })
        }
    }
}

impl fmt::Display for BcVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.revision
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BCVersionError {
    #[error("failed to parse version segment: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("failed to parse version: {0}")]
    ParseVersion(String),
}
