use sea_orm::{entity::prelude::*, DeleteMany};
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
