pub mod user{
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use uuid::Uuid;

    #[derive(Debug, Deserialize, Serialize, ToSchema)]
    pub struct NewUser{
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub dob: NaiveDate,
        pub password: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Login{
        pub email: String,
        pub password: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DisplayUser{
        pub id: Uuid,
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub dob: NaiveDate,
    }
}