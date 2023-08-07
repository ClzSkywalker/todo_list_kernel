use super::super::model::preclude::*;
use domain::aggregate::preclude::*;

pub fn deserialize(t: TaskModeModel) -> TaskModeAggregate {
    TaskModeAggregate {
        id: t.id,
        uid: t.uid,
        config: t.config,
        mode_type: t.mode_type,
    }
}

pub fn serialize(t: TaskModeAggregate) -> TaskModeModel {
    TaskModeModel {
        id: t.id,
        created_at: None,
        updated_at: None,
        deleted_at: None,
        uid: t.uid,
        config: t.config,
        mode_type: t.mode_type,
    }
}
