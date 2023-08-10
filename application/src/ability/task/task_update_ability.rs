use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::{contextx::AppContext, errorx::Errorx};
use domain::aggregate::task::{model::task::Task, repository::itask_repository::ITaskRepository};

use super::cmd::task_update_command::TaskUpdateCommand;

pub struct TaskUpdateAbility<TR>
where
    TR: ITaskRepository<AG = Task, ID = String>,
{
    pub task_repository: TR,
    pub task_content_id: String,
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl<TR> IAbility for TaskUpdateAbility<TR>
where
    TR: ITaskRepository<AG = Task, ID = String>,
{
    type R = Task;
    type CMD = TaskUpdateCommand;
    async fn check_handler(&mut self, cmd: &Self::CMD) -> anyhow::Result<()> {
        let task = match __self.task_repository.by_id(cmd.id.clone()).await {
            Ok(r) => match r {
                Some(r) => r,
                None => {
                    anyhow::bail!(Errorx::new(
                        self.ctx.locale,
                        common::i18n::I18nKey::TaskNotFound
                    ))
                }
            },
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    self.ctx.locale,
                    common::i18n::I18nKey::TaskNotFound
                ))
            }
        };
        self.task_content_id = task.task_content.id;
        Ok(())
    }
    async fn check_idempotent(&mut self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn execute(&self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        let task = cmd.to_task(self.ctx.uid.clone(), self.task_content_id.clone());
        match self.task_repository.update(task.clone()).await {
            Ok(_) => {}
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    self.ctx.locale,
                    common::i18n::I18nKey::TaskUpdate
                ));
            }
        };
        Ok(task)
    }
}
