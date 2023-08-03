use base::ddd::entity::IEntity;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TaskMode {
    pub id: String,
    // pub config: Option<ModeConfig>,
    // pub mode_type: ModeType,
}

impl IEntity for TaskMode {}
