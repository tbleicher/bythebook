use async_graphql::{Context, InputObject, Object, SimpleObject};
use domain::entities::project::{NewProjectDTO, Project as ProjectEntity};

use super::{errors::ResolverError, Note};
use crate::db::Database;
use adapter_sql::repositories::NoteRepositorySql;
use domain::interfaces::NoteRepository;

#[derive(InputObject)]
pub struct CreateProjectInput {
    pub title: String,
    pub description: String,
    pub organisation_id: String,
}

impl CreateProjectInput {
    pub fn into_dto(self) -> NewProjectDTO {
        NewProjectDTO {
            title: self.title,
            description: self.description,
            organisation_id: self.organisation_id,
        }
    }
}

#[derive(SimpleObject)]
pub struct DeleteProjectResult {
    pub project: Project,
}

pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub organisation_id: String,
}

impl Project {
    pub fn from_entity(entity: &ProjectEntity) -> Project {
        Project {
            id: entity.id.clone(),
            title: entity.title.clone(),
            description: entity.description.clone(),
            organisation_id: entity.organisation_id.clone(),
        }
    }
}

#[Object]
impl Project {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn description(&self) -> String {
        self.description.to_string()
    }

    async fn organisation_id(&self) -> String {
        self.organisation_id.to_string()
    }

    async fn title(&self) -> String {
        self.title.to_string()
    }

    async fn notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>, ResolverError> {
        let db = ctx.data::<Database>().unwrap();
        let repo = NoteRepositorySql {
            db: db.get_connection(),
        };

        let result = repo.find_by_project_id(self.id.to_owned()).await;

        match result {
            Ok(notes) => {
                let note_types = notes.into_iter().map(|n| Note::from_entity(&n)).collect();
                Ok(note_types)
            }
            _ => Err(ResolverError::new("could not load notes")),
        }
    }
}
