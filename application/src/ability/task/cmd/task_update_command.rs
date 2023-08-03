use base::ddd::command::ICommand;
use chrono::{DateTime, Local};
use domain::{
    aggregate::task::model::{task::Task, task_content::TaskContent, task_mode::TaskMode},
    share::value_object::task_date::TaskDate,
};
use serde::Deserialize;

use super::task_create_command::TaskCreateCommand;

#[derive(Debug, Clone, Deserialize)]
pub struct TaskUpdateCommand {
    pub id: String,
    pub title: String,
    pub task_content: String,
    pub task_mode_id: String,
    pub parent_id: Option<String>,
    pub devide_id: String,
    pub completed_at: Option<DateTime<Local>>,
    pub give_up_at: Option<DateTime<Local>>,
    pub start_at: Option<DateTime<Local>>,
    pub end_at: Option<DateTime<Local>>,
}

impl ICommand for TaskUpdateCommand {}

impl TaskUpdateCommand {
    pub fn to_task(&self, created_by: String, task_content_id: String) -> Task {
        Task {
            uuid: self.id.clone(),
            uid: created_by,
            devide_id: self.devide_id.clone(),
            parent_id: self.parent_id.clone(),
            title: self.title.clone(),
            task_date: Some(TaskDate {
                completed_at: self.completed_at,
                give_up_at: self.give_up_at,
                start_at: self.start_at,
                end_at: self.end_at,
            }),
            task_mode: TaskMode {
                uuid: self.task_mode_id.clone(),
            },
            task_content: TaskContent {
                id: task_content_id,
                content: self.task_content.clone(),
            },
        }
    }

    pub fn from_task_create(id: String, t: TaskCreateCommand) -> Self {
        TaskUpdateCommand {
            id: id,
            title: t.title,
            task_content: t.task_content,
            task_mode_id: t.task_mode_id,
            parent_id: t.parent_id,
            devide_id: t.devide_id,
            completed_at: t.completed_at,
            give_up_at: t.give_up_at,
            start_at: t.start_at,
            end_at: t.end_at,
        }
    }
}
