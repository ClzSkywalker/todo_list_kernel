use chrono::{DateTime, Local};
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default)]
#[sea_orm(table_name = "classify")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub uid: String,
    pub team_id: String,
    pub title: String,
    pub color: String,
    pub show_type: ShowType,
    pub order_type: OrderType,
    pub sort: u32,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ShowType {
    #[sea_orm(num_value = 0)]
    Normal,
    #[sea_orm(num_value = 1)]
    Simple,
}

impl Default for ShowType {
    fn default() -> Self {
        ShowType::Normal
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum OrderType {
    // 默认，创建时间排序
    #[sea_orm(num_value = 0)]
    Default,
    // 分组排序,创建时间
    #[sea_orm(num_value = 1)]
    Group,
    // 截止时间排序
    #[sea_orm(num_value = 2)]
    EndTime,
    // 重要程度排序，创建时间
    #[sea_orm(num_value = 3)]
    Important,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Default
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Team,
    Classify,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Classify => Entity::has_many(super::devide_po::Entity).into(),
            Relation::Team => Entity::belongs_to(super::team_po::Entity)
                .from(Column::TeamId)
                .to(super::team_po::Column::Id)
                .into(),
        }
    }
}

impl Related<super::devide_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Classify.def()
    }
}

impl Related<super::team_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
