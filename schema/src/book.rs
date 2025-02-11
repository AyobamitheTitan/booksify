pub mod book{
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;


    #[derive(Debug, Deserialize, Serialize)]
    pub struct NewBook{
        pub title: String,
        pub author: String,
        pub published_year: i32
    }
}