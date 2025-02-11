pub mod review{
    use sea_orm::entity::prelude::*;

    use crate::{book::book, user::user};


    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name="review")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub book_id: Uuid,
        pub user_id: Uuid,
        pub rating: i32,
        pub comment: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation{
        User,
        Book
    }

    impl RelationTrait for Relation{
        fn def(&self) -> RelationDef {
            match self {
                Self::User => Entity::belongs_to(user::Entity)
                    .from(Column::UserId)
                    .to(user::Column::Id)
                    .into(),
                Self::Book => Entity::belongs_to(book::Entity)
                    .from(Column::BookId)
                    .to(book::Column::Id)
                    .into(),

            }
        }
    }

    impl Related<user::Entity> for Entity{
        fn to() -> RelationDef {
            Relation::User.def()
        }
    }

    impl Related<book::Entity> for Entity{
        fn to() -> RelationDef {
            Relation::Book.def()
        }
    }


    impl ActiveModelBehavior for ActiveModel{}
}