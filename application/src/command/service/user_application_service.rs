use std::sync::Arc;

use crate::{
    ability::user::cmd::{user_reset_info_cmd::UserResetInfoCmd, user_update_cmd::UserUpdateCmd},
    command::iuser_application_service::IUserCmdApplicationService,
    query::model::common::dto::RespToken,
};
use base::ddd::{ability::IAbility, application_service::IApplicationService};
use chrono::Local;
use common::{contextx::AppContext, jwt};
use domain::aggregate::{
    preclude::UserAggregate, user::service::iuser_domain_service::IUserDomainService,
};

pub struct UserCmdApplicationService<US, UUA, URIA>
where
    US: IUserDomainService,
    UUA: IAbility<R = UserAggregate, CMD = UserUpdateCmd>,
    URIA: IAbility<R = UserAggregate, CMD = UserResetInfoCmd>,
{
    pub ctx: Arc<AppContext>,
    pub user_service: US,
    pub user_update_ability: UUA,
    pub user_reset_info_ability: URIA,
}

impl<US, UUA, URIA> IApplicationService for UserCmdApplicationService<US, UUA, URIA>
where
    US: IUserDomainService,
    UUA: IAbility<R = UserAggregate, CMD = UserUpdateCmd>,
    URIA: IAbility<R = UserAggregate, CMD = UserResetInfoCmd>,
{
}

#[async_trait::async_trait]
impl<US, UUA, URIA> IUserCmdApplicationService for UserCmdApplicationService<US, UUA, URIA>
where
    US: IUserDomainService,
    UUA: IAbility<R = UserAggregate, CMD = UserUpdateCmd>,
    URIA: IAbility<R = UserAggregate, CMD = UserResetInfoCmd>,
{
    async fn create_by_id(&mut self) -> anyhow::Result<RespToken> {
        let u = self.user_service.register().await?;
        let token = u.generate_token(self.ctx.locale)?;

        Ok(RespToken {
            token_type: jwt::TOKEN_TYPE.to_string(),
            token: token,
            expires_in: Local::now().timestamp() + jwt::EXP as i64,
        })
    }

    async fn update(&mut self, cmd: &UserUpdateCmd) -> anyhow::Result<()> {
        match __self.user_update_ability.execute_ability(cmd).await {
            Ok(_) => Ok(()),
            Err(e) => {
                anyhow::bail!(e)
            }
        }
    }

    async fn reset_user_info(&mut self, cmd: &UserResetInfoCmd) -> anyhow::Result<()> {
        match __self.user_reset_info_ability.execute_ability(&cmd).await {
            Ok(_) => Ok(()),
            Err(e) => {
                anyhow::bail!(e)
            }
        }
    }
}
