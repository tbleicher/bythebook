use crate::{
    entities::user::{AuthUser, NewUserDTO, User},
    errors::RepositoryError,
    interfaces::RepoProvider,
};
use argonautica::Hasher;
use nanoid::nanoid;

pub fn hash_password(password: &str, hash_secret: String) -> String {
    let mut hasher = Hasher::default();
    hasher
        .with_password(password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap()
}

pub struct UserUseCases {}

impl UserUseCases {
    pub async fn create_user(
        repo_provider: &impl RepoProvider,
        dto: NewUserDTO,
    ) -> Result<User, RepositoryError> {
        let repo = repo_provider.get_user_repo();

        let user_tmp = AuthUser {
            id: nanoid!(10, &nanoid::alphabet::SAFE),
            deleted: false,
            email: dto.email.to_string(),
            email_verified: false,
            name: dto.name.to_string(),
            organisation_id: dto.organisation_id.to_string(),
            password_hash: hash_password("password", "secret".to_string()), // XXX
            verify_token: nanoid!(20, &nanoid::alphabet::SAFE),
        };

        let create_result = repo.create(user_tmp).await;
        let user = match create_result {
            Ok(user) => user,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        // TODO: send user email

        Ok(user)
    }

    pub async fn get_auth_user(
        repo_provider: &impl RepoProvider,
        email: String,
    ) -> Result<AuthUser, RepositoryError> {
        let repo = repo_provider.get_user_repo();
        let user_option = repo.find_auth_user(email).await?;

        match user_option {
            Some(user) => Ok(user),
            None => return Err(RepositoryError::new("user not found")),
        }
    }

    pub async fn delete_user(
        repo_provider: &impl RepoProvider,
        id: String,
    ) -> Result<User, RepositoryError> {
        let repo = repo_provider.get_user_repo();
        repo.delete_by_id(id).await
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
