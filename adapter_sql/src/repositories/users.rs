use crate::models::user;
use domain::async_trait::async_trait;
use domain::entities::user::{AuthUser, User, VerifyEmailDTO};
use domain::errors::RepositoryError;
use domain::interfaces::UserRepository;

use sea_orm::sea_query::Expr;
use sea_orm::*;
use sea_orm::{ActiveModelTrait, Set};

fn convert_to_entity(model: user::Model) -> User {
    User {
        id: model.id.to_string(),
        deleted: model.deleted.to_owned(),
        email: model.email.to_string(),
        email_verified: model.email_verified.to_owned(),
        name: model.name.to_string(),
        organisation_id: model.organisation_id,
    }
}

fn convert_to_auth_user(model: user::Model) -> AuthUser {
    AuthUser {
        id: model.id.to_string(),
        deleted: model.deleted.to_owned(),
        email: model.email.to_string(),
        email_verified: model.email_verified.to_owned(),
        name: model.name.to_string(),
        organisation_id: model.organisation_id.to_string(),
        password_hash: model.password_hash.to_string(),
        verify_token: model.verify_token,
    }
}

pub struct UserRepositorySql<'a> {
    pub db: &'a DbConn,
}

#[async_trait]
impl UserRepository for UserRepositorySql<'_> {
    async fn create(&self, data: AuthUser) -> Result<User, RepositoryError> {
        let insert_result = user::ActiveModel::from(data).insert(self.db).await;

        match insert_result {
            Ok(model) => Ok(convert_to_entity(model)),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn delete_by_id(&self, id: String) -> Result<User, RepositoryError> {
        let db_result = user::Entity::find_by_id(id).one(self.db).await;

        match db_result {
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
            Ok(model_or_none) => match model_or_none {
                Some(record) => {
                    let model = record.clone();

                    match record.delete(self.db).await {
                        Ok(_delete_result) => Ok(convert_to_entity(model)),
                        Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
                    }
                }
                None => Err(RepositoryError::new("User not found.")),
            },
        }
    }

    async fn delete_by_organisation_id(&self, id: String) -> Result<(), RepositoryError> {
        let update_result = user::Entity::update_many()
            .col_expr(user::Column::Deleted, Expr::value(true))
            .filter(user::Column::OrganisationId.eq(id))
            .exec(self.db)
            .await;

        match update_result {
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
            Ok(_) => Ok(()),
        }
    }

    async fn find_auth_user(&self, email: String) -> Result<Option<AuthUser>, RepositoryError> {
        let record = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(self.db)
            .await;

        match record {
            Ok(model_or_none) => match model_or_none {
                Some(model) => Ok(Some(convert_to_auth_user(model))),
                None => Ok(None),
            },
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn find_by_organisation_id(
        &self,
        organisation_id: String,
    ) -> Result<Vec<User>, RepositoryError> {
        let users_list = user::Entity::find()
            .filter(user::Column::OrganisationId.eq(organisation_id))
            .all(self.db)
            .await;

        match users_list {
            Ok(models) => Ok(models
                .iter()
                .map(|model| convert_to_entity(model.clone()))
                .collect()),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn find_one_by_id(&self, id: String) -> Result<Option<User>, RepositoryError> {
        let record = user::Entity::find_by_id(id).one(self.db).await;

        match record {
            Ok(model_or_none) => match model_or_none {
                Some(model) => Ok(Some(convert_to_entity(model))),
                None => Ok(None),
            },
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn list(&self) -> Result<Vec<User>, RepositoryError> {
        let users_list = user::Entity::find().all(self.db).await;

        match users_list {
            Ok(models) => Ok(models
                .iter()
                .map(|model| convert_to_entity(model.clone()))
                .collect()),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn update(&self, data: User) -> Result<User, RepositoryError> {
        let result = user::Entity::find_by_id(data.id.clone()).one(self.db).await;
        let _user = match result {
            Ok(model_or_none) => match model_or_none {
                Some(model) => model,
                None => return Err(RepositoryError::new("user not found")),
            },
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        let update_result = user::ActiveModel::from(data).update(self.db).await;

        match update_result {
            Ok(model) => Ok(convert_to_entity(model)),
            Err(error) => Err(RepositoryError::new(&error.to_string())),
        }
    }

    async fn verify_email(&self, data: VerifyEmailDTO) -> Result<User, RepositoryError> {
        let result = user::Entity::find_by_verification_token(data.token)
            .one(self.db)
            .await;

        let user = match result {
            Ok(model_or_none) => match model_or_none {
                Some(model) => model,
                None => return Err(RepositoryError::new("user not found")),
            },
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        let password_hash = "TODO: password hash";

        let update_result = user::ActiveModel {
            email_verified: Set(true),
            password_hash: Set(password_hash.to_string()),
            verify_token: Set("".to_string()),
            ..user.into()
        }
        .update(self.db)
        .await;

        match update_result {
            Ok(user) => Ok(convert_to_entity(user)),
            Err(error) => Err(RepositoryError::new(&error.to_string())),
        }
    }
}
