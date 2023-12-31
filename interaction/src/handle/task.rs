use std::sync::Arc;

use application::{
    ability::task::cmd::task_create_command::TaskCreateCommand,
    command::{itask_application_service::ITaskCmdApplicationService, new_task_cmd_application_service},
};
use axum::{extract::Path, Extension};
use common::{
    contextx::AppContext,
    res::{err_to_resp, Responsex},
};
use domain::aggregate::task::model::task::Task;
use middlewarex::validator::ValidatedJson;

pub async fn task_create(
    Extension(c): Extension<AppContext>,
    ValidatedJson(cmd): ValidatedJson<TaskCreateCommand>,
) -> Responsex<Task> {
    let ctx = Arc::new(c);
    let mut a = new_task_cmd_application_service(ctx.clone());
    match a.create(&cmd).await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}

pub async fn task_update(
    Extension(c): Extension<AppContext>,
    Path(id): Path<String>,
    ValidatedJson(cmd): ValidatedJson<TaskCreateCommand>,
) -> Responsex<()> {
    let ctx = Arc::new(c);
    let mut server = new_task_cmd_application_service(ctx.clone());
    match server.update(id, &cmd).await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}

pub async fn task_delete(
    Extension(c): Extension<AppContext>,
    Path(id): Path<String>,
) -> Responsex<()> {
    let ctx = Arc::new(c);
    let mut server = new_task_cmd_application_service(ctx.clone());
    match server.delete(id).await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}
