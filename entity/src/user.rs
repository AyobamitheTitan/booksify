pub mod user {

    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};
    

    use crate::review::review;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "user")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub dob: Date,
        pub password: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation{
        Review,
    }

    impl RelationTrait for Relation{
        fn def(&self) -> RelationDef {
            match self {
                Self::Review => Entity::has_many(review::Entity).into(),
            }
        }
    }

    impl Related<review::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Review.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel{}
}
