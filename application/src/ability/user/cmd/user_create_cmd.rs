use base::ddd::command::ICommand;
use common::{config::VERSION, utils};
use domain::{
    aggregate::preclude::*,
    share::value_object::user_type::{MemberType, RegisterType},
};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UserCreateCmd {
    pub nick_name: String,
    pub member_type: MemberType,
    pub register_type: RegisterType,
    pub team_id_port: String,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub pwd: String,
    pub version: String,
}

impl ICommand for UserCreateCmd {}

impl UserCreateCmd {
    pub fn to_ag(&self, tl: Vec<TeamDomainEntity>) -> UserAggregate {
        UserAggregate {
            id: utils::generate_ulid(),
            nick_name: self.nick_name.clone(),
            member_type: self.member_type.clone(),
            register_type: self.register_type.clone(),
            team_id_port: self.team_id_port.clone(),
            picture: self.picture.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            pwd: self.pwd.clone(),
            team_list: tl,
            version: VERSION.to_string(),
        }
    }
}
