use std::sync::Arc;

use application::{
    command::{iuser_application_service::IUserApplicationService, new_user_application_service},
    query::model::common::dto::RespToken,
};
use axum::Extension;
use common::contextx::AppContext;

use super::res::{err_to_resp, Responsex};

pub async fn user_create(Extension(c): Extension<AppContext>) -> Responsex<RespToken> {
    let ctx = Arc::new(c);
    let mut a = new_user_application_service(ctx.clone());
    match a.create_by_id().await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}
