use async_trait::async_trait;

use crate::{entities::organisation::Organisation, errors::RepositoryError};

#[async_trait]
pub trait OrganisationRepository {
    async fn create(&self, organisation: Organisation) -> Result<Organisation, RepositoryError>;
    async fn delete_by_id(&self, id: String) -> Result<Organisation, RepositoryError>;
    async fn find_one_by_id(&self, id: String) -> Result<Option<Organisation>, RepositoryError>;
    async fn list(&self) -> Result<Vec<Organisation>, RepositoryError>;
    async fn update(&self, data: Organisation) -> Result<Organisation, RepositoryError>;
}
