use std::sync::Arc;

use application::{
    ability::task::{
        cmd::create_task_ability_command::CreateTaskAbilityCommand,
        task_create_ability::TaskCreateAbility,
    },
    command::{
        itask_application_service::ITaskApplicationService,
        service::task_application_service::TaskApplicationService,
    },
};
use axum::{Extension, Json};
use common::contextx::AppContext;
use domain::aggregate::task::model::task::Task;
use infrastructure::db::repository::task_repository::TaskRepository;

use super::res::{err_to_resp, Responsex};

pub async fn task_create(
    Extension(c): Extension<AppContext>,
    Json(cmd): Json<CreateTaskAbilityCommand>,
) -> Responsex<Task> {
    let c = Arc::new(c);
    let a = &TaskApplicationService::new(
        c.clone(),
        TaskCreateAbility::new(c.clone(), TaskRepository::new(c.clone())),
    );
    match a.create(&cmd).await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, c.locale),
    }
}
