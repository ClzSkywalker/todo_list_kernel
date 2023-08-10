use std::sync::Arc;

use common::contextx::AppContext;
use infrastructure::{db::repository::new_task_repostiory, service::new_user_domain_service};

use crate::ability::{
    task::{new_task_create_ability, new_task_update_ability},
    user::{new_user_reset_info_ability, new_user_update_ability},
};

use self::{
    itask_application_service::ITaskCmdApplicationService,
    iuser_application_service::IUserCmdApplicationService,
    service::{
        task_application_service::TaskCmdApplicationService,
        user_application_service::UserCmdApplicationService,
    },
};

pub mod itask_application_service;
pub mod iuser_application_service;
pub mod service;

pub fn new_task_cmd_application_service(ctx: Arc<AppContext>) -> impl ITaskCmdApplicationService {
    TaskCmdApplicationService {
        ctx: ctx.clone(),
        task_create_ability: new_task_create_ability(ctx.clone(), new_task_repostiory(ctx.clone())),
        task_update_ability: new_task_update_ability(ctx.clone(), new_task_repostiory(ctx.clone())),
        task_repository: new_task_repostiory(ctx.clone()),
    }
}

pub fn new_user_cmd_application_service(ctx: Arc<AppContext>) -> impl IUserCmdApplicationService {
    UserCmdApplicationService {
        ctx: ctx.clone(),
        user_service: new_user_domain_service(ctx.clone()),
        user_update_ability: new_user_update_ability(ctx.clone()),
        user_reset_info_ability: new_user_reset_info_ability(ctx.clone()),
    }
}
