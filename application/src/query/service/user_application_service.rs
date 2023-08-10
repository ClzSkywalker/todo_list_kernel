use std::sync::Arc;

use base::ddd::application_service::IApplicationService;
use chrono::Local;
use common::{contextx::AppContext, errorx::Errorx, jwt};
use domain::aggregate::user::repository::iuser_repository::IUserRepository;

use crate::query::{
    iuser_application_service::IUserApplicationService,
    model::{common::dto::RespToken, user::qry::user::UserLoginEmailReq},
};

pub struct UserApplicationService<UR>
where
    UR: IUserRepository,
{
    pub ctx: Arc<AppContext>,
    pub user_repository: UR,
}

impl<UR> IApplicationService for UserApplicationService<UR> where UR: IUserRepository {}

#[async_trait::async_trait]
impl<UR> IUserApplicationService for UserApplicationService<UR>
where
    UR: IUserRepository,
{
    async fn login_email(&self, data: UserLoginEmailReq) -> anyhow::Result<RespToken> {
        let user = match __self
            .user_repository
            .find_by_email(data.email.clone())
            .await
        {
            Ok(r) => match r {
                Some(r) => r,
                None => {
                    anyhow::bail!(Errorx::new(
                        self.ctx.locale,
                        common::i18n::I18nKey::UserNotfound
                    ))
                }
            },
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    self.ctx.locale,
                    common::i18n::I18nKey::UserNotfound
                ))
            }
        };

        match user.pwd_varify(self.ctx.locale, data.pwd.clone()) {
            Ok(r) => {
                if !r {
                    anyhow::bail!(Errorx::new(
                        self.ctx.locale,
                        common::i18n::I18nKey::UserLogin
                    ))
                }
            }
            Err(e) => {
                anyhow::bail!(e)
            }
        };

        let token = user.generate_token(self.ctx.locale)?;

        Ok(RespToken {
            token_type: jwt::TOKEN_TYPE.to_string(),
            token: token,
            expires_in: Local::now().timestamp() + jwt::EXP as i64,
        })
    }
}
