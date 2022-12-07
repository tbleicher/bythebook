use crate::graphql::types::Project;
use crate::repo_provider::RepoProviderGraphql;
use async_graphql::Error;
use async_graphql::{Context, Object, Result};
use domain::use_cases::ProjectUseCases;

#[derive(Default)]
pub struct ProjectsQuery;

#[Object]
impl ProjectsQuery {
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();

        let list_result = ProjectUseCases::list_projects(repo_provider).await;

        match list_result {
            Ok(entities) => {
                let graphql_projects = entities.iter().map(|n| Project::from_entity(n)).collect();
                Ok(graphql_projects)
            }
            Err(error) => Err(Error::new(error.to_string())),
        }
    }

    async fn project(&self, ctx: &Context<'_>, id: String) -> Result<Option<Project>> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();

        let search_result = ProjectUseCases::get_project_by_id(repo_provider, id).await;

        let option = match search_result {
            Ok(option) => option,
            Err(error) => return Err(Error::new(error.to_string())),
        };

        match option {
            Some(entity) => Ok(Some(Project::from_entity(&entity))),
            None => Ok(None),
        }
    }
}
