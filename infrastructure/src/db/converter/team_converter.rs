use super::super::model::preclude::*;
use domain::aggregate::preclude::*;

pub fn deserialize(t: TeamModel) -> TeamAggregate {
    TeamAggregate {
        id: t.id,
        uid: t.uid,
        name: t.name,
        description: t.description,
    }
}

pub fn serialize(t: TeamAggregate) -> TeamModel {
    TeamModel {
        id: t.id,
        uid: t.uid,
        name: t.name,
        description: t.description,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    }
}
