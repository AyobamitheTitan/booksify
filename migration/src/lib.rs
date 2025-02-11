pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250210_184816_create_books_table;
mod m20250210_190454_create_genres_table;
mod m20250210_190658_create_book_genres_table;
mod m20250210_192401_create_user_table;
mod m20250210_192612_create_review_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250210_184816_create_books_table::Migration),
            Box::new(m20250210_190454_create_genres_table::Migration),
            Box::new(m20250210_190658_create_book_genres_table::Migration),
            Box::new(m20250210_192401_create_user_table::Migration),
            Box::new(m20250210_192612_create_review_table::Migration),
        ]
    }
}
