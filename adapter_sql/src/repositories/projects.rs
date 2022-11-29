use crate::models::{project, project::Model as ProjectModel};
use domain::async_trait::async_trait;
use domain::entities::project::{NewProjectDTO, Project};
use domain::errors::RepositoryError;
use domain::interfaces::ProjectRepository;
use nanoid::nanoid;
use sea_orm::*;

fn convert_to_entity(model: project::Model) -> Project {
    Project {
        id: model.id.to_string(),
        title: model.title.to_string(),
        description: model.description.to_string(),
        organisation_id: model.organisation_id.to_string(),
    }
}
pub struct ProjectRepositorySql<'a> {
    pub db: &'a DbConn,
}

#[async_trait]
impl ProjectRepository for ProjectRepositorySql<'_> {
    async fn create(&self, dto: NewProjectDTO) -> Result<Project, RepositoryError> {
        let new_project = project::ActiveModel {
            id: Set(nanoid!(10, &nanoid::alphabet::SAFE)),
            title: Set(dto.title.to_owned()),
            description: Set(dto.description.to_owned()),
            organisation_id: Set(dto.organisation_id.to_owned()),
        };

        let record_option = new_project.insert(self.db).await;

        match record_option {
            Ok(model) => Ok(convert_to_entity(model)),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn delete_by_id(&self, id: String) -> Result<Project, RepositoryError> {
        let db_result = project::Entity::find_by_id(id).one(self.db).await;

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
                None => Err(RepositoryError::new("Project not found.")),
            },
        }
    }

    async fn find_one_by_id(&self, id: String) -> Result<Option<Project>, RepositoryError> {
        let project = project::Entity::find_by_id(id).one(self.db).await;

        match project {
            Ok(model_or_none) => match model_or_none {
                Some(model) => Ok(Some(convert_to_entity(model))),
                None => Ok(None),
            },
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn list(&self) -> Result<Vec<Project>, RepositoryError> {
        let projects_list = project::Entity::find().all(self.db).await;

        match projects_list {
            Ok(models) => Ok(models
                .iter()
                .map(|model| convert_to_entity(model.clone()))
                .collect()),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }
}

impl ProjectRepositorySql<'_> {
    pub async fn find_project_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<Option<Project>, RepositoryError> {
        let project = project::Entity::find_by_id(id).one(db).await;

        match project {
            Ok(model_or_none) => match model_or_none {
                Some(model) => {
                    let proj = Project {
                        id: model.id.to_string(),
                        title: model.title.to_string(),
                        description: model.description.to_string(),
                        organisation_id: model.organisation_id.to_string(),
                    };
                    Ok(Some(proj))
                }
                None => Ok(None),
            },
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    pub async fn get_all_projects(db: &DbConn) -> Result<Vec<ProjectModel>, DbErr> {
        project::Entity::find().all(db).await
    }

    /// If ok, returns (note models, num pages).
    pub async fn find_projects_in_page(
        db: &DbConn,
        page: u64,
        notes_per_page: u64,
    ) -> Result<(Vec<ProjectModel>, u64), DbErr> {
        // Setup paginator
        let paginator = project::Entity::find()
            .order_by_asc(project::Column::Id)
            .paginate(db, notes_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated notes
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create_project(db: &DbConn, dto: NewProjectDTO) -> Result<ProjectModel, DbErr> {
        let new_project = project::ActiveModel {
            id: Set(nanoid!(10, &nanoid::alphabet::SAFE)),
            title: Set(dto.title.to_owned()),
            description: Set(dto.description.to_owned()),
            organisation_id: Set(dto.organisation_id.to_owned()),
        };

        Ok(new_project.insert(db).await?)
    }

    pub async fn update_project_by_id(
        db: &DbConn,
        id: String,
        form_data: ProjectModel,
    ) -> Result<ProjectModel, DbErr> {
        let note: project::ActiveModel = project::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find note.".to_owned()))
            .map(Into::into)?;

        project::ActiveModel {
            id: note.id,
            title: Set(form_data.title.to_owned()),
            description: Set(form_data.description.to_owned()),
            organisation_id: Set(form_data.organisation_id.to_owned()),
            // project_id: Set(form_data.project_id.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_project(db: &DbConn, id: String) -> Result<DeleteResult, DbErr> {
        let note: project::ActiveModel = project::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find note.".to_owned()))
            .map(Into::into)?;

        note.delete(db).await
    }

    pub async fn delete_all_notes(db: &DbConn) -> Result<DeleteResult, DbErr> {
        project::Entity::delete_many().exec(db).await
    }
}
