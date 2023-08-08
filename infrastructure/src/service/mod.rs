use common::contextx::AppContext;
use domain::aggregate::user::service::iuser_domain_service::IUserDomainService;
use std::sync::Arc;

use crate::db::repository::*;

use self::user_domain_service::UserDomainService;

pub mod user_domain_service;

pub fn new_user_domain_service(ctx: Arc<AppContext>) -> impl IUserDomainService {
    UserDomainService {
        ctx: ctx.clone(),
        user_repostitory: new_user_repository(ctx.clone()),
        team_repository: new_team_repostiory(ctx.clone()),
        classify_repository: new_classify_repostiory(ctx.clone()),
        task_mode_repository: new_task_mode_repostiory(ctx.clone()),
        devide_repository: new_devide_repostiory(ctx.clone()),
        task_repository: new_task_repostiory(ctx.clone()),
    }
}
