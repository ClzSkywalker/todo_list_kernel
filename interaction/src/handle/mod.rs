use axum::{routing::get, Router};

use self::config::get_config;

pub mod config;
pub mod res;
pub mod task;

pub fn unauth_api() -> Router {
    Router::new().route("/config", get(get_config))
}
