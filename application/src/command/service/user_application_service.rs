use std::sync::Arc;

use crate::{
    ability::user::cmd::user_update_command::UserUpdateCommand,
    command::iuser_application_service::IUserCmdApplicationService,
    query::model::common::dto::RespToken,
};
use base::ddd::{ability::IAbility, application_service::IApplicationService};
use chrono::Local;
use common::{contextx::AppContext, errorx::Errorx, jwt};
use domain::aggregate::{
    preclude::UserAggregate, user::service::iuser_domain_service::IUserDomainService,
};

pub struct UserCmdApplicationService<US, UUA>
where
    US: IUserDomainService,
    UUA: IAbility<R = UserAggregate, CMD = UserUpdateCommand>,
{
    pub ctx: Arc<AppContext>,
    pub user_service: US,
    pub user_update_ability: UUA,
}

impl<US, UUA> IApplicationService for UserCmdApplicationService<US, UUA>
where
    US: IUserDomainService,
    UUA: IAbility<R = UserAggregate, CMD = UserUpdateCommand>,
{
}

#[async_trait::async_trait]
impl<US, UUA> IUserCmdApplicationService for UserCmdApplicationService<US, UUA>
where
    US: IUserDomainService,
    UUA: IAbility<R = UserAggregate, CMD = UserUpdateCommand>,
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
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    self.ctx.locale,
                    common::i18n::I18nKey::EncryptionError
                ))
            }
        };

        Ok(RespToken {
            token_type: jwt::TOKEN_TYPE.to_string(),
            token: token,
            expires_in: Local::now().timestamp() + jwt::EXP as i64,
        })
    }

    async fn update(&mut self, cmd: &UserUpdateCommand) -> anyhow::Result<()> {
        match __self.user_update_ability.execute_ability(cmd).await {
            Ok(_) => Ok(()),
            Err(e) => {
                anyhow::bail!(e)
            }
        }
    }
}
