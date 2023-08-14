use base::ddd::command::ICommand;
use common::config::VERSION;
use domain::aggregate::preclude::*;
use serde::Deserialize;

use crate::ability::share::user::UserResetInfoReq;

#[derive(Debug, Clone, Deserialize)]
pub struct UserResetInfoCmd {
    pub id: String,
    pub nick_name: String,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub pwd: String,
}

impl ICommand for UserResetInfoCmd {}

impl UserResetInfoCmd {
    pub fn to_ag(&self, ag: UserAggregate) -> UserAggregate {
        UserAggregate {
            id: self.id.clone(),
            nick_name: self.nick_name.clone(),
            member_type: ag.member_type,
            register_type: ag.register_type,
            team_id_port: ag.team_id_port,
            picture: self.picture.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            pwd: self.pwd.clone(),
            team_list: ag.team_list,
            version: VERSION.to_string(),
            resource: ag.resource,
        }
    }

    pub fn to_self(id: String, req: UserResetInfoReq) -> Self {
        Self {
            id: id,
            nick_name: req.nick_name,
            picture: req.picture,
            email: req.email,
            phone: req.phone,
            pwd: req.pwd,
        }
    }
}
