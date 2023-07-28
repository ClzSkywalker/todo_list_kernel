use chrono::{DateTime, Local};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "task_mode")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub oc: String,
    pub team_id: String,
    pub created_by: String,
    pub config: ModeConfig,
    pub mode_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, Default)]
pub struct ModeConfig {
    pub days: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ModeType {
    #[sea_orm(num_value = 0)]
    Normal,
    #[sea_orm(num_value = 1)]
    Day,
    #[sea_orm(num_value = 2)]
    Week,
    #[sea_orm(num_value = 3)]
    Month,
    #[sea_orm(num_value = 4)]
    Year,
    // 工作日(周一-周五)
    #[sea_orm(num_value = 5)]
    WorkDay,
    // 法定工作日
    #[sea_orm(num_value = 6)]
    LegalWorkingDay,
    // 法定节假日
    #[sea_orm(num_value = 7)]
    StatutoryHoliday,
    // 自定义
    #[sea_orm(num_value = 8)]
    Custome,
}

impl Default for ModeType {
    fn default() -> Self {
        ModeType::Normal
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Task,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Task => Entity::belongs_to(super::task::Entity)
                .from(Column::Oc)
                .to(super::task::Column::Oc)
                .into(),
        }
    }
}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
