use chrono::{DateTime, Local};
use domain::share::value_object::{task_mode_config::ModeConfig, task_mode_type::ModeType};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "task_mode")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub uid: String,
    pub config: Option<ModeConfig>,
    pub mode_type: ModeType,
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
