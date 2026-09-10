use crate::application::error::AppError;
use async_trait::async_trait;
use std::path::PathBuf;
use url::Url;

#[async_trait]
pub trait RequestRepository: Send + Sync {
    async fn download(&self, src: &Url, dst: PathBuf) -> Result<(), AppError>;
}
