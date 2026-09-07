use crate::application::error::AppError;
use crate::domain::{Container, Image};

pub trait DockerRepository {
    async fn start_container(&self, container: &Container) -> Result<(), AppError>;
    async fn stop_container(&self, container: &Container) -> Result<(), AppError>;
    async fn remove_container(&self, container: &Container) -> Result<(), AppError>;
    async fn create_container(&self, image: &Image, name: &str) -> Result<(), AppError>;
    async fn list_containers(&self) -> Result<Vec<Container>, AppError>;
    async fn inspect_container(&self, name: &str) -> Result<Container, AppError>;

    async fn remove_image(&self, image: &Image) -> Result<(), AppError>;
    async fn inspect_image(&self, name: &str) -> Result<Image, AppError>;
    async fn build_image(&self, tar_data: Vec<u8>, image_name: &str) -> Result<(), AppError>;
}
