use base::ddd::aggregate::IAggregate;
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
