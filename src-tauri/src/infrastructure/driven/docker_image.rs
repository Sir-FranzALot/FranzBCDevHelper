use crate::{
    application::{error::AppError, ports::image::ImageRepository},
    domain::Image,
};
use bollard::Docker;

struct DockerImageRepository {
    docker: Docker,
}

impl DockerImageRepository {
    fn new(docker: Docker) -> DockerImageRepository {
        DockerImageRepository { docker }
    }
}

impl ImageRepository for DockerImageRepository {
    async fn remove(&self, image: &Image) -> Result<(), AppError> {
        todo!()
    }

    async fn inspect(&self, name: &str) -> Result<Image, AppError> {
        todo!()
    }

    async fn build(&self, tar_data: Vec<u8>, image_name: &str) -> Result<Image, AppError> {
        todo!()
    }
}
