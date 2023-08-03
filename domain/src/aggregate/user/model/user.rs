use base::ddd::aggregate::IAggregate;
use serde::Serialize;

use super::team::Team;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    nick_name: String,
    member_type: String,
    register_type: String,
    picture: String,
    email: String,
    phone: String,
    pwd: String,
    team_list: Vec<Team>,
    version: String,
}
impl IAggregate for User {}
