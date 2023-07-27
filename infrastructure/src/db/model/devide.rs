use chrono::{DateTime, Local};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "devide")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub oc: String,
    pub classify_id: String,
    pub created_by: String,
    pub title: String,
    pub sort: u32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Classify,
    Task,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Task => Entity::has_many(super::task::Entity).into(),
            Relation::Classify => Entity::belongs_to(super::classify::Entity)
                .from(Column::ClassifyId)
                .to(super::classify::Column::Oc)
                .into(),
        }
    }
}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl Related<super::classify::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classify.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
