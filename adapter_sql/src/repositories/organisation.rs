use crate::models::organisation;
use domain::async_trait::async_trait;
use domain::entities::organisation::Organisation;
use domain::errors::RepositoryError;
use domain::interfaces::OrganisationRepository;

use nanoid::nanoid;
use sea_orm::*;

fn convert_to_entity(model: organisation::Model) -> Organisation {
    Organisation {
        id: model.id.to_string(),
        name: model.name.to_string(),
        admin_id: model.admin_id.to_string(),
    }
}
pub struct OrganisationRepositorySql<'a> {
    pub db: &'a DbConn,
}

#[async_trait]
impl OrganisationRepository for OrganisationRepositorySql<'_> {
    async fn create(
        &self,
        name: String,
        admin_id: String,
    ) -> Result<Organisation, RepositoryError> {
        let new_organisation = organisation::ActiveModel {
            id: Set(nanoid!(10, &nanoid::alphabet::SAFE)),
            name: Set(name.to_owned()),
            admin_id: Set(admin_id.to_owned()),
        };

        let insert_result = new_organisation.insert(self.db).await;

        match insert_result {
            Ok(model) => Ok(convert_to_entity(model)),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn delete_by_id(&self, id: String) -> Result<Organisation, RepositoryError> {
        let db_result = organisation::Entity::find_by_id(id).one(self.db).await;

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
                None => Err(RepositoryError::new("Organisation not found.")),
            },
        }
    }

    async fn find_one_by_id(&self, id: String) -> Result<Option<Organisation>, RepositoryError> {
        let record = organisation::Entity::find_by_id(id).one(self.db).await;

        match record {
            Ok(model_or_none) => match model_or_none {
                Some(model) => Ok(Some(convert_to_entity(model))),

                None => Ok(None),
            },
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn list(&self) -> Result<Vec<Organisation>, RepositoryError> {
        let organisations_list = organisation::Entity::find().all(self.db).await;

        match organisations_list {
            Ok(models) => Ok(models
                .iter()
                .map(|model| convert_to_entity(model.clone()))
                .collect()),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn update(&self, data: Organisation) -> Result<Organisation, RepositoryError> {
        let record = organisation::Entity::find_by_id(data.id).one(self.db).await;

        let current = match record {
            Ok(model_or_none) => match model_or_none {
                Some(model) => model,
                None => return Err(RepositoryError::new("DB error: entity not found")),
            },
            Err(error) => return Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        };

        let update_result = organisation::ActiveModel {
            id: Set(current.id.to_owned()),
            name: Set(data.name.to_owned()),
            admin_id: Set(data.admin_id.to_owned()),
        }
        .update(self.db)
        .await;

        match update_result {
            Ok(model) => Ok(convert_to_entity(model)),
            Err(error) => return Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }
}
