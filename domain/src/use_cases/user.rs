use crate::{
    entities::user::{NewUserDTO, User},
    errors::RepositoryError,
    interfaces::RepoProvider,
};

pub struct UserUseCases {}

impl UserUseCases {
    pub async fn create_user(
        repo_provider: &impl RepoProvider,
        dto: NewUserDTO,
    ) -> Result<User, RepositoryError> {
        let repo = repo_provider.get_user_repo();
        let result = repo.create(dto).await;

        result
    }

    pub async fn delete_user(
        repo_provider: &impl RepoProvider,
        id: String,
    ) -> Result<User, RepositoryError> {
        let repo = repo_provider.get_user_repo();
        let result = repo.delete_by_id(id).await;

        result
    }

    pub async fn get_user_by_id(
        repo_provider: &impl RepoProvider,
        user_id: String,
    ) -> Result<Option<User>, RepositoryError> {
        let repo = repo_provider.get_user_repo();
        let search_result = repo.find_one_by_id(user_id).await;

        let option = match search_result {
            Ok(option) => option,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        let user = match option {
            Some(entity) => entity,
            None => return Ok(None),
        };

        Ok(Some(user))
    }

    pub async fn list_users(
        repo_provider: &impl RepoProvider,
    ) -> Result<Vec<User>, RepositoryError> {
        let repo = repo_provider.get_user_repo();
        let list_result = repo.list().await;

        let users = match list_result {
            Ok(list) => list,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        Ok(users)
    }
}
