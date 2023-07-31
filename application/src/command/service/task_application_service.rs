use std::sync::Arc;

use base::ddd::{ability::IAbility, application_service::IApplicationService};
use common::contextx::AppContext;
use domain::aggregate::task::model::task::Task;

use crate::{
    ability::task::cmd::create_task_ability_command::CreateTaskAbilityCommand,
    command::itask_application_service::ITaskApplicationService,
};

pub struct TaskApplicationService<CB>
where
    CB: IAbility<R = Task, CMD = CreateTaskAbilityCommand>,
{
    pub ctx: Arc<AppContext>,
    pub create_task_ability: CB,
}

impl<CB> TaskApplicationService<CB>
where
    CB: IAbility<R = Task, CMD = CreateTaskAbilityCommand>,
{
    pub fn new(ctx: Arc<AppContext>, create_task_ability: CB) -> Self {
        TaskApplicationService {
            ctx: ctx,
            create_task_ability: create_task_ability,
        }
    }
}

impl<CB> IApplicationService for TaskApplicationService<CB> where
    CB: IAbility<R = Task, CMD = CreateTaskAbilityCommand>
{
}

#[async_trait::async_trait]
impl<CB> ITaskApplicationService for TaskApplicationService<CB>
where
    CB: IAbility<R = Task, CMD = CreateTaskAbilityCommand>,
{
    async fn create(&self, cmd: &CreateTaskAbilityCommand) -> anyhow::Result<Task> {
        self.create_task_ability.execute_ability(cmd).await
    }
}
