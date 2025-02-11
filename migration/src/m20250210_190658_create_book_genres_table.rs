use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250210_184816_create_books_table::Book, m20250210_190454_create_genres_table::Genre,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(BookGenre::Table)
                    .if_not_exists()
                    .col(uuid(BookGenre::BookId).not_null())
                    .col(integer(BookGenre::GenreId).not_null())
                    .foreign_key(ForeignKey::create()
                            .from(BookGenre::Table, BookGenre::BookId)
                            .to(Book::Table, Book::Id)
                            .on_delete(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create()
                            .from(BookGenre::Table, BookGenre::GenreId)
                            .to(Genre::Table, Genre::Id)
                            .on_delete(ForeignKeyAction::Cascade))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(BookGenre::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BookGenre {
    Table,
    BookId,
    GenreId,
}
