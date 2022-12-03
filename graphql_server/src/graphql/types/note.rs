use async_graphql::{Context, InputObject, Object, SimpleObject};
use domain::{
    entities::note::NewNoteDTO, entities::note::Note as NoteEntity, interfaces::RepoProvider,
};

use super::{errors::ResolverError, Project};
use crate::repo_provider::RepoProviderGraphql;

#[derive(InputObject)]
pub struct CreateNoteInput {
    pub title: String,
    pub text: String,
    pub project_id: String,
    pub organisation_id: String,
}

impl CreateNoteInput {
    pub fn into_dto(self) -> NewNoteDTO {
        NewNoteDTO {
            title: self.title,
            text: self.text,
            organisation_id: self.organisation_id,
            project_id: self.project_id,
        }
    }
}

#[derive(SimpleObject)]
pub struct DeleteNoteResult {
    pub note: Note,
}

pub struct Note {
    pub id: String,
    pub title: String,
    pub text: String,
    pub project_id: String,
    pub organisation_id: String,
}

impl Note {
    pub fn from_entity(entity: &NoteEntity) -> Note {
        Note {
            id: entity.id.clone(),
            organisation_id: entity.organisation_id.clone(),
            project_id: entity.project_id.clone(),
            text: entity.text.clone(),
            title: entity.title.clone(),
        }
    }
}

#[Object]
impl Note {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn title(&self) -> String {
        self.title.to_string()
    }

    async fn text(&self) -> String {
        self.text.to_string()
    }

    async fn project(&self, ctx: &Context<'_>) -> Result<Project, ResolverError> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();
        let repo = repo_provider.get_project_repo();

        let result = repo.find_one_by_id(self.project_id.clone()).await;

        let option = match result {
            Ok(option) => option,
            Err(error) => return Err(ResolverError::new(&error.to_string())),
        };

        match option {
            Some(pr) => Ok(Project::from_entity(&pr)),
            None => Err(ResolverError::new("parent project not found")),
        }

        // let fut = match o.recv() {
        //     Ok(idx) => printer(idx),
        //     Err(_) => break,
        // };
        // fut.await;
    }

    async fn organisation_id(&self) -> String {
        self.organisation_id.to_string()
    }
}
