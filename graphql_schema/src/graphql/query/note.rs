use crate::graphql::types::Note;
use crate::repo_provider::RepoProviderGraphql;
use async_graphql::{Context, Object, Result};
use domain::interfaces::RepoProvider;

#[derive(Default)]
pub struct NotesQuery;

#[Object]
impl NotesQuery {
    async fn notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();
        let repo = repo_provider.get_note_repo();

        let result = repo
            .list()
            .await
            .map(|v| v.iter().map(Note::from_entity).collect())
            .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn note(&self, ctx: &Context<'_>, id: String) -> Result<Option<Note>> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();
        let repo = repo_provider.get_note_repo();

        let option = repo.find_one_by_id(id).await.map_err(|e| e.to_string())?;

        match option {
            Some(n) => Ok(Some(Note::from_entity(&n))),
            None => Ok(None),
        }

        // Ok(option)
    }
}
