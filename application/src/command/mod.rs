use std::sync::Arc;

use common::contextx::AppContext;
use infrastructure::{db::repository::new_task_repostiory, service::new_user_domain_service};

use crate::ability::task::{new_task_create_ability, new_task_update_ability};

use self::{
    itask_application_service::ITaskApplicationService,
    iuser_application_service::IUserApplicationService,
    service::{
        task_application_service::TaskApplicationService,
        user_application_service::UserApplicationService,
    },
};

pub mod itask_application_service;
pub mod iuser_application_service;
pub mod service;

pub fn new_task_application_service(ctx: Arc<AppContext>) -> impl ITaskApplicationService {
    TaskApplicationService {
        ctx: ctx.clone(),
        task_create_ability: new_task_create_ability(ctx.clone(), new_task_repostiory(ctx.clone())),
        task_update_ability: new_task_update_ability(ctx.clone(), new_task_repostiory(ctx.clone())),
        task_repository: new_task_repostiory(ctx.clone()),
    }
}

pub fn new_user_application_service(ctx: Arc<AppContext>) -> impl IUserApplicationService {
    UserApplicationService {
        ctx: ctx.clone(),
        user_service: new_user_domain_service(ctx.clone()),
    }
}
