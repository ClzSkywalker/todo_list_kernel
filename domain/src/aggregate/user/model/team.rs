use base::ddd::entity::IEntity;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Team {
    pub id: String,
    pub name: String,
}

impl IEntity for Team {}
