use crate::application::error::AppError;
use async_trait::async_trait;
use std::path::{Path, PathBuf};

#[async_trait]
pub trait FilesystemRepository: Send + Sync {
    async fn create_dir_all(&self, path: &Path) -> Result<(), AppError>;
    async fn copy_dir_recursively(&self, src: &Path, dst: &Path) -> Result<(), AppError>;
    async fn compress_dir(&self, path: PathBuf) -> Result<Vec<u8>, AppError>;
    async fn extract_zip(&self, src: &Path, dst: &Path) -> Result<(), AppError>;
}
