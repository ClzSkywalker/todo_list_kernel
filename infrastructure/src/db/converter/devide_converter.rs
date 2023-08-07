use super::super::model::preclude::*;
use domain::aggregate::preclude::*;

pub fn deserialize(c: DevideModel, dms: Vec<TaskModel>) -> DevideAggregate {
    DevideAggregate {
        id: c.id,
        uid: c.uid,
        classify_id: c.classify_id,
        title: c.title,
        sort: c.sort,
        task_ids: dms.into_iter().map(|item| item.id).collect(),
    }
}

pub fn serialize(c: DevideAggregate) -> DevideModel {
    DevideModel {
        id: c.id,
        uid: c.uid,
        classify_id: c.classify_id,
        title: c.title,
        sort: c.sort,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    }
}
