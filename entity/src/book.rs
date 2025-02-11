pub mod book {

    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::{book_genre::book_genre, genre::genre, review::review};


    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "book")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub title: String,
        pub author: String,
        pub published_year: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        BookGenre,
        Review
    }

    impl RelationTrait for Relation{
        fn def(&self) -> RelationDef {
            match self {
                Self::BookGenre => Entity::has_many(book_genre::Entity).into(),
                Self::Review => Entity::has_many(review::Entity).into(),
            }
        }
    }

    impl Related<genre::Entity> for Entity{
        fn to() -> RelationDef {
            book_genre::Relation::Genre.def()
        }

        fn via() -> Option<RelationDef> {
            Some(book_genre::Relation::Book.def().rev())
        }
    }

    impl ActiveModelBehavior for ActiveModel{}
}
