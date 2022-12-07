use async_graphql::{self, Context, Error, Object, Result};
use domain::use_cases::ProjectUseCases;

use crate::{
    graphql::types::{CreateProjectInput, Project},
    repo_provider::RepoProviderGraphql,
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
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();

        let result = ProjectUseCases::create_project(repo_provider, input.into_dto()).await;

        match result {
            Ok(entity) => Ok(Project::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }

    pub async fn delete_project(&self, ctx: &Context<'_>, id: String) -> Result<Project> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();

        let result = ProjectUseCases::delete_project(repo_provider, id).await;

        match result {
            Ok(entity) => Ok(Project::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }
}
