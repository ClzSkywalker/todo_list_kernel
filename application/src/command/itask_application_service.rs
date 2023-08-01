use base::ddd::application_service::IApplicationService;
use domain::aggregate::task::model::task::Task;

use crate::ability::task::cmd::{
    task_create_command::TaskCreateCommand, task_update_command::TaskUpdateCommand,
};

#[async_trait::async_trait]
pub trait ITaskApplicationService: IApplicationService {
    ///
    /// Author         : ClzSkywalker
    /// Date           : 2023-08-01
    /// Description    : 创建task
    /// param           {*} self
    /// param           {*} cmd
    /// return          {*}
    ///    
    async fn create(&self, cmd: &TaskCreateCommand) -> anyhow::Result<Task>;

    async fn update(&self, cmd: &TaskUpdateCommand) -> anyhow::Result<()>;

    async fn delete(&self, id: String) -> anyhow::Result<()>;
}
