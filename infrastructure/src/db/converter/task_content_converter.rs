use domain::aggregate::task::model::task_content::TaskContent;

use super::super::model::preclude::*;

pub fn deserialize(task_content: TaskContentModel) -> TaskContent {
    TaskContent {
        id: task_content.id,
        content: task_content.content,
    }
}

pub fn serialize(u: TaskContent) -> TaskContentModel {
    TaskContentModel {
        created_at: None,
        updated_at: None,
        deleted_at: None,
        id: u.id,
        content: u.content,
    }
}
