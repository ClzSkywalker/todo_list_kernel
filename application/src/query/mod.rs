use std::sync::Arc;

use common::contextx::AppContext;
use infrastructure::db::repository::new_user_repository;

use self::{
    iuser_application_service::IUserApplicationService,
    service::user_application_service::UserApplicationService,
};

pub mod assembler;
pub mod model;
pub mod repository;
pub mod service;

pub mod itask_application_service;
pub mod iuser_application_service;

pub fn new_user_application_service(ctx: Arc<AppContext>) -> impl IUserApplicationService {
    UserApplicationService {
        ctx: ctx.clone(),
        user_repository: new_user_repository(ctx.clone()),
    }
}
