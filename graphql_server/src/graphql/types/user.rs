use async_graphql::{InputObject, Object, SimpleObject};
use domain::entities::user::{NewUserDTO, User as UserEntity};

#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub organisation_id: String,
}

impl CreateUserInput {
    pub fn into_dto(self) -> NewUserDTO {
        NewUserDTO {
            name: self.name,
            email: self.email,
            organisation_id: self.organisation_id.to_string(),
        }
    }
}

#[derive(SimpleObject)]
pub struct DeleteUserResult {
    pub user: User,
}

pub struct User {
    pub id: String,
    pub name: String,
    pub deleted: bool,
    pub email: String,
    pub email_verified: bool,
    pub organisation_id: String,
}

impl User {
    pub fn from_entity(entity: &UserEntity) -> User {
        User {
            id: entity.id.clone(),
            deleted: entity.deleted.clone(),
            email: entity.email.clone(),
            email_verified: entity.email_verified.clone(),
            name: entity.name.clone(),
            organisation_id: entity.organisation_id.clone(),
        }
    }
}

#[Object]
impl User {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn deleted(&self) -> bool {
        self.deleted.to_owned()
    }

    async fn email(&self) -> String {
        self.email.to_string()
    }

    async fn email_verified(&self) -> bool {
        self.email_verified.to_owned()
    }

    async fn name(&self) -> String {
        self.name.to_string()
    }
}
