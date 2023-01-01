use crate::{
    entities::note::{NewNoteDTO, Note},
    errors::RepositoryError,
    interfaces::RepoProvider,
};

pub struct NoteUseCases {}

impl NoteUseCases {
    pub async fn create_note(
        repo_provider: &impl RepoProvider,
        dto: NewNoteDTO,
    ) -> Result<Note, RepositoryError> {
        let repo = repo_provider.get_note_repo();
        repo.create(dto).await
    }

    pub async fn delete_note(
        repo_provider: &impl RepoProvider,
        id: String,
    ) -> Result<Note, RepositoryError> {
        let repo = repo_provider.get_note_repo();
        repo.delete_by_id(id).await
    }

    pub async fn get_note_by_id(
        repo_provider: &impl RepoProvider,
        note_id: String,
    ) -> Result<Option<Note>, RepositoryError> {
        let repo = repo_provider.get_note_repo();
        let search_result = repo.find_one_by_id(note_id).await;

        let option = match search_result {
            Ok(option) => option,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        let note = match option {
            Some(entity) => entity,
            None => return Ok(None),
        };

        Ok(Some(note))
    }

    pub async fn list_notes(
        repo_provider: &impl RepoProvider,
    ) -> Result<Vec<Note>, RepositoryError> {
        let repo = repo_provider.get_note_repo();
        let list_result = repo.list().await;

        let notes = match list_result {
            Ok(list) => list,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        Ok(notes)
    }
}
