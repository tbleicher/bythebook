use crate::db::Database;
use crate::graphql::types::Note;
use adapter_sql::repositories::NoteRepositorySql;
use async_graphql::{Context, Object, Result};
use domain::interfaces::NoteRepository;

#[derive(Default)]
pub struct NotesQuery;

#[Object]
impl NotesQuery {
    async fn notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let db = ctx.data::<Database>().unwrap();
        let repo = NoteRepositorySql {
            db: db.get_connection(),
        };

        let result = repo
            .list()
            .await
            .map(|v| v.iter().map(|n| Note::from_entity(n)).collect())
            .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn note(&self, ctx: &Context<'_>, id: String) -> Result<Option<Note>> {
        let db = ctx.data::<Database>().unwrap();
        let repo = NoteRepositorySql {
            db: db.get_connection(),
        };

        let option = repo.find_one_by_id(id).await.map_err(|e| e.to_string())?;

        match option {
            Some(n) => Ok(Some(Note::from_entity(&n))),
            None => Ok(None),
        }

        // Ok(option)
    }
}
