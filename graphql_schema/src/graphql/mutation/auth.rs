use async_graphql::{self, Context, Error, Object, Result};
use domain::use_cases::UserUseCases;

use crate::{
    graphql::types::{CreateUserInput, DeleteUserResult, User},
    repo_provider::RepoProviderGraphql,
};

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();
        let result = UserUseCases::create_user(repo_provider, input.into_dto()).await;

        match result {
            Ok(entity) => Ok(User::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, id: String) -> Result<DeleteUserResult> {
        let repo_provider = ctx.data::<RepoProviderGraphql>().unwrap();
        let result = UserUseCases::delete_user(repo_provider, id).await;

        match result {
            Ok(entity) => Ok(DeleteUserResult {
                user: User::from_entity(&entity),
            }),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }
}
