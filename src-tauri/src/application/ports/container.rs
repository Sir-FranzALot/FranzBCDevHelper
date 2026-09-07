use crate::application::error::AppError;
use crate::domain::{Container, Image};

pub trait ContainerRepository {
    async fn start(&self, container: &Container) -> Result<(), AppError>;
    async fn stop(&self, container: &Container) -> Result<(), AppError>;
    async fn remove(&self, container: &Container) -> Result<(), AppError>;
    async fn create(&self, image: &Image, name: &str) -> Result<(), AppError>;
    async fn list_containers(&self) -> Result<Vec<Container>, AppError>;
    async fn inspect(&self, name: &str) -> Result<Container, AppError>;
}
