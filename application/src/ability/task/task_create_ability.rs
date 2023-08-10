use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use domain::aggregate::task::{model::task::Task, repository::itask_repository::ITaskRepository};

use super::cmd::task_create_command::TaskCreateCommand;

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
    async fn check_handler(&mut self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn check_idempotent(&mut self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn execute(&self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        let task = cmd.to_task(self.ctx.uid.clone());

        let task = match self.task_repository.insert(task).await {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e);
            }
        };
        Ok(task)
    }
}
