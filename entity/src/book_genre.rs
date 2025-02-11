pub mod book_genre{
    use sea_orm::entity::prelude::*;

    use crate::{book::book, genre::genre};
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "book_genre")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub book_id: Uuid,
        pub genre_id: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation{
        Book,
        Genre
    }

    impl Related<book::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Book.def()
        }
    }

    impl Related<genre::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Genre.def()
        }
    }


    impl RelationTrait for Relation{
        fn def(&self) -> RelationDef {
            match self {
                Self::Book => Entity::belongs_to(book::Entity)
                    .from(Column::BookId)
                    .to(book::Column::Id)
                    .into(),
                Self::Genre => Entity::belongs_to(genre::Entity)
                    .from(Column::GenreId)
                    .to(genre::Column::Id)
                    .into(),    
            }
        }
    }
    
    impl ActiveModelBehavior for ActiveModel{}
}