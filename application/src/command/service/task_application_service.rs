use std::sync::Arc;

use anyhow::Ok;
use base::ddd::{ability::IAbility, application_service::IApplicationService};
use common::contextx::AppContext;
use domain::aggregate::task::model::task::Task;
use infrastructure::db::repository::task_repository::{self};

use crate::{
    ability::task::{
        cmd::{task_create_command::TaskCreateCommand, task_update_command::TaskUpdateCommand},
        task_create_ability::{self},
    },
    command::itask_application_service::ITaskApplicationService,
};

pub fn new_task_application_service(ctx: Arc<AppContext>) -> impl ITaskApplicationService {
    TaskApplicationService {
        ctx: ctx.clone(),
        task_create_ability: task_create_ability::new_task_create_ability(
            ctx.clone(),
            task_repository::new_task_repostiory(ctx.clone()),
        ),
    }
}

pub struct TaskApplicationService<CB>
where
    CB: IAbility<R = Task, CMD = TaskCreateCommand>,
{
    pub ctx: Arc<AppContext>,
    pub task_create_ability: CB,
}

impl<CB> TaskApplicationService<CB> where CB: IAbility<R = Task, CMD = TaskCreateCommand> {}

impl<CB> IApplicationService for TaskApplicationService<CB> where
    CB: IAbility<R = Task, CMD = TaskCreateCommand>
{
}

#[async_trait::async_trait]
impl<CB> ITaskApplicationService for TaskApplicationService<CB>
where
    CB: IAbility<R = Task, CMD = TaskCreateCommand>,
{
    async fn create(&self, cmd: &TaskCreateCommand) -> anyhow::Result<Task> {
        self.task_create_ability.execute_ability(cmd).await
    }

    async fn update(&self, cmd: &TaskUpdateCommand) -> anyhow::Result<()> {
        Ok(())
    }

    async fn delete(&self, id: String) -> anyhow::Result<()> {
        Ok(())
    }
}
