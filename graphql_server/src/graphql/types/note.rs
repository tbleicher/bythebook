use adapter_sql::repositories::ProjectRepositorySql;
use async_graphql::{Context, InputObject, Object, SimpleObject};
use domain::{
    entities::note::NewNoteDTO, entities::note::Note as NoteEntity, interfaces::ProjectRepository,
};

use super::{errors::ResolverError, Project};
use crate::db::Database;

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
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        let repo = ProjectRepositorySql { db: conn };

        let option = repo
            .find_one_by_id(self.project_id.clone())
            .await
            .map_err(|e| e.to_string());

        match option {
            Ok(Some(pr)) => Ok(Project::from_entity(&pr)),
            _ => Err(ResolverError::new("parent project not found")),
        }
    }

    async fn organisation_id(&self) -> String {
        self.organisation_id.to_string()
    }
}
