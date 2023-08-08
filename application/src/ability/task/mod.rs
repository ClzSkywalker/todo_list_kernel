use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use domain::aggregate::{
    preclude::TaskAggregate, task::repository::itask_repository::ITaskRepository,
};

use self::{
    cmd::{task_create_command::TaskCreateCommand, task_update_command::TaskUpdateCommand},
    task_create_ability::TaskCreateAbility,
    task_update_ability::TaskUpdateAbility,
};

pub mod assembler;
pub mod cmd;
pub mod task_create_ability;
pub mod task_update_ability;

pub fn new_task_create_ability<T: ITaskRepository<AG = TaskAggregate, ID = String>>(
    ctx: Arc<AppContext>,
    task_repository: T,
) -> impl IAbility<R = TaskAggregate, CMD = TaskCreateCommand> {
    TaskCreateAbility {
        task_repository: task_repository,
        ctx: ctx,
    }
}

pub fn new_task_update_ability<T: ITaskRepository<AG = TaskAggregate, ID = String>>(
    ctx: Arc<AppContext>,
    task_repository: T,
) -> impl IAbility<R = TaskAggregate, CMD = TaskUpdateCommand> {
    TaskUpdateAbility {
        task_repository: task_repository,
        task_content_id: "".to_string(),
        ctx: ctx,
    }
}
