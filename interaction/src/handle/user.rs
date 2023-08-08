use std::sync::Arc;

use application::{
    ability::user::cmd::user_update_command::UserUpdateCommand,
    command::{iuser_application_service::IUserApplicationService, new_user_application_service},
    query::model::{common::dto::RespToken, user::dto::cud::UserUpdateReq},
};
use axum::Extension;
use common::contextx::AppContext;
use middlewarex::validator::ValidatedJson;

use super::res::{err_to_resp, Responsex};

pub async fn user_create(Extension(c): Extension<AppContext>) -> Responsex<RespToken> {
    let ctx = Arc::new(c);
    let mut server = new_user_application_service(ctx.clone());
    match server.create_by_id().await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}

pub async fn user_update(
    Extension(c): Extension<AppContext>,
    ValidatedJson(data): ValidatedJson<UserUpdateReq>,
) -> Responsex<()> {
    let ctx = Arc::new(c);
    let mut server = new_user_application_service(ctx.clone());
    let cmd = UserUpdateCommand::to_self(ctx.uid.to_string(), data);
    match server.update(&cmd).await {
        Ok(_) => Responsex::ok(ctx.locale),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}
