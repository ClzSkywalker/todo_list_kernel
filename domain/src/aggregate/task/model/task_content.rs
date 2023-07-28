use base::ddd::entity::IEntity;

#[derive(Debug, Clone)]
pub struct TaskContent {
    pub uuid: String,
    pub content: String,
}

impl IEntity for TaskContent {}
