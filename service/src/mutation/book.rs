pub mod book {
    use database::db::db::get_db;
    use entity::book::book;
    use schema::book::book::NewBook;
    use sea_orm::ActiveModelTrait;
    use sea_orm::Set;
    use uuid::Uuid;

    pub async fn add_book(payload: NewBook) -> Result<entity::book::book::Model, sea_orm::DbErr> {
        let new_book = book::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(payload.title),
            author: Set(payload.author),
            published_year: Set(payload.published_year),
        };

        let db = get_db().await;

        let new_book = new_book.insert(&db).await;

        match new_book {
            Ok(book) => Ok(book),
            Err(err) => Err(err)
        }
    }

    pub async fn delete_book(){

    }
}
