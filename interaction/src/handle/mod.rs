use self::config::get_config;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub mod config;
pub mod res;
pub mod task;

pub fn unauth_api() -> Router {
    Router::new().route("/config", get(get_config))
}

pub fn auth_api() -> Router {
    Router::new().nest("/task", task_api())
}

fn task_api() -> Router {
    Router::new()
        .route("/", post(task::task_create))
        .route("/:id", put(task::task_update))
        .route("/:id", delete(task::task_delete))
}
