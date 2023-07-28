use sea_orm::{EnumIter, DeriveActiveEnum};
use serde::Serialize;

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
