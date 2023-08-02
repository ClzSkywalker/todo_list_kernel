use chrono::{DateTime, Local};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "devide")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub classify_id: String,
    pub uid: String,
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
            Relation::Task => Entity::has_many(super::task_po::Entity).into(),
            Relation::Classify => Entity::belongs_to(super::classify_po::Entity)
                .from(Column::ClassifyId)
                .to(super::classify_po::Column::Id)
                .into(),
        }
    }
}

impl Related<super::task_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl Related<super::classify_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classify.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
