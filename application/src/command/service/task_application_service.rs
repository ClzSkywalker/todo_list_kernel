use std::sync::Arc;

use base::ddd::application_service::IApplicationService;
use common::{contextx::AppContext, utils};
use domain::aggregate::task::{
    model::{task::Task, task_content::TaskContent},
    repository::itask_repository::ITaskRepository,
};

use crate::{
    ability::task::cmd::create_task_ability_command::CreateTaskAbilityCommand,
    command::itask_application_service::ITaskApplicationService,
};

pub struct TaskApplicationService {
    pub ctx: Arc<AppContext>,
    pub task_repository: Arc<dyn ITaskRepository<AG = Task, ID = String>>,
}

impl IApplicationService for TaskApplicationService {}

#[async_trait::async_trait]
impl ITaskApplicationService for TaskApplicationService {
    async fn create(&self, cmd: CreateTaskAbilityCommand) -> anyhow::Result<Task> {
        let tc_ulid = utils::generate_ulid();
        let tc = TaskContent {
            uuid: tc_ulid.clone(),
            content: cmd.task_content.clone(),
        };
        match self.task_repository.insert_content(tc).await {
            Ok(_) => {}
            Err(e) => anyhow::bail!("{}", e),
        };
        let task = cmd.to_task("test".to_string(), tc_ulid);
        let task = match self.task_repository.insert(task).await {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e);
            }
        };
        Ok(task)
    }
}
