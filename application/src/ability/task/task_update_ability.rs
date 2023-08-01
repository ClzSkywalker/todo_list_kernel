use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::{contextx::AppContext, errorx::Errorx};
use domain::aggregate::task::{model::task::Task, repository::itask_repository::ITaskRepository};

use super::cmd::task_update_command::TaskUpdateCommand;

pub fn new_task_update_ability<T: ITaskRepository<AG = Task, ID = String>>(
    ctx: Arc<AppContext>,
    task_repository: T,
) -> impl IAbility<R = Task, CMD = TaskUpdateCommand> {
    TaskUpdateAbility {
        task_repository: task_repository,
        ctx: ctx,
    }
}

pub struct TaskUpdateAbility<TR>
where
    TR: ITaskRepository<AG = Task, ID = String>,
{
    pub task_repository: TR,
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl<TR> IAbility for TaskUpdateAbility<TR>
where
    TR: ITaskRepository<AG = Task, ID = String>,
{
    type R = Task;
    type CMD = TaskUpdateCommand;
    async fn check_handler(&self, cmd: &Self::CMD) -> anyhow::Result<()> {
        match __self.task_repository.by_id(cmd.id.clone()).await {
            Ok(_) => {}
            Err(_) => {
                anyhow::bail!(Errorx::new(
                    self.ctx.locale,
                    common::i18n::I18nKey::TaskNotFound
                ))
            }
        }
        Ok(())
    }
    async fn check_idempotent(&self, _: &Self::CMD) -> anyhow::Result<()> {
        Ok(())
    }
    async fn execute(&self, cmd: &Self::CMD) -> anyhow::Result<Self::R> {
        let task = match self.task_repository.first_by_id(cmd.id.clone()).await {
            Ok(r) => match r {
                Some(r) => r,
                None => {
                    tracing::error!("{},cmd:{:?}", self.ctx.to_string(), cmd);
                    anyhow::bail!(Errorx::new(
                        self.ctx.locale,
                        common::i18n::I18nKey::TaskNotFound
                    ))
                }
            },
            Err(e) => {
                tracing::error!("{},e:{},cmd:{:?}", self.ctx.to_string(), e, cmd);
                anyhow::bail!(Errorx::new(
                    self.ctx.locale,
                    common::i18n::I18nKey::TaskNotFound
                ))
            }
        };
        let task = cmd.to_task("test".to_string(), task.task_content.id.clone());
        match self.task_repository.update(task.clone()).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{},e:{},cmd:{:?}", self.ctx.to_string(), e, cmd);
                anyhow::bail!(Errorx::new(
                    self.ctx.locale,
                    common::i18n::I18nKey::TaskUpdate
                ))
            }
        };
        Ok(task)
    }
}
