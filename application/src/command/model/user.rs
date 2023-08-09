use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UserUpdateReq {
    #[validate(length(min = 2))]
    pub nick_name: String,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}
