use std::sync::Arc;

use base::ddd::ability::IAbility;
use common::contextx::AppContext;
use infrastructure::service::new_user_domain_service;

use self::user::user_create_ability::UserCreateAbility;

pub mod share;
pub mod task;
pub mod user;

pub fn new_user_create_ability(ctx: Arc<AppContext>) -> impl IAbility {
    UserCreateAbility {
        ctx: ctx.clone(),
        user_service: new_user_domain_service(ctx.clone()),
    }
}
