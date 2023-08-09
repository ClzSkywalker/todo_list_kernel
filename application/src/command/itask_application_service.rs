use base::ddd::application_service::IApplicationService;
use domain::aggregate::task::model::task::Task;

use crate::ability::task::cmd::task_create_command::TaskCreateCommand;

#[async_trait::async_trait]
pub trait ITaskCmdApplicationService: IApplicationService {
    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-08-01
    /// Description    : 创建task
    /// param           {*} self
    /// param           {*} cmd
    /// return          {*}
    ///    
    async fn create(&mut self, cmd: &TaskCreateCommand) -> anyhow::Result<Task>;

    async fn update(&mut self, id: String, cmd: &TaskCreateCommand) -> anyhow::Result<()>;

    async fn delete(&mut self, id: String) -> anyhow::Result<()>;
}
