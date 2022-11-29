use crate::models::user;
use domain::async_trait::async_trait;
use domain::entities::user::{NewUserDTO, User};
use domain::errors::RepositoryError;
use domain::interfaces::UserRepository;

use nanoid::nanoid;
use sea_orm::*;

fn convert_to_entity(model: user::Model) -> User {
    User {
        id: model.id.to_string(),
        email: model.email.to_string(),
        name: model.name.to_string(),
        organisation_id: model.organisation_id.to_string(),
    }
}
pub struct UserRepositorySql<'a> {
    pub db: &'a DbConn,
}

#[async_trait]
impl UserRepository for UserRepositorySql<'_> {
    async fn create(&self, dto: NewUserDTO) -> Result<User, RepositoryError> {
        let new_user = user::ActiveModel {
            id: Set(nanoid!(10, &nanoid::alphabet::SAFE)),
            email: Set(dto.email.to_owned()),
            name: Set(dto.name.to_owned()),
            organisation_id: Set(dto.organisation_id.to_owned()),
        };

        let insert_result = new_user.insert(self.db).await;

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
        let delete_result = user::Entity::delete_many()
            .filter(user::Column::OrganisationId.eq(id))
            .exec(self.db)
            .await;

        match delete_result {
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
            Ok(_) => Ok(()),
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
        let result = user::Entity::find_by_id(data.id).one(self.db).await;
        let user = match result {
            Ok(model_or_none) => match model_or_none {
                Some(model) => model,
                None => return Err(RepositoryError::new("user not found")),
            },
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        let update_result = user::ActiveModel {
            id: Set(user.id),
            email: Set(data.email.to_owned()),
            name: Set(data.name.to_owned()),
            organisation_id: Set(data.organisation_id.to_owned()),
        }
        .update(self.db)
        .await;

        match update_result {
            Ok(model) => Ok(convert_to_entity(model.clone())),
            Err(error) => Err(RepositoryError::new(&error.to_string())),
        }
    }
}
