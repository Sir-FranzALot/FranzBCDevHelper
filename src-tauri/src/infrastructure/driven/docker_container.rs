use crate::{
    application::{error::AppError, ports::container::ContainerRepository},
    domain::{self, Container},
};
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
            .or_else(|err| Err(AppError::Repository(err.to_string())));
        Ok(())
    }

    async fn stop(&self, container: &Container) -> Result<(), AppError> {
        todo!()
    }

    async fn remove(&self, container: &Container) -> Result<(), AppError> {
        todo!()
    }

    async fn create(&self, image: &domain::Image) -> Result<Container, AppError> {
        todo!()
    }

    async fn list_containers(&self) -> Result<Vec<Container>, AppError> {
        todo!()
    }

    async fn inspect(&self, name: &str) -> Result<Container, AppError> {
        todo!()
    }
}
