use sea_orm_migration::{prelude::*, schema::{integer, pk_uuid, string}};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                .table(Book::Table)
                .if_not_exists()
                .col(pk_uuid(Book::Id))
                .col(string(Book::Title).not_null())
                .col(string(Book::Author).not_null())
                .col(integer(Book::PublishedYear).not_null())
                .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
        .drop_table(Table::drop().table(Book::Table).to_owned())
        .await
    }
}

#[derive(DeriveIden)]
pub enum Book {
    Table,
    Id,
    Title,
    Author,
    PublishedYear
}
