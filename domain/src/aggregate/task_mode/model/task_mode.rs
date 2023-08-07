use base::ddd::aggregate::IAggregate;
use serde::Serialize;

use crate::share::value_object::{task_mode_config::ModeConfig, task_mode_type::ModeType};

#[derive(Debug, Clone, Serialize)]
pub struct TaskMode {
    pub id: String,
    pub uid: String,
    pub config: Option<ModeConfig>,
    pub mode_type: ModeType,
}

impl IAggregate for TaskMode {}

impl TaskMode {
    pub fn init_data(uid: String) -> Self {
        Self {
            id: common::utils::generate_ulid(),
            uid: uid,
            config: None,
            mode_type: ModeType::Normal,
        }
    }
}
