use async_trait::async_trait;

use crate::{
    entities::project::{NewProjectDTO, Project},
    errors::RepositoryError,
};

#[async_trait]
pub trait ProjectRepository {
    async fn create(&self, dto: NewProjectDTO) -> Result<Project, RepositoryError>;

    async fn delete_by_id(&self, id: String) -> Result<Project, RepositoryError>;

    async fn find_one_by_id(&self, id: String) -> Result<Option<Project>, RepositoryError>;

    async fn list(&self) -> Result<Vec<Project>, RepositoryError>;
}
