use crate::{
    entities::{
        organisation::{NewOrganisationDTO, Organisation},
        user::NewUserDTO,
    },
    errors::RepositoryError,
    interfaces::{OrganisationRepository, UserRepository},
};

pub struct OrganisationUseCases {}

impl OrganisationUseCases {
    pub async fn create_organisation(
        repo: impl OrganisationRepository,
        user_repo: impl UserRepository,
        dto: NewOrganisationDTO,
    ) -> Result<Organisation, RepositoryError> {
        let create_org_result = repo.create(dto.name, "tmp_admin_id".to_string()).await;
        let org_tmp = match create_org_result {
            Ok(org) => org,
            Err(error) => return Err(RepositoryError::new(&error.to_string())),
        };

        let admin_dto = NewUserDTO {
            email: dto.admin_email,
            name: dto.admin_name,
            organisation_id: org_tmp.id.to_string(),
        };
        let create_admin_result = user_repo.create(admin_dto).await;
        let admin = match create_admin_result {
            Ok(user) => user,
            Err(error) => {
                let _delete = repo.delete_by_id(org_tmp.id).await;
                return Err(RepositoryError::new(&error.to_string()));
            }
        };

        // TODO: active org only after admin email verification
        let org_updated = Organisation {
            id: org_tmp.id.clone(),
            name: org_tmp.name,
            active: true,
            admin_id: admin.id.clone(),
            deleted: false,
        };
        let update_org_result = repo.update(org_updated).await;
        match update_org_result {
            Ok(org) => Ok(org),
            Err(error) => {
                let _delete = repo.delete_by_id(org_tmp.id).await;
                let _delete_user = user_repo.delete_by_id(admin.id).await;
                Err(RepositoryError::new(&error.to_string()))
            }
        }
    }

    pub async fn delete_organisation(
        repo: impl OrganisationRepository,
        user_repo: impl UserRepository,
        id: String,
    ) -> Result<Organisation, RepositoryError> {
        // TODO: check permissions
        let _delete_users = user_repo.delete_by_organisation_id(id.clone()).await;
        let result = repo.delete_by_id(id).await;

        result
    }
}
