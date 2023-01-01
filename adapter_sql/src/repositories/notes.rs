use crate::models::note;
use domain::async_trait::async_trait;
use domain::entities::note::{NewNoteDTO, Note};
use domain::errors::RepositoryError;
use domain::interfaces::NoteRepository;
use nanoid::nanoid;
use sea_orm::*;

fn convert_to_entity(model: note::Model) -> Note {
    Note {
        id: model.id.to_string(),
        title: model.title.to_string(),
        text: model.text.to_string(),
        project_id: model.project_id,
    }
}

pub struct NoteRepositorySql<'a> {
    pub db: &'a DbConn,
}

#[async_trait]
impl NoteRepository for NoteRepositorySql<'_> {
    async fn create(&self, dto: NewNoteDTO) -> Result<Note, RepositoryError> {
        let new_note = note::ActiveModel {
            id: Set(nanoid!(10, &nanoid::alphabet::SAFE)),
            title: Set(dto.title.to_owned()),
            text: Set(dto.text.to_owned()),
            project_id: Set(dto.project_id.to_owned()),
        };

        let insert_result = new_note.insert(self.db).await;

        match insert_result {
            Ok(model) => Ok(convert_to_entity(model)),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn delete_by_id(&self, id: String) -> Result<Note, RepositoryError> {
        let db_result = note::Entity::find_by_id(id).one(self.db).await;

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
                None => Err(RepositoryError::new("Note not found.")),
            },
        }
    }

    async fn find_by_project_id(&self, project_id: String) -> Result<Vec<Note>, RepositoryError> {
        let notes_list = note::Entity::find()
            .filter(note::Column::ProjectId.eq(project_id))
            .all(self.db)
            .await;

        match notes_list {
            Ok(models) => Ok(models
                .iter()
                .map(|model| convert_to_entity(model.clone()))
                .collect()),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn find_one_by_id(&self, id: String) -> Result<Option<Note>, RepositoryError> {
        let record = note::Entity::find_by_id(id).one(self.db).await;

        match record {
            Ok(model_or_none) => match model_or_none {
                Some(model) => Ok(Some(convert_to_entity(model))),

                None => Ok(None),
            },
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }

    async fn list(&self) -> Result<Vec<Note>, RepositoryError> {
        let notes_list = note::Entity::find().all(self.db).await;

        match notes_list {
            Ok(models) => Ok(models
                .iter()
                .map(|model| convert_to_entity(model.clone()))
                .collect()),
            Err(error) => Err(RepositoryError::new(&format!("DB error: {:?}", error))),
        }
    }
}

pub struct UnusedFunctions {}

impl UnusedFunctions {
    pub async fn find_note_by_id(db: &DbConn, id: String) -> Result<Option<note::Model>, DbErr> {
        note::Entity::find_by_id(id).one(db).await
    }

    pub async fn get_all_notes(db: &DbConn) -> Result<Vec<note::Model>, DbErr> {
        note::Entity::find().all(db).await
    }

    pub async fn find_notes_by_project_id(
        db: &DbConn,
        project_id: String,
    ) -> Result<Vec<note::Model>, DbErr> {
        note::Entity::find()
            .filter(note::Column::ProjectId.eq(project_id))
            .all(db)
            .await
    }

    /// If ok, returns (note models, num pages).
    pub async fn find_notes_in_page(
        db: &DbConn,
        page: u64,
        notes_per_page: u64,
    ) -> Result<(Vec<note::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = note::Entity::find()
            .order_by_asc(note::Column::Id)
            .paginate(db, notes_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated notes
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create_note(db: &DbConn, form_data: NewNoteDTO) -> Result<note::Model, DbErr> {
        let new_note = note::ActiveModel {
            id: Set(nanoid!(10, &nanoid::alphabet::SAFE)),
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            project_id: Set(form_data.project_id.to_owned()),
        };

        new_note.insert(db).await
    }

    pub async fn update_note_by_id(
        db: &DbConn,
        id: String,
        form_data: note::Model,
    ) -> Result<note::Model, DbErr> {
        let note: note::ActiveModel = note::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("Cannot find note.".to_owned()))
            .map(Into::into)?;

        note::ActiveModel {
            id: note.id,
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            project_id: Set(form_data.project_id.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_note(db: &DbConn, id: String) -> Result<DeleteResult, DbErr> {
        let note: note::ActiveModel = note::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("Cannot find note.".to_owned()))
            .map(Into::into)?;

        note.delete(db).await
    }

    pub async fn delete_all_notes(db: &DbConn) -> Result<DeleteResult, DbErr> {
        note::Entity::delete_many().exec(db).await
    }
}
