use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RespToken {
    pub token_type: String,
    pub token: String,
    pub expires_in: i64,
}
