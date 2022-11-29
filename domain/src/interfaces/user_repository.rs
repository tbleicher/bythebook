use async_trait::async_trait;

use crate::{
    entities::user::{NewUserDTO, User},
    errors::RepositoryError,
};

#[async_trait]
pub trait UserRepository {
    async fn create(&self, dto: NewUserDTO) -> Result<User, RepositoryError>;
    async fn delete_by_id(&self, id: String) -> Result<User, RepositoryError>;
    async fn delete_by_organisation_id(&self, id: String) -> Result<(), RepositoryError>;
    async fn find_by_organisation_id(&self, id: String) -> Result<Vec<User>, RepositoryError>;
    async fn find_one_by_id(&self, id: String) -> Result<Option<User>, RepositoryError>;
    async fn list(&self) -> Result<Vec<User>, RepositoryError>;
    async fn update(&self, data: User) -> Result<User, RepositoryError>;
}
