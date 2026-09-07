use crate::domain::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("email already subscribed")]
    AlreadySubscribed,
    #[error("repository failure: {0}")]
    Repository(String),
}
