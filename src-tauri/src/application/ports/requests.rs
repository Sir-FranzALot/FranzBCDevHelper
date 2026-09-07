use crate::application::error::AppError;
use std::path::PathBuf;
use url::Url;

pub trait RequestRepository {
    async fn download(&self, src: &Url, dst: PathBuf) -> Result<(), AppError>;
}
