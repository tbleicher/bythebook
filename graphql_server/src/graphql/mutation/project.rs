use adapter_sql::repositories::ProjectRepositorySql;
use async_graphql::{self, Context, Error, Object, Result};
use domain::use_cases::ProjectUseCases;

use crate::{
    db::Database,
    graphql::types::{CreateProjectInput, Project},
};

#[derive(Default)]
pub struct ProjectsMutation;

#[Object]
impl ProjectsMutation {
    pub async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> Result<Project> {
        let db = ctx.data::<Database>().unwrap();
        let repo = ProjectRepositorySql {
            db: db.get_connection(),
        };

        let result = ProjectUseCases::create_project(repo, input.into_dto()).await;

        match result {
            Ok(entity) => Ok(Project::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }

    pub async fn delete_project(&self, ctx: &Context<'_>, id: String) -> Result<Project> {
        let db = ctx.data::<Database>().unwrap();
        let repo = ProjectRepositorySql {
            db: db.get_connection(),
        };

        let result = ProjectUseCases::delete_project(repo, id).await;

        match result {
            Ok(entity) => Ok(Project::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }
}
