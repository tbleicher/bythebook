use sea_orm_migration::prelude::*;

use super::m20221117_000003_create_projects_table::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Notes::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Notes::Title).string().not_null())
                    .col(ColumnDef::new(Notes::Text).string().not_null())
                    .col(ColumnDef::new(Projects::OrganisationId).string().not_null())
                    .col(ColumnDef::new(Notes::ProjectId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-note-project_id")
                            .from(Notes::Table, Notes::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notes::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Notes {
    Table,
    Id,
    Title,
    Text,
    ProjectId,
}
