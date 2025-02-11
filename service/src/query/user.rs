pub mod user{

    use database::db::db::get_db;
    use entity::user::user::{self, Entity as User, Model};
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

    pub async fn get_user(email: &str) -> Option<Model> {
        let db = get_db().await;

        let retrieved_user = User::find()
            .filter(user::Column::Email.eq(email))
            .one(&db)
            .await
            .unwrap();

        retrieved_user
    }
}