use std::sync::Arc;

use crate::{
    command::iuser_application_service::IUserApplicationService,
    query::model::common::dto::RespToken,
};
use base::ddd::application_service::IApplicationService;
use chrono::Local;
use common::{contextx::AppContext, jwt};
use domain::aggregate::user::service::iuser_domain_service::IUserDomainService;

pub struct UserApplicationService<US>
where
    US: IUserDomainService,
{
    pub ctx: Arc<AppContext>,
    pub user_service: US,
}

impl<US> IApplicationService for UserApplicationService<US> where US: IUserDomainService {}

#[async_trait::async_trait]
impl<US> IUserApplicationService for UserApplicationService<US>
where
    US: IUserDomainService,
{
    async fn create_by_id(&mut self) -> anyhow::Result<RespToken> {
        let u = match __self.user_service.register().await {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e)
            }
        };
        let token = match jwt::generate_token(u.id, u.team_id_port) {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e)
            }
        };

        Ok(RespToken {
            token_type: jwt::TOKEN_TYPE.to_string(),
            token: token,
            expires_in: Local::now().timestamp() + jwt::EXP as i64,
        })
    }
}
