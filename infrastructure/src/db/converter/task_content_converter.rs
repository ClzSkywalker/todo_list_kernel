use domain::aggregate::task::model::task_content::TaskContent;

use super::super::model::preclude::*;

pub fn deserialize(task_content: TaskContentModel) -> TaskContent {
    TaskContent {
        uuid: task_content.uuid,
        content: task_content.content,
    }
}

pub fn serialize(u: TaskContent) -> TaskContentModel {
    TaskContentModel {
        id: 0,
        created_at: None,
        updated_at: None,
        deleted_at: None,
        uuid: u.uuid,
        content: u.content,
    }
}
