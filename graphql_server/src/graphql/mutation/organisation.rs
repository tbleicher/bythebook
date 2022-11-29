use adapter_sql::repositories::{OrganisationRepositorySql, UserRepositorySql};
use async_graphql::{self, Context, Error, Object, Result};
use domain::use_cases::OrganisationUseCases;

use crate::{
    db::Database,
    graphql::types::{CreateOrganisationInput, Organisation},
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
        let db = ctx.data::<Database>().unwrap();
        let repo = OrganisationRepositorySql {
            db: db.get_connection(),
        };
        let user_repo = UserRepositorySql {
            db: db.get_connection(),
        };

        let result =
            OrganisationUseCases::create_organisation(repo, user_repo, input.into_dto()).await;

        match result {
            Ok(entity) => Ok(Organisation::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }

    pub async fn delete_organisation(&self, ctx: &Context<'_>, id: String) -> Result<Organisation> {
        let db = ctx.data::<Database>().unwrap();
        let repo = OrganisationRepositorySql {
            db: db.get_connection(),
        };
        let user_repo = UserRepositorySql {
            db: db.get_connection(),
        };

        let result = OrganisationUseCases::delete_organisation(repo, user_repo, id).await;

        match result {
            Ok(entity) => Ok(Organisation::from_entity(&entity)),
            Err(error) => Err(Error::new(error.to_string())),
        }
    }
}
