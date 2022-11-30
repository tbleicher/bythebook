use adapter_sql::repositories::UserRepositorySql;
use async_graphql::{Context, InputObject, Object, SimpleObject};
use domain::{
    entities::organisation::NewOrganisationDTO,
    entities::organisation::Organisation as OrganisationEntity, interfaces::UserRepository,
};

use super::errors::ResolverError;
use super::User;
use crate::db::Database;

#[derive(InputObject)]
pub struct CreateOrganisationInput {
    pub name: String,
    pub admin_email: String,
    pub admin_name: String,
}

impl CreateOrganisationInput {
    pub fn into_dto(self) -> NewOrganisationDTO {
        NewOrganisationDTO {
            name: self.name,
            admin_email: self.admin_email,
            admin_name: self.admin_name,
        }
    }
}

#[derive(SimpleObject)]
pub struct DeleteOrganisationResult {
    pub organisation: Organisation,
}

pub struct Organisation {
    pub id: String,
    pub name: String,
    pub admin_id: String,
    pub active: bool,
    pub deleted: bool,
}

impl Organisation {
    pub fn from_entity(entity: &OrganisationEntity) -> Organisation {
        Organisation {
            id: entity.id.clone(),
            name: entity.name.clone(),
            active: entity.active.clone(),
            admin_id: entity.admin_id.clone(),
            deleted: entity.deleted.clone(),
        }
    }
}

#[Object]
impl Organisation {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn name(&self) -> String {
        self.name.to_string()
    }

    async fn active(&self) -> bool {
        self.active.to_owned()
    }

    async fn admin(&self, ctx: &Context<'_>) -> Result<User, ResolverError> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        let repo = UserRepositorySql { db: conn };

        let option = repo
            .find_one_by_id(self.admin_id.clone())
            .await
            .map_err(|e| e.to_string());

        match option {
            Ok(Some(user)) => Ok(User::from_entity(&user)),
            _ => Err(ResolverError::new("organisation admin not found")),
        }
    }

    async fn deleted(&self) -> bool {
        self.deleted.to_owned()
    }
}
