use crate::application::error::AppError;
pub trait FilesystemRepository {
    async fn create_dir_all() -> Result<(), AppError>;
}
