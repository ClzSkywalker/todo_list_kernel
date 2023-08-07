use sea_orm::{EnumIter, DeriveActiveEnum};
use serde::Serialize;

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