use std::sync::Arc;

use base::ddd::aggregate::IAggregate;
use common::{contextx::AppContext, utils};
use serde::Serialize;

use crate::share::value_object::task_date::TaskDate;

use super::{task_content::TaskContent, task_mode::TaskMode};

#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub id: String,
    pub uid: String,
    pub devide_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub task_date: Option<TaskDate>,
    pub task_mode: TaskMode,
    pub task_content: TaskContent,
}

impl IAggregate for Task {}

impl Task {
    pub fn init_data(ctx: Arc<AppContext>, uid: String, did: String, tmid: String) -> Self {
        let title = match ctx.locale {
            common::i18n::Locale::En => String::from("First step"),
            common::i18n::Locale::Zh => String::from("第一步"),
        };
        Self {
            id: common::utils::generate_ulid(),
            uid: uid,
            devide_id: did,
            parent_id: None,
            title: title,
            task_date: None,
            task_mode: TaskMode { id: tmid },
            task_content: TaskContent {
                id: "".to_string(),
                content: "".to_string(),
            },
        }
    }

    pub fn new_task_content(&mut self, content: String) {
        let tc_ulid = utils::generate_ulid();
        let tc = TaskContent {
            id: tc_ulid.clone(),
            content: content,
        };
        self.task_content = tc;
    }
}
