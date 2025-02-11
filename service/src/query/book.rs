pub mod book{
    use database::db::db::get_db;
    use sea_orm::EntityTrait;
    use uuid::Uuid;
    use entity::book::book::Entity as Book;

    pub async fn find_book(book_id: Uuid) -> Result<Option<entity::book::book::Model>, sea_orm::DbErr> {
        let db = get_db().await;

        let book = Book::find_by_id(book_id).one(&db).await;
        book
    }      

    pub async fn get_books() -> Result<Vec<entity::book::book::Model>, sea_orm::DbErr> {
        let db = get_db().await;

        let book = Book::find().all(&db).await;

        book
    } 
}