use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UserLoginEmailReq {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 18))]
    pub pwd: String,
}