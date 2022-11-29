use adapter_sql::repositories::UserRepositorySql;
use async_graphql::{self, Context, Error, Object, Result};
use domain::use_cases::UserUseCases;

use crate::{
    db::Database,
    graphql::types::{CreateUserInput, DeleteUserResult, User},
};

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    pub async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let db = ctx.data::<Database>().unwrap();
        let repo = UserRepositorySql {
            db: db.get_connection(),
        };

        let result = UserUseCases::create_user(repo, input.into_dto()).await;

        match result {
            Ok(entity) => Ok(User::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, id: String) -> Result<DeleteUserResult> {
        let db = ctx.data::<Database>().unwrap();
        let repo = UserRepositorySql {
            db: db.get_connection(),
        };

        let result = UserUseCases::delete_user(repo, id).await;

        match result {
            Ok(entity) => Ok(DeleteUserResult {
                user: User::from_entity(&entity),
            }),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }
}
