use base::ddd::command::ICommand;
use chrono::{DateTime, Local};
use common::utils;
use domain::{
    aggregate::task::model::{task::Task, task_content::TaskContent, task_mode::TaskMode},
    share::value_object::task_date::TaskDate,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct TaskCreateCommand {
    #[validate(length(min = 5))]
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

impl ICommand for TaskCreateCommand {}

impl TaskCreateCommand {
    pub fn to_task(&self, created_by: String) -> Task {
        let mut t = Task {
            id: utils::generate_ulid(),
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
                id: self.task_mode_id.clone(),
            },
            task_content: TaskContent {
                id: "".to_string(),
                content: "".to_string(),
            },
        };
        t.new_task_content(self.task_content.clone());

        t
    }
}
