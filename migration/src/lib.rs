pub use sea_orm_migration::prelude::*;

mod m20221117_000001_create_organisations_table;
mod m20221117_000002_create_users_table;
mod m20221117_000003_create_projects_table;
mod m20221117_000004_create_notes_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221117_000001_create_organisations_table::Migration),
            Box::new(m20221117_000003_create_projects_table::Migration),
            Box::new(m20221117_000004_create_notes_table::Migration),
            Box::new(m20221117_000002_create_users_table::Migration),
        ]
    }
}
