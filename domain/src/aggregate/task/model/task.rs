use base::ddd::aggregate::IAggregate;
use serde::Serialize;

use crate::share::value_object::task_date::TaskDate;

use super::{task_content::TaskContent, task_mode::TaskMode};

#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub uuid: String,
    pub uid: String,
    pub devide_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub task_date: Option<TaskDate>,
    pub task_mode: TaskMode,
    pub task_content: TaskContent,
}

impl IAggregate for Task {}
