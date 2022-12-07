use crate::models::organisation;
use domain::async_trait::async_trait;
use domain::entities::organisation::Organisation;
use domain::errors::RepositoryError;
use domain::interfaces::OrganisationRepository;

use sea_orm::*;

pub struct OrganisationRepositorySql<'a> {
    pub db: &'a DbConn,
}

#[async_trait]
impl OrganisationRepository for OrganisationRepositorySql<'_> {
    async fn create(&self, org: Organisation) -> Result<Organisation, RepositoryError> {
        let new_organisation = organisation::ActiveModel::from(org);

        let insert_result = new_organisation.insert(self.db).await;

        match insert_result {
            Ok(model) => Ok(model.into()),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn delete_by_id(&self, id: String) -> Result<Organisation, RepositoryError> {
        let db_result = organisation::Entity::find_by_id(id).one(self.db).await;

        let org = match db_result {
            Err(error) => return Err(RepositoryError::new(&format!("DB error: {:?}", error))),
            Ok(model_or_none) => match model_or_none {
                None => return Err(RepositoryError::new("Organisation not found.")),
                Some(record) => record,
            },
        };

        let updated = Organisation {
            active: false,
            deleted: true,
            ..org.into()
        };

        self.update(updated).await
    }

    async fn find_one_by_id(&self, id: String) -> Result<Option<Organisation>, RepositoryError> {
        let record = organisation::Entity::find_by_id(id).one(self.db).await;

        match record {
            Ok(model_or_none) => match model_or_none {
                Some(model) => Ok(Some(model.into())),

                None => Ok(None),
            },
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn list(&self) -> Result<Vec<Organisation>, RepositoryError> {
        let organisations_list = organisation::Entity::find().all(self.db).await;

        match organisations_list {
            Ok(models) => Ok(models.iter().map(|model| model.clone().into()).collect()),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn update(&self, data: Organisation) -> Result<Organisation, RepositoryError> {
        let record = organisation::Entity::find_by_id(data.id.clone())
            .one(self.db)
            .await;

        let _existing = match record {
            Ok(model_or_none) => match model_or_none {
                Some(model) => model,
                None => return Err(RepositoryError::new("DB error: entity not found")),
            },
            Err(error) => return Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        };

        let update_result = organisation::ActiveModel::from(data).update(self.db).await;

        match update_result {
            Ok(model) => Ok(model.into()),
            Err(error) => return Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }
}
