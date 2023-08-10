use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use domain::aggregate::{
    preclude::UserAggregate, user::service::iuser_domain_service::IUserDomainService,
};

use super::cmd::user_create_cmd::UserCreateCmd;

pub struct UserCreateAbility<US>
where
    US: IUserDomainService,
{
    pub ctx: Arc<AppContext>,
    pub user_service: US,
}

#[async_trait::async_trait]
impl<US> IAbility for UserCreateAbility<US>
where
    US: IUserDomainService,
{
    type R = UserAggregate;
    type CMD = UserCreateCmd;
    async fn check_handler(&mut self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn check_idempotent(&mut self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn execute(&self, _: &Self::CMD) -> anyhow::Result<Self::R> {
        // self.user_service.register()
        todo!()
    }
}
