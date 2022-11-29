use crate::{
    entities::project::{NewProjectDTO, Project},
    errors::RepositoryError,
    interfaces::ProjectRepository,
};

pub struct ProjectUseCases {}

impl ProjectUseCases {
    pub async fn create_project(
        repo: impl ProjectRepository,
        dto: NewProjectDTO,
    ) -> Result<Project, RepositoryError> {
        let result = repo.create(dto).await;

        result
    }

    pub async fn delete_project(
        repo: impl ProjectRepository,
        id: String,
    ) -> Result<Project, RepositoryError> {
        let result = repo.delete_by_id(id).await;

        result
    }

    pub async fn get_project_by_id(
        repo: impl ProjectRepository,
        project_id: String,
    ) -> Result<Option<Project>, RepositoryError> {
        let search_result = repo.find_one_by_id(project_id).await;

        let option = match search_result {
            Ok(option) => option,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        let project = match option {
            Some(entity) => entity,
            None => return Ok(None),
        };

        Ok(Some(project))
    }

    pub async fn list_projects(
        repo: impl ProjectRepository,
    ) -> Result<Vec<Project>, RepositoryError> {
        let list_result = repo.list().await;

        let projects = match list_result {
            Ok(list) => list,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        Ok(projects)
    }
}
