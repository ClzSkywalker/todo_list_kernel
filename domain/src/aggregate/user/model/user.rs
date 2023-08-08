use std::sync::Arc;

use base::ddd::aggregate::IAggregate;
use common::{config::VERSION, contextx::AppContext, i18n::Locale};
use serde::Serialize;

use crate::share::value_object::user_type::{MemberType, RegisterType};

use super::team::Team;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub nick_name: String,
    pub member_type: MemberType,
    pub register_type: RegisterType,
    pub team_id_port: String,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub pwd: String,
    pub team_list: Vec<Team>,
    pub version: String,
}
impl IAggregate for User {}

impl User {
    pub fn init_data(ctx: Arc<AppContext>) -> Self {
        let tid = common::utils::generate_ulid();
        match ctx.locale {
            Locale::Zh => Self {
                id: common::utils::generate_ulid(),
                nick_name: String::from("花猫"),
                member_type: MemberType::Normal,
                register_type: RegisterType::Uid,
                team_id_port: tid.clone(),
                picture: None,
                email: None,
                phone: None,
                pwd: String::from(""),
                team_list: vec![Team {
                    id: tid,
                    name: String::from(""),
                }],
                version: VERSION.to_string(),
            },
            Locale::En => Self {
                id: common::utils::generate_ulid(),
                nick_name: String::from("hi"),
                member_type: MemberType::Normal,
                register_type: RegisterType::Uid,
                team_id_port: tid.clone(),
                picture: None,
                email: None,
                phone: None,
                pwd: String::from(""),
                team_list: vec![Team {
                    id: tid,
                    name: String::from(""),
                }],
                version: VERSION.to_string(),
            },
        }
    }
}
