use std::sync::Arc;

use application::{
    ability::task::cmd::task_create_command::TaskCreateCommand,
    command::{
        itask_application_service::ITaskApplicationService,
        service::task_application_service::{self},
    },
};
use axum::{Extension, Json};
use common::contextx::AppContext;
use domain::aggregate::task::model::task::Task;

use super::res::{err_to_resp, Responsex};

pub async fn task_create(
    Extension(c): Extension<AppContext>,
    Json(cmd): Json<TaskCreateCommand>,
) -> Responsex<Task> {
    let ctx = Arc::new(c);
    let a = task_application_service::new_task_application_service(ctx.clone());
    match a.create(&cmd).await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}
