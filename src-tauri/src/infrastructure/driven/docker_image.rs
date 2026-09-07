use crate::{
    application::{error::AppError, ports::image::ImageRepository},
    domain::Image,
};
use bollard::query_parameters::BuildImageOptionsBuilder;
use bollard::{body_full, Docker};
use bytes::Bytes;
use futures_util::{StreamExt, TryFutureExt};

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
        let image = self
            .docker
            .inspect_image(name)
            .await
            .map_err(|err| AppError::Repository(err.to_string()))?;

        let id = match image.id {
            Some(v) => v,
            None => "NA".to_string(),
        };

        Ok(Image::new(id, name.to_string()))
    }

    async fn build(&self, tar_data: Vec<u8>, image_name: &str) -> Result<(), AppError> {
        let options = BuildImageOptionsBuilder::default() // TODO add memory parameter when fixed by bollard
            .dockerfile("dockerfile")
            .t(image_name)
            .rm(true)
            .build();

        let mut stream =
            self.docker
                .build_image(options, None, Some(body_full(Bytes::from(tar_data))));

        while let Some(result) = stream.next().await {
            match result {
                Ok(_) => (),
                Err(err) => return Err(AppError::Repository(err.to_string())),
            }
        }

        Ok(())
    }
}
