use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use domain::aggregate::preclude::UserAggregate;
use infrastructure::{db::repository::new_user_repository, service::new_user_domain_service};

use self::{
    cmd::user_reset_info_cmd::UserResetInfoCmd, user_reset_info_ability::UserResetInfoAbility,
};

use super::user::{
    cmd::{user_create_cmd::UserCreateCmd, user_update_cmd::UserUpdateCmd},
    user_create_ability::UserCreateAbility,
    user_update_ability::UserUpdateAbility,
};

pub mod assembler;
pub mod cmd;
pub mod user_create_ability;
pub mod user_reset_info_ability;
pub mod user_update_ability;

pub fn new_user_create_ability(
    ctx: Arc<AppContext>,
) -> impl IAbility<R = UserAggregate, CMD = UserCreateCmd> {
    UserCreateAbility {
        ctx: ctx.clone(),
        user_service: new_user_domain_service(ctx.clone()),
    }
}

pub fn new_user_update_ability(
    ctx: Arc<AppContext>,
) -> impl IAbility<R = UserAggregate, CMD = UserUpdateCmd> {
    UserUpdateAbility {
        ctx: ctx.clone(),
        user_repository: new_user_repository(ctx.clone()),
        user: None,
    }
}

pub fn new_user_reset_info_ability(
    ctx: Arc<AppContext>,
) -> impl IAbility<R = UserAggregate, CMD = UserResetInfoCmd> {
    UserResetInfoAbility {
        ctx: ctx.clone(),
        user_repository: new_user_repository(ctx.clone()),
        user: None,
    }
}
