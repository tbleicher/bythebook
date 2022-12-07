use domain::entities::organisation::Organisation;
use sea_orm::{entity::prelude::*, DeleteMany, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "organisations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub active: bool,
    pub admin_id: String,
    pub deleted: bool,
    pub created_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: String) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn delete_by_id(id: String) -> DeleteMany<Entity> {
        // only delete entries that are already marked as 'deleted'
        Self::delete_many()
            .filter(Column::Id.eq(id))
            .filter(Column::Deleted.eq(true))
    }
}

impl From<Organisation> for ActiveModel {
    fn from(org: Organisation) -> Self {
        ActiveModel {
            active: Set(org.active),
            admin_id: Set(org.admin_id),
            created_at: Set(org.created_at),
            deleted: Set(org.deleted),
            id: Set(org.id),
            name: Set(org.name),
        }
    }
}

impl Into<Organisation> for Model {
    fn into(self) -> Organisation {
        Organisation {
            id: self.id,
            name: self.name,
            active: self.active,
            admin_id: self.admin_id,
            created_at: self.created_at,
            deleted: self.deleted,
        }
    }
}
