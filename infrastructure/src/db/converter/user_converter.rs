use super::super::model::preclude::*;
use domain::aggregate::preclude::*;

pub fn deserialize(u: UserModel, tl: Vec<TeamModel>) -> UserAggregate {
    UserAggregate {
        id: u.id,
        nick_name: u.nick_name,
        member_type: u.member_type,
        register_type: u.register_type,
        team_id_port: u.team_id_port,
        picture: u.picture,
        email: u.email,
        phone: u.phone,
        pwd: u.pwd,
        team_list: tl
            .iter()
            .map(|item| TeamDomainEntity {
                id: item.id.clone(),
                name: item.name.clone(),
            })
            .collect::<Vec<TeamDomainEntity>>(),
        version: u.version,
    }
}

pub fn serialize(u: UserAggregate) -> UserModel {
    UserModel {
        created_at: None,
        updated_at: None,
        deleted_at: None,
        id: u.id,
        nick_name: u.nick_name,
        member_type: u.member_type,
        register_type: u.register_type,
        team_id_port: u.team_id_port,
        picture: u.picture,
        email: u.email,
        phone: u.phone,
        pwd: u.pwd,
        version: u.version,
    }
}
