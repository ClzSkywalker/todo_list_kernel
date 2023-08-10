use std::sync::Arc;

use application::{
    ability::{user::cmd::{user_update_cmd::UserUpdateCmd, user_reset_info_cmd::UserResetInfoCmd}, share::user::{UserUpdateReq, UserResetInfoReq}},
    command::{
        iuser_application_service::IUserCmdApplicationService, new_user_cmd_application_service,
    },
    query::{
        iuser_application_service::IUserApplicationService,
        model::{common::dto::RespToken, user::qry::user::UserLoginEmailReq},
        new_user_application_service,
    },
};
use axum::Extension;
use common::{
    contextx::AppContext,
    res::{err_to_resp, Responsex},
};
use middlewarex::validator::ValidatedJson;

pub async fn user_create(Extension(c): Extension<AppContext>) -> Responsex<RespToken> {
    let ctx = Arc::new(c);
    let mut server = new_user_cmd_application_service(ctx.clone());
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
    let mut server = new_user_cmd_application_service(ctx.clone());
    let cmd = UserUpdateCmd::to_self(ctx.uid.to_string(), data);
    match server.update(&cmd).await {
        Ok(_) => Responsex::ok(ctx.locale),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}

pub async fn user_reset_info(
    Extension(c): Extension<AppContext>,
    ValidatedJson(data): ValidatedJson<UserResetInfoReq>,
) -> Responsex<()> {
    let ctx = Arc::new(c);
    let mut server = new_user_cmd_application_service(ctx.clone());
    let cmd = UserResetInfoCmd::to_self(ctx.uid.to_string(), data);
    match server.reset_user_info(&cmd).await {
        Ok(_) => Responsex::ok(ctx.locale),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}

pub async fn user_login_email(
    Extension(c): Extension<AppContext>,
    ValidatedJson(data): ValidatedJson<UserLoginEmailReq>,
) -> Responsex<RespToken> {
    let ctx = Arc::new(c);
    let service = new_user_application_service(ctx.clone());
    match service.login_email(data).await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}