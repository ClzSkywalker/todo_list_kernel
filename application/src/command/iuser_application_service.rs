use base::ddd::application_service::IApplicationService;

use crate::{query::model::common::dto::RespToken, ability::user::cmd::user_update_command::UserUpdateCommand};

#[async_trait::async_trait]
pub trait IUserCmdApplicationService: IApplicationService {
    async fn create_by_id(&mut self) -> anyhow::Result<RespToken>;

    async fn update(&mut self, cmd: &UserUpdateCommand) -> anyhow::Result<()>;
}
