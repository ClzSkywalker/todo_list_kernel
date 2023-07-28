use chrono::{DateTime, Local};
use domain::share::value_object::{task_mode_config::ModeConfig, task_mode_type::ModeType};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "task_mode")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub uuid: String,
    pub created_by: String,
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
                .from(Column::Uuid)
                .to(super::task_po::Column::Uuid)
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
