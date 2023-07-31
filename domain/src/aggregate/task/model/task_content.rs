use base::ddd::entity::IEntity;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TaskContent {
    pub uuid: String,
    pub content: String,
}

impl IEntity for TaskContent {}
