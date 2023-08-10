use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::{contextx::AppContext, errorx::Errorx, i18n::I18nKey};
use domain::aggregate::{
    preclude::UserAggregate, user::repository::iuser_repository::IUserRepository,
};

use super::cmd::user_reset_info_cmd::UserResetInfoCmd;

pub struct UserResetInfoAbility<UR>
where
    UR: IUserRepository,
{
    pub ctx: Arc<AppContext>,
    pub user_repository: UR,
    pub user: Option<UserAggregate>,
}

#[async_trait::async_trait]
impl<UR> IAbility for UserResetInfoAbility<UR>
where
    UR: IUserRepository,
{
    type R = UserAggregate;
    type CMD = UserResetInfoCmd;
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
        let u = self.user.clone().unwrap();
        if !u.pwd.is_empty() || !u.email.is_some() || !u.phone.is_some() {
            anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UseResetInfo))
        }
        Ok(())
    }

    async fn execute(&self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        let mut ag = cmd.to_ag(self.user.clone().unwrap());
        ag.pwd_bcrypt(self.ctx.locale)?;
        match __self.user_repository.update(ag.clone()).await {
            Ok(_) => Ok(ag),
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserUpdate))
            }
        }
    }
}
