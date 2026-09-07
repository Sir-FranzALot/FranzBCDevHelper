use crate::{
    application::{error::AppError, ports::docker::DockerRepository},
    domain::{Container, Image},
};
use bollard::config::ContainerCreateBody;
use bollard::query_parameters::{
    BuildImageOptionsBuilder, CreateContainerOptionsBuilder, ListImagesOptionsBuilder,
};
use bollard::{body_full, Docker};
use bytes::Bytes;
use futures_util::StreamExt;

struct BollardDockerRepository {
    docker: Docker,
}

impl BollardDockerRepository {
    fn new(docker: Docker) -> BollardDockerRepository {
        BollardDockerRepository { docker }
    }
}

impl DockerRepository for BollardDockerRepository {
    async fn start_container(&self, container: &Container) -> Result<(), AppError> {
        self.docker
            .start_container(container.name(), None)
            .await
            .or_else(|err| Err(AppError::Repository(err.to_string())))
    }

    async fn stop_container(&self, container: &Container) -> Result<(), AppError> {
        self.docker
            .stop_container(container.name(), None)
            .await
            .or_else(|err| Err(AppError::Repository(err.to_string())))
    }

    async fn remove_container(&self, container: &Container) -> Result<(), AppError> {
        self.docker
            .remove_container(container.name(), None)
            .await
            .or_else(|err| Err(AppError::Repository(err.to_string())))
    }

    async fn create_container(&self, image: &Image, name: &str) -> Result<(), AppError> {
        let options = ListImagesOptionsBuilder::default().all(true).build();
        let images = self
            .docker
            .list_images(Some(options))
            .await
            .map_err(|err| AppError::Repository(err.to_string()))?;
        let image_ids: Vec<String> = images.iter().map(|i| i.id.clone()).collect(); // TODO redo
        if !image_ids.contains(&image.id().to_string()) {
            return Err(AppError::Repository(format!(
                "Image does not exist. ID: {}",
                image.id()
            )));
        }

        let options = CreateContainerOptionsBuilder::default().name(name).build();
        let config = ContainerCreateBody {
            image: Some(image.id().to_string()),
            env: Some(Vec::from(["accept_eula=Y".to_string()])),
            ..Default::default()
        };

        self.docker
            .create_container(Some(options), config)
            .await
            .map_err(|err| AppError::Repository(err.to_string()));
        Ok(())
    }

    async fn list_containers(&self) -> Result<Vec<Container>, AppError> {
        let options = bollard::query_parameters::ListContainersOptionsBuilder::default()
            .all(true)
            .build();
        let container_sum: Vec<bollard::plugin::ContainerSummary> = self
            .docker
            .list_containers(Some(options))
            .await
            .map_err(|err| AppError::Repository(err.to_string()))?;
        let mut containers: Vec<Container> = Vec::new();
        for cont in container_sum {
            let name = match cont.names {
                Some(n) => n[0].clone().trim_start_matches("/").to_string(),
                None => "NA".to_string(),
            };
            let id = match cont.id {
                Some(id) => id,
                None => "NA".to_string(),
            };
            let status = match cont.status {
                Some(status) => status,
                None => "NA".to_string(),
            };
            let version: String = match &cont.labels {
                Some(v) => v.get("version").unwrap().clone(),
                None => "NA".to_string(),
            };
            let country: String = match &cont.labels {
                Some(v) => v.get("country").unwrap().clone(),
                None => "NA".to_string(),
            };
            containers.push(Container::new(name, id, status, version, country));
        }
        Ok(containers)
    }

    async fn inspect_container(&self, name: &str) -> Result<Container, AppError> {
        todo!()
    }

    async fn remove_image(&self, image: &Image) -> Result<(), AppError> {
        todo!()
    }

    async fn inspect_image(&self, name: &str) -> Result<Image, AppError> {
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

    async fn build_image(&self, tar_data: Vec<u8>, image_name: &str) -> Result<(), AppError> {
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
