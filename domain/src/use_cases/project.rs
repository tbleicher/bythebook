use crate::{
    entities::project::{NewProjectDTO, Project},
    errors::RepositoryError,
    interfaces::RepoProvider,
};

pub struct ProjectUseCases {}

impl ProjectUseCases {
    pub async fn create_project(
        repo_provider: &impl RepoProvider,
        dto: NewProjectDTO,
    ) -> Result<Project, RepositoryError> {
        let repo = repo_provider.get_project_repo();
        let result = repo.create(dto).await;

        result
    }

    pub async fn delete_project(
        repo_provider: &impl RepoProvider,
        id: String,
    ) -> Result<Project, RepositoryError> {
        let repo = repo_provider.get_project_repo();
        let result = repo.delete_by_id(id).await;

        result
    }

    pub async fn get_project_by_id(
        repo_provider: &impl RepoProvider,
        project_id: String,
    ) -> Result<Option<Project>, RepositoryError> {
        let repo = repo_provider.get_project_repo();
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
        repo_provider: &impl RepoProvider,
    ) -> Result<Vec<Project>, RepositoryError> {
        let repo = repo_provider.get_project_repo();
        let list_result = repo.list().await;

        let projects = match list_result {
            Ok(list) => list,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        Ok(projects)
    }
}
