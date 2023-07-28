use base::ddd::value_object::IValueObject;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, Default)]
pub struct ModeConfig {
    pub days: Vec<u32>,
}

impl IValueObject for ModeConfig {}
