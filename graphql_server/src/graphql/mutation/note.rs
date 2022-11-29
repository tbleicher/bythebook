use async_graphql::{self, Context, Error, Object, Result};

use crate::{
    db::Database,
    graphql::types::{CreateNoteInput, Note},
};
use adapter_sql::repositories::NoteRepositorySql;
use domain::use_cases::NoteUseCases;

#[derive(Default)]
pub struct NotesMutation;

#[Object]
impl NotesMutation {
    pub async fn create_note(&self, ctx: &Context<'_>, input: CreateNoteInput) -> Result<Note> {
        let db = ctx.data::<Database>().unwrap();
        let repo = NoteRepositorySql {
            db: db.get_connection(),
        };

        let result = NoteUseCases::create_note(repo, input.into_dto()).await;

        match result {
            Ok(entity) => Ok(Note::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }

    pub async fn delete_note(&self, ctx: &Context<'_>, id: String) -> Result<Note> {
        let db = ctx.data::<Database>().unwrap();
        let repo = NoteRepositorySql {
            db: db.get_connection(),
        };

        let result = NoteUseCases::delete_note(repo, id).await;

        match result {
            Ok(entity) => Ok(Note::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }
}
