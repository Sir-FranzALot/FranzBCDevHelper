use crate::application::error::AppError;
use crate::domain::Image;

pub trait ImageRepository {
    async fn remove(&self, image: &Image) -> Result<(), AppError>;
    async fn inspect(&self, name: &str) -> Result<Image, AppError>;
    async fn build(&self, tar_data: Vec<u8>, image_name: &str) -> Result<(), AppError>;
}
