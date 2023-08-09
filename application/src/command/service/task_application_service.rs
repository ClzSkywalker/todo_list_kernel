use std::sync::Arc;

use base::ddd::{ability::IAbility, application_service::IApplicationService};
use common::contextx::AppContext;
use domain::aggregate::task::{model::task::Task, repository::itask_repository::ITaskRepository};

use crate::{
    ability::task::cmd::{
        task_create_command::TaskCreateCommand, task_update_command::TaskUpdateCommand,
    },
    command::itask_application_service::ITaskCmdApplicationService,
};

pub struct TaskCmdApplicationService<CTA, UTA, TR>
where
    CTA: IAbility<R = Task, CMD = TaskCreateCommand>,
    UTA: IAbility<R = Task, CMD = TaskUpdateCommand>,
    TR: ITaskRepository<AG = Task, ID = String>,
{
    pub ctx: Arc<AppContext>,
    pub task_create_ability: CTA,
    pub task_update_ability: UTA,
    pub task_repository: TR,
}

impl<CTA, UTA, TR> TaskCmdApplicationService<CTA, UTA, TR>
where
    CTA: IAbility<R = Task, CMD = TaskCreateCommand>,
    UTA: IAbility<R = Task, CMD = TaskUpdateCommand>,
    TR: ITaskRepository<AG = Task, ID = String>,
{
}

impl<CTA, UTA, TR> IApplicationService for TaskCmdApplicationService<CTA, UTA, TR>
where
    CTA: IAbility<R = Task, CMD = TaskCreateCommand>,
    UTA: IAbility<R = Task, CMD = TaskUpdateCommand>,
    TR: ITaskRepository<AG = Task, ID = String>,
{
}

#[async_trait::async_trait]
impl<CTA, UTA, TR> ITaskCmdApplicationService for TaskCmdApplicationService<CTA, UTA, TR>
where
    CTA: IAbility<R = Task, CMD = TaskCreateCommand>,
    UTA: IAbility<R = Task, CMD = TaskUpdateCommand>,
    TR: ITaskRepository<AG = Task, ID = String>,
{
    async fn create(&mut self, cmd: &TaskCreateCommand) -> anyhow::Result<Task> {
        self.task_create_ability.execute_ability(cmd).await
    }

    async fn update(&mut self, id: String, cmd: &TaskCreateCommand) -> anyhow::Result<()> {
        let cmd = &TaskUpdateCommand::from_task_create(id, cmd.clone());
        match __self.task_update_ability.execute_ability(cmd).await {
            Ok(_) => Ok(()),
            Err(e) => {
                anyhow::bail!(e)
            }
        }
    }

    async fn delete(&mut self, id: String) -> anyhow::Result<()> {
        self.task_repository.delete(id).await
    }
}
