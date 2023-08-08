pub mod classify_repository;
pub mod devide_repository;
pub mod task_mode_repository;
pub mod task_repository;
pub mod team_repository;
pub mod user_repository;

use std::sync::Arc;

use common::contextx::AppContext;
use domain::aggregate::{
    classify::repository::iclassify_repository::IClassifyRepository,
    devide::repository::idevide_repository::IDevideRepository,
    preclude::{ClassifyAggregate, DevideAggregate, TaskModeAggregate, TaskAggregate, TeamAggregate},
    task_mode::repository::itask_mode_repository::ITaskModeRepository, task::repository::itask_repository::ITaskRepository, team::repository::item_repository::ITeamRepository, user::repository::iuser_repository::IUserRepository,
};

use self::{
    classify_repository::ClassifyRepository, devide_repository::DevideRepository,
    task_mode_repository::TaskModeRepository, task_repository::TaskRepository, team_repository::TeamRepository, user_repository::UserRepository,
};

pub fn new_classify_repostiory(
    ctx: Arc<AppContext>,
) -> impl IClassifyRepository<AG = ClassifyAggregate, ID = String> {
    ClassifyRepository { ctx: ctx }
}

pub fn new_devide_repostiory(
    ctx: Arc<AppContext>,
) -> impl IDevideRepository<AG = DevideAggregate, ID = String> {
    DevideRepository { ctx: ctx }
}

pub fn new_task_mode_repostiory(
    ctx: Arc<AppContext>,
) -> impl ITaskModeRepository<AG = TaskModeAggregate, ID = String> {
    TaskModeRepository { ctx: ctx }
}

pub fn new_task_repostiory(
    ctx: Arc<AppContext>,
) -> impl ITaskRepository<AG = TaskAggregate, ID = String> {
    TaskRepository { ctx: ctx }
}

pub fn new_team_repostiory(
    ctx: Arc<AppContext>,
) -> impl ITeamRepository<AG = TeamAggregate, ID = String> {
    TeamRepository { ctx: ctx }
}

pub fn new_user_repository(ctx: Arc<AppContext>) -> impl IUserRepository {
    UserRepository { ctx: ctx }
}
