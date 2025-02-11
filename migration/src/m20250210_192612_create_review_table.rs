use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250210_184816_create_books_table::Book, m20250210_192401_create_user_table::User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Review::Table)
                    .if_not_exists()
                    .col(pk_uuid(Review::Id))
                    .col(uuid(Review::UserId).not_null())
                    .col(uuid(Review::BookId).not_null())
                    .col(integer(Review::Rating).not_null())
                    .col(string(Review::Comment).not_null())
                    .foreign_key(ForeignKey::create()
                        .from(Review::Table, Review::BookId)
                        .to(Book::Table, Book::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(ForeignKey::create()
                        .from(Review::Table, Review::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Review {
    Table,
    Id,
    BookId,
    UserId,
    Rating,
    Comment,
}
