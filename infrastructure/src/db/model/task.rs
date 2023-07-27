use chrono::{DateTime, Local};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub oc: String,
    pub created_by: String,
    pub devide_id: String,
    pub content_id: String,
    pub task_mode_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTime<Local>>,
    pub give_up_at: Option<DateTime<Local>>,
    pub start_at: Option<DateTime<Local>>,
    pub end_at: Option<DateTime<Local>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Devide,
    TaskContent,
    TaskMode,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::TaskContent => Entity::has_many(super::task_content::Entity).into(),
            Relation::TaskMode => Entity::has_many(super::task_mode::Entity).into(),
            Relation::Devide => Entity::belongs_to(super::devide::Entity)
                .from(Column::DevideId)
                .to(super::devide::Column::Oc)
                .into(),
        }
    }
}

impl Related<super::devide::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Devide.def()
    }
}

impl Related<super::task_content::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskContent.def()
    }
}

impl Related<super::task_mode::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskMode.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
