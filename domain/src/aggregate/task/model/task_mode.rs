use base::ddd::entity::IEntity;

#[derive(Debug, Clone)]
pub struct TaskMode {
    pub uuid: String,
    // pub config: Option<ModeConfig>,
    // pub mode_type: ModeType,
}

impl IEntity for TaskMode {}
