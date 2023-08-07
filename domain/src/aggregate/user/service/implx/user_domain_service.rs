use std::sync::Arc;

use super::super::super::super::preclude::*;
use base::ddd::domain_service::IDomainService;
use common::{contextx::AppContext, errorx::Errorx, i18n::I18nKey};

use crate::aggregate::{
    team::repository::item_repository::ITeamRepository,
    user::{
        repository::iuser_repository::IUserRepository,
        service::iuser_domain_service::IUserDomainService,
    },
};

pub struct UserDomainService<UR, TR>
where
    UR: IUserRepository,
    TR: ITeamRepository,
{
    pub ctx: Arc<AppContext>,
    pub user_repostitory: UR,
    pub team_repository: TR,
}

impl<UR, TR> IDomainService for UserDomainService<UR, TR>
where
    UR: IUserRepository,
    TR: ITeamRepository,
{
}

#[async_trait::async_trait]
impl<UR, TR> IUserDomainService for UserDomainService<UR, TR>
where
    UR: IUserRepository,
    TR: ITeamRepository,
{
    async fn register(&self) -> anyhow::Result<UserAggregate> {
        let u = UserAggregate::init_data(self.ctx.clone());
        let u = match __self.user_repostitory.insert(u).await {
            Ok(r) => r,
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserCreate))
            }
        };

        let t = TeamAggregate::init_data(self.ctx.clone(), u.id, u.team_id_port);
        let t = match __self.team_repository.insert(t).await {
            Ok(r) => r,
            Err(_) => {
                anyhow::bail!(Errorx::new(self.ctx.locale, I18nKey::UserCreate))
            }
        };

        

        todo!()
    }
}
