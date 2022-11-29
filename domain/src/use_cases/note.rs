use crate::{
    entities::note::{NewNoteDTO, Note},
    errors::RepositoryError,
    interfaces::NoteRepository,
};

pub struct NoteUseCases {}

impl NoteUseCases {
    pub async fn create_note(
        repo: impl NoteRepository,
        dto: NewNoteDTO,
    ) -> Result<Note, RepositoryError> {
        let result = repo.create(dto).await;

        result
    }

    pub async fn delete_note(
        repo: impl NoteRepository,
        id: String,
    ) -> Result<Note, RepositoryError> {
        let result = repo.delete_by_id(id).await;

        result
    }

    pub async fn get_note_by_id(
        repo: impl NoteRepository,
        note_id: String,
    ) -> Result<Option<Note>, RepositoryError> {
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

    pub async fn list_notes(repo: impl NoteRepository) -> Result<Vec<Note>, RepositoryError> {
        let list_result = repo.list().await;

        let notes = match list_result {
            Ok(list) => list,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        Ok(notes)
    }
}
