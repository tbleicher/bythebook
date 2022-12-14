use crate::{
    entities::{
        organisation::{NewOrganisationDTO, Organisation},
        user::NewUserDTO,
    },
    errors::RepositoryError,
    interfaces::RepoProvider,
};
use chrono::Utc;
use nanoid::nanoid;

use super::UserUseCases;

pub struct OrganisationUseCases {}

impl OrganisationUseCases {
    pub async fn create_organisation(
        repo_provider: &impl RepoProvider,
        dto: NewOrganisationDTO,
    ) -> Result<Organisation, RepositoryError> {
        let repo = repo_provider.get_organisation_repo();

        let create_org_result = repo
            .create(Organisation {
                id: nanoid!(10, &nanoid::alphabet::SAFE),
                name: dto.name,
                active: false,
                admin_id: "temporary".to_string(),
                created_at: Utc::now(),
                deleted: false,
            })
            .await;
        let org_tmp = match create_org_result {
            Ok(org) => org,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        let admin_dto = NewUserDTO {
            email: dto.admin_email,
            name: dto.admin_name,
            organisation_id: org_tmp.id.to_string(),
        };
        let create_admin_result = UserUseCases::create_user(repo_provider, admin_dto).await;

        let admin = match create_admin_result {
            Ok(user) => user,
            Err(error) => {
                let _delete = repo.delete_by_id(org_tmp.id).await;
                return Err(RepositoryError::new(&error.to_string()));
            }
        };

        let org_updated = Organisation {
            id: org_tmp.id.clone(),
            name: org_tmp.name,
            active: false,
            admin_id: admin.id.clone(),
            created_at: org_tmp.created_at,
            deleted: false,
        };
        let update_org_result = repo.update(org_updated).await;

        match update_org_result {
            Ok(org) => Ok(org),
            Err(error) => {
                let _delete_user = UserUseCases::delete_user(repo_provider, admin.id).await;
                let _delete = repo.delete_by_id(org_tmp.id).await;
                Err(RepositoryError::new(&error.to_string()))
            }
        }
    }

    pub async fn delete_organisation(
        repo_provider: &impl RepoProvider,
        id: String,
    ) -> Result<Organisation, RepositoryError> {
        let repo = repo_provider.get_organisation_repo();
        let user_repo = repo_provider.get_user_repo();

        // TODO: check permissions
        let _delete_users = user_repo.delete_by_organisation_id(id.clone()).await;
        repo.delete_by_id(id).await
    }
}
