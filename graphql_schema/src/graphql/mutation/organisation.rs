use async_graphql::{self, Context, Error, Object, Result};
use domain::use_cases::OrganisationUseCases;

use crate::{
    graphql::types::{CreateOrganisationInput, Organisation},
    repo_provider::RepoProviderGraphql,
};

#[derive(Default)]
pub struct OrganisationMutation;

#[Object]
impl OrganisationMutation {
    pub async fn create_organisation(
        &self,
        ctx: &Context<'_>,
        input: CreateOrganisationInput,
    ) -> Result<Organisation> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();

        let result =
            OrganisationUseCases::create_organisation(repo_provider, input.into_dto()).await;

        match result {
            Ok(entity) => Ok(Organisation::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }

    pub async fn delete_organisation(&self, ctx: &Context<'_>, id: String) -> Result<Organisation> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();

        let result = OrganisationUseCases::delete_organisation(repo_provider, id).await;

        match result {
            Ok(entity) => Ok(Organisation::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }
}
