use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum MemberType {
    #[sea_orm(num_value = 0)]
    Normal = 0,
    #[sea_orm(num_value = 1)]
    Member = 1,
    #[sea_orm(num_value = 2)]
    Permanent = 2,
}

impl Default for MemberType {
    fn default() -> Self {
        MemberType::Normal
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum RegisterType {
    #[sea_orm(num_value = 0)]
    Uid = 0,
    #[sea_orm(num_value = 1)]
    Email = 1,
    #[sea_orm(num_value = 2)]
    Phone = 2,
}

impl Default for RegisterType {
    fn default() -> Self {
        RegisterType::Uid
    }
}
