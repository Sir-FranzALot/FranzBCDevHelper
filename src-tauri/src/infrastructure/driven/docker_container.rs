use crate::{
    application::{error::AppError, ports::container::ContainerRepository},
    domain::{Container, Image},
};
use bollard::config::ContainerCreateBody;
use bollard::query_parameters::{CreateContainerOptionsBuilder, ListImagesOptionsBuilder};
use bollard::Docker;

struct DockerContainerRepository {
    docker: Docker,
}

impl DockerContainerRepository {
    fn new(docker: Docker) -> DockerContainerRepository {
        DockerContainerRepository { docker }
    }
}

impl ContainerRepository for DockerContainerRepository {
    async fn start(&self, container: &Container) -> Result<(), AppError> {
        self.docker
            .start_container(container.name(), None)
            .await
            .or_else(|err| Err(AppError::Repository(err.to_string())))
    }

    async fn stop(&self, container: &Container) -> Result<(), AppError> {
        self.docker
            .stop_container(container.name(), None)
            .await
            .or_else(|err| Err(AppError::Repository(err.to_string())))
    }

    async fn remove(&self, container: &Container) -> Result<(), AppError> {
        self.docker
            .remove_container(container.name(), None)
            .await
            .or_else(|err| Err(AppError::Repository(err.to_string())))
    }

    async fn create(&self, image: &Image, name: &str) -> Result<(), AppError> {
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

    async fn inspect(&self, name: &str) -> Result<Container, AppError> {
        todo!()
    }
}
