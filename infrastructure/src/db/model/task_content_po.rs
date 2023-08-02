use chrono::{DateTime, Local};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "task_content")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(created_at)]
    pub created_at: Option<DateTime<Local>>,
    #[sea_orm(updated_at)]
    pub updated_at: Option<DateTime<Local>>,
    #[sea_orm(deleted_at)]
    pub deleted_at: Option<DateTime<Local>>,
    pub content: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Task,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Task => Entity::belongs_to(super::task_po::Entity)
                .from(Column::Id)
                .to(super::task_po::Column::Id)
                .into(),
        }
    }
}

impl Related<super::task_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// impl PrimaryKeyTrait for PrimaryKey {
//     type ValueType = (i32, i32);

//     fn auto_increment() -> bool {
//         false
//     }
// }
