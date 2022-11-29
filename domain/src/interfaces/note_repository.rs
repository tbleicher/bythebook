use async_trait::async_trait;

use crate::{
    entities::note::{NewNoteDTO, Note},
    errors::RepositoryError,
};

#[async_trait]
pub trait NoteRepository {
    async fn create(&self, dto: NewNoteDTO) -> Result<Note, RepositoryError>;

    async fn delete_by_id(&self, id: String) -> Result<Note, RepositoryError>;

    async fn find_by_project_id(&self, id: String) -> Result<Vec<Note>, RepositoryError>;

    async fn find_one_by_id(&self, id: String) -> Result<Option<Note>, RepositoryError>;

    async fn list(&self) -> Result<Vec<Note>, RepositoryError>;
}
