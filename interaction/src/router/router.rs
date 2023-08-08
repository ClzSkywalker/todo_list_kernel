use axum::{middleware, Router};
use middlewarex::jwt;

use crate::handle::{auth_api, unauth_api};

pub fn create_router() -> Router {
    Router::new().nest("/api/v1", global_router())
}

fn global_router() -> Router {
    Router::new()
        .nest("/unauth", set_unauth_middleware(unauth_api()))
        .nest("/auth", set_auth_middleware(auth_api()))
}

fn set_unauth_middleware(router: Router) -> Router {
    router.layer(middleware::from_fn(middlewarex::ctx::ctx_fn_mid))
}

fn set_auth_middleware(router: Router) -> Router {
    router
        .layer(middleware::from_fn(jwt::auth))
        .layer(middleware::from_fn(middlewarex::ctx::ctx_fn_mid))
    // .layer(Extension(AppContext::new(
    //     GLOBAL_DB.get().unwrap().clone(),
    //     common::i18n::Locale::En,
    // )))
}
