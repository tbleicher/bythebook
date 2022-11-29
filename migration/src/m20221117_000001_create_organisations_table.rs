use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organisations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Organisations::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Organisations::Name).string().not_null())
                    .col(ColumnDef::new(Organisations::AdminId).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Organisations::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Organisations {
    Table,
    Id,
    AdminId,
    Name,
}
