use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use domain::aggregate::preclude::UserAggregate;
use infrastructure::{db::repository::new_user_repository, service::new_user_domain_service};

use self::user::{
    cmd::{user_create_command::UserCreateCommand, user_update_command::UserUpdateCommand},
    user_create_ability::UserCreateAbility,
    user_update_ability::UserUpdateAbility,
};

pub mod share;
pub mod task;
pub mod user;

pub fn new_user_create_ability(
    ctx: Arc<AppContext>,
) -> impl IAbility<R = UserAggregate, CMD = UserCreateCommand> {
    UserCreateAbility {
        ctx: ctx.clone(),
        user_service: new_user_domain_service(ctx.clone()),
    }
}

pub fn new_user_update_ability(
    ctx: Arc<AppContext>,
) -> impl IAbility<R = UserAggregate, CMD = UserUpdateCommand> {
    UserUpdateAbility {
        ctx: ctx.clone(),
        user_repository: new_user_repository(ctx.clone()),
        user: None,
    }
}
