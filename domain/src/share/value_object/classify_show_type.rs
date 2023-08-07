use sea_orm::{EnumIter, DeriveActiveEnum};
use serde::Serialize;


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
