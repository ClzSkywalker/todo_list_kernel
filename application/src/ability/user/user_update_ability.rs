use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::{contextx::AppContext, errorx::Errorx, i18n::I18nKey};
use domain::aggregate::{
    preclude::UserAggregate, user::repository::iuser_repository::IUserRepository,
};

use super::cmd::user_update_command::UserUpdateCommand;

pub struct UserUpdateAbility<UR>
where
    UR: IUserRepository,
{
    pub ctx: Arc<AppContext>,
    pub user_repository: UR,
    pub user: Option<UserAggregate>,
}

#[async_trait::async_trait]
impl<UR> IAbility for UserUpdateAbility<UR>
where
    UR: IUserRepository,
{
    type R = UserAggregate;
    type CMD = UserUpdateCommand;
    async fn check_handler(&mut self, cmd: &Self::CMD) -> anyhow::Result<()> {
        let u = match __self.user_repository.by_id(cmd.id.clone()).await {
            Ok(r) => match r {
                Some(r) => r,
                None => {
                    anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserNotfound))
                }
            },
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserNotfound))
            }
        };
        self.user = Some(u);
        Ok(())
    }

    async fn check_idempotent(&mut self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }

    async fn execute(&self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        let ag = cmd.to_ag(self.user.clone().unwrap());
        match __self.user_repository.update(ag.clone()).await {
            Ok(_) => Ok(ag),
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserUpdate))
            }
        }
    }
}
