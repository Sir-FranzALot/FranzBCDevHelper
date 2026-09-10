use crate::domain::BcVersion;

pub struct ArtifactRequest {
    pub deployment_type: String,
    pub version: BcVersion,
    pub country: String,
}
