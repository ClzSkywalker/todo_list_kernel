use anyhow::Result;
use base::ddd::application_service::IApplicationService;
use domain::aggregate::task::model::task::Task;

use crate::ability::task::cmd::create_task_ability_command::CreateTaskAbilityCommand;

#[async_trait::async_trait]
pub trait ITaskApplicationService: IApplicationService {
    async fn create(&self, cmd: &CreateTaskAbilityCommand) -> Result<Task>;
}
