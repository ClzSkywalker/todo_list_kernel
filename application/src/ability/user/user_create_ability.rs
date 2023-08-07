use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use domain::aggregate::{
    preclude::UserAggregate, user::service::iuser_domain_service::IUserDomainService,
};

use super::cmd::user_create_command::UserCreateCommand;

pub struct UserCreateAbility<US>
where
    US: IUserDomainService,
{
    ctx: Arc<AppContext>,
    user_service: US,
}

#[async_trait::async_trait]
impl<US> IAbility for UserCreateAbility<US>
where
    US: IUserDomainService,
{
    type R = UserAggregate;
    type CMD = UserCreateCommand;
    async fn check_handler(&mut self, cmd: &Self::CMD) -> anyhow::Result<()> {
        todo!()
    }
    async fn check_idempotent(&mut self, cmd: &Self::CMD) -> anyhow::Result<()> {
        todo!()
    }
    async fn execute(&self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        todo!()
    }
}
