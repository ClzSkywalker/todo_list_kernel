use base::ddd::application_service::IApplicationService;

use super::model::{common::dto::RespToken, user::qry::user::UserLoginEmailReq};

#[async_trait::async_trait]
pub trait IUserApplicationService: IApplicationService {
    async fn login_email(&self, data: UserLoginEmailReq) -> anyhow::Result<RespToken>;
}
