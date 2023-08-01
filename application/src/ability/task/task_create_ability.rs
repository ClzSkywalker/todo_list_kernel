use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::{contextx::AppContext, utils};
use domain::aggregate::task::{
    model::{task::Task, task_content::TaskContent},
    repository::itask_repository::ITaskRepository,
};

use super::cmd::task_create_command::TaskCreateCommand;

pub fn new_task_create_ability<T: ITaskRepository<AG = Task, ID = String>>(
    ctx: Arc<AppContext>,
    task_repository: T,
) -> impl IAbility<R = Task, CMD = TaskCreateCommand> {
    TaskCreateAbility {
        task_repository: task_repository,
        ctx: ctx,
    }
}

pub struct TaskCreateAbility<TR>
where
    TR: ITaskRepository<AG = Task, ID = String>,
{
    pub task_repository: TR,
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl<TR> IAbility for TaskCreateAbility<TR>
where
    TR: ITaskRepository<AG = Task, ID = String>,
{
    type R = Task;
    type CMD = TaskCreateCommand;
    async fn check_handler(&self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn check_idempotent(&self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn execute(&self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        let tc_ulid = utils::generate_ulid();
        let tc = TaskContent {
            id: tc_ulid.clone(),
            content: cmd.task_content.clone(),
        };
        match self.task_repository.content_insert(tc).await {
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
