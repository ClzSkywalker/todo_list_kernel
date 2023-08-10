use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UserUpdateReq {
    #[validate(length(min = 2))]
    pub nick_name: String,
    pub picture: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    // #[validate(phone)]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UserResetInfoReq {
    #[validate(length(min = 2))]
    pub nick_name: String,
    pub picture: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    // #[validate(phone)]
    pub phone: Option<String>,
    #[validate(length(min = 6, max = 18))]
    pub pwd: String,
}
