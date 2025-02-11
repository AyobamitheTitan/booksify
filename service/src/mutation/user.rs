pub mod user {
    use database::db::db::get_db;
    use entity::user::user;

    use schema::user::user::NewUser;
    use sea_orm::{ActiveModelTrait, Set};
    use uuid::Uuid;

    use crate::helper::password::password::hash_password;


    pub async fn create_user(payload: NewUser) -> user::ActiveModel {
        let new_user = user::ActiveModel {
            first_name: Set(payload.first_name),
            dob: Set(payload.dob),
            last_name: Set(payload.last_name),
            email: Set(payload.email),
            id: Set(Uuid::new_v4()),
            password: Set(hash_password(&payload.password)),
        };

        let db = get_db().await;
        
        let new_user: user::ActiveModel = new_user.insert(&db).await.expect("Unable to add to database").into();

        new_user
    }


}
