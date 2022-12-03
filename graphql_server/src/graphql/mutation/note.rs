use async_graphql::{self, Context, Error, Object, Result};

use crate::{
    graphql::types::{CreateNoteInput, Note},
    repo_provider::RepoProviderGraphql,
};
use domain::use_cases::NoteUseCases;

#[derive(Default)]
pub struct NotesMutation;

#[Object]
impl NotesMutation {
    pub async fn create_note(&self, ctx: &Context<'_>, input: CreateNoteInput) -> Result<Note> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();
        let result = NoteUseCases::create_note(repo_provider, input.into_dto()).await;

        match result {
            Ok(entity) => Ok(Note::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }

    pub async fn delete_note(&self, ctx: &Context<'_>, id: String) -> Result<Note> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();
        let result = NoteUseCases::delete_note(repo_provider, id).await;

        match result {
            Ok(entity) => Ok(Note::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }
}
