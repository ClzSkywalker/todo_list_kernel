use base::ddd::entity::IEntity;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Resource {
    pub id: String,
    pub exp: i64,
    pub gold_coin: i64,
    pub diamond: i64,
}

impl IEntity for Resource {}
