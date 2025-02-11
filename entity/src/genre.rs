pub mod genre {

    use sea_orm::entity::prelude::*;

    use crate::{book::book, book_genre::book_genre};


    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "genre")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation{
        BookGenre
    }

    impl RelationTrait for Relation{
        fn def(&self) -> RelationDef {
            match self {
                Self::BookGenre => Entity::has_many(book_genre::Entity).into(),
            }
        }
    }

    impl Related<book::Entity> for Entity{
        fn to() -> RelationDef {
            book_genre::Relation::Book.def()
        }

        fn via() -> Option<RelationDef> {
            Some(book_genre::Relation::Genre.def().rev())
        }
    }

    

    impl ActiveModelBehavior for ActiveModel{}
}
