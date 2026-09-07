use crate::application::error::AppError;
use std::path::{Path, PathBuf};

pub trait FilesystemRepository {
    async fn create_dir_all(path: impl AsRef<Path>) -> Result<(), AppError>;
    async fn copy_dir_recursively(
        &self,
        src: impl AsRef<Path>,
        dst: impl AsRef<Path>,
    ) -> Result<(), AppError>;
    async fn compress_dir(path: PathBuf) -> Result<Vec<u8>, AppError>;
    async fn extract_zip(src: &Path, dst: &Path) -> Result<(), AppError>;
}
