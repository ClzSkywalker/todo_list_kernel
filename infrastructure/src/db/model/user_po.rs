use chrono::{DateTime, Local};
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Default)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
    pub team_id_port: String,
    pub nick_name: String,
    pub member_type: MemberType,
    pub register_type: RegisterType,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub pwd: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum MemberType {
    #[sea_orm(num_value = 0)]
    Normal,
    #[sea_orm(num_value = 1)]
    Member,
    #[sea_orm(num_value = 2)]
    Permanent,
}

impl Default for MemberType {
    fn default() -> Self {
        MemberType::Normal
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum RegisterType {
    #[sea_orm(num_value = 0)]
    Uid,
    #[sea_orm(num_value = 1)]
    Email,
    #[sea_orm(num_value = 2)]
    Phone,
}

impl Default for RegisterType {
    fn default() -> Self {
        RegisterType::Uid
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Team,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Team => Entity::has_many(super::team_po::Entity).into(),
        }
    }
}

impl Related<super::team_po::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    // async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    // where
    //     C: ConnectionTrait,
    // {
    //     self.created_at=Set(DateTime::default());
    //     Ok(self)
    // }
}
