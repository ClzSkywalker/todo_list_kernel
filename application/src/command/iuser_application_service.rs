use base::ddd::application_service::IApplicationService;

use crate::{
    ability::user::cmd::{user_reset_info_cmd::UserResetInfoCmd, user_update_cmd::UserUpdateCmd},
    query::model::common::dto::RespToken,
};

#[async_trait::async_trait]
pub trait IUserCmdApplicationService: IApplicationService {
    async fn create_by_id(&mut self) -> anyhow::Result<RespToken>;

    async fn update(&mut self, cmd: &UserUpdateCmd) -> anyhow::Result<()>;

    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-08-10
    /// Description    :  只拥有uid，第一次该密码需要把账号也绑上
    /// param           {*} self
    /// param           {UserResetInfoCmd} cmd
    /// return          {*}
    ///    
    async fn reset_user_info(&mut self, cmd: &UserResetInfoCmd) -> anyhow::Result<()>;
}
