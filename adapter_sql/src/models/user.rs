use domain::entities::user::{AuthUser, User};
use sea_orm::{entity::prelude::*, ActiveValue::NotSet, DeleteMany, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub deleted: bool,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub organisation_id: String,
    pub password_hash: String,
    pub verify_token: String,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<AuthUser> for ActiveModel {
    fn from(user: AuthUser) -> Self {
        ActiveModel {
            id: Set(user.id),
            deleted: Set(user.deleted),
            email: Set(user.email),
            email_verified: Set(user.email_verified),
            name: Set(user.name),
            organisation_id: Set(user.organisation_id),
            password_hash: Set(user.password_hash),
            verify_token: Set(user.verify_token),
        }
    }
}

impl From<User> for ActiveModel {
    fn from(user: User) -> Self {
        ActiveModel {
            id: Set(user.id),
            deleted: Set(user.deleted),
            email: Set(user.email),
            email_verified: Set(user.email_verified),
            name: Set(user.name),
            organisation_id: Set(user.organisation_id),
            password_hash: NotSet,
            verify_token: NotSet,
        }
    }
}

impl Entity {
    pub fn find_by_id(id: String) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_email(email: &str) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }

    pub fn find_by_verification_token(token: String) -> Select<Entity> {
        Self::find().filter(Column::VerifyToken.eq(token))
    }

    pub fn delete_by_id(id: String) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
