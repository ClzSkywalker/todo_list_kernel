use super::super::model::preclude::*;
use domain::aggregate::preclude::*;

pub fn deserialize(c: ClassifyModel, dms: Vec<DevideModel>) -> ClassifyAggregate {
    ClassifyAggregate {
        id: c.id,
        uid: c.uid,
        team_id: c.team_id,
        title: c.title,
        color: c.color,
        show_type: c.show_type,
        order_type: c.order_type,
        sort: c.sort,
        parent_id: c.parent_id,
        devide_ids: dms.iter().map(|item| item.id.clone()).collect(),
    }
}

pub fn serialize(c: ClassifyAggregate) -> ClassifyModel {
    ClassifyModel {
        id: c.id,
        uid: c.uid,
        team_id: c.team_id,
        title: c.title,
        color: c.color,
        show_type: c.show_type,
        order_type: c.order_type,
        sort: c.sort,
        parent_id: c.parent_id,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    }
}
