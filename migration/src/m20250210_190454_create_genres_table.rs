use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Genre::Table)
                    .if_not_exists()
                    .col(pk_auto(Genre::Id))
                    .col(string(Genre::Name))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Genre::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Genre {
    Table,
    Id,
    Name,
}
