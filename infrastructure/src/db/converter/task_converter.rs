use domain::{
    aggregate::task::model::{task::Task, task_content::TaskContent, task_mode::TaskMode},
    share::value_object::task_date::TaskDate,
};

use super::super::model::preclude::*;

pub fn deserialize(
    task: TaskModel,
    task_content: TaskContentModel,
) -> Task {
    Task {
        uuid: task.uuid,
        created_by: task.created_by,
        devide_id: task.devide_id,
        parent_id: task.parent_id,
        title: task.title,
        task_date: Some(TaskDate {
            completed_at: task.completed_at,
            give_up_at: task.give_up_at,
            start_at: task.start_at,
            end_at: task.end_at,
        }),
        task_mode: TaskMode {
            uuid: task.task_mode_id,
        },
        task_content: TaskContent {
            uuid: task_content.uuid,
            content: task_content.content,
        },
    }
}

pub fn serialize(u: Task) -> TaskModel {
    let td = u.task_date.clone().unwrap_or_default();
    TaskModel {
        id: 0,
        created_at: None,
        updated_at: None,
        deleted_at: None,
        uuid: u.uuid,
        created_by: u.created_by,
        devide_id: u.devide_id,
        content_id: u.task_content.uuid,
        task_mode_id: u.task_mode.uuid,
        parent_id: u.parent_id,
        title: u.title,
        completed_at: td.completed_at,
        give_up_at: td.give_up_at,
        start_at: td.start_at,
        end_at: td.end_at,
    }
}
