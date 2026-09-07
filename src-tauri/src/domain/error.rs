#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("could not parse int {0}")]
    ParseInt(#[from] crate::domain::bc_version::BCVersionError),
}
