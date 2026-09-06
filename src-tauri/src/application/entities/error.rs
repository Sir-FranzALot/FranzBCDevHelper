use se

#[derive(Debug, thiserror::Error)]
pub enum EntityError {
    #[error("could not parse int {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}
