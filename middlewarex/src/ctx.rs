use std::fmt::Display;

use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use bytes::Bytes;
use common::{contextx::AppContext, i18n::Locale};
use infrastructure::db::model::common::GLOBAL_DB;
use sea_orm::{sea_query::tests_cfg::json, DatabaseConnection, DatabaseTransaction};

// #[derive(Debug)]
// pub struct AppContext {
//     pub db: DatabaseConnection,
//     pub tx: Option<DatabaseTransaction>,
//     pub uid: String,
//     pub tid: String,
//     pub locale: Locale,
// }

// impl AppContext {
//     pub fn new(db: DatabaseConnection, locale: Locale) -> Self {
//         Self {
//             db,
//             tx: None,
//             uid: "".to_string(),
//             tid: "".to_string(),
//             locale: locale,
//         }
//     }
// }

// impl Display for AppContext {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "ctx:{{uid:{},tid:{},locale:{})}}",
//             self.uid, self.tid, self.locale
//         )
//     }
// }

// impl Clone for AppContext {
//     fn clone(&self) -> Self {
//         Self {
//             db: self.db.clone(),
//             tx: None,
//             uid: self.uid.clone(),
//             tid: self.tid.clone(),
//             locale: self.locale.clone(),
//         }
//     }
// }

// #[axum::async_trait]
// impl<S> FromRequestParts<S> for AppContext
// where
//     S: Send + Sync,
// {
//     type Rejection = AuthError;
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let db = GLOBAL_DB.get().unwrap();
//         let ctx = parts.extensions.get::<AppContext>().unwrap();

//         // let ctx = AppContext::new(db.clone(), locale);
//         // parts.extensions.insert(UserInfoCtx {
//         //     id: user.id.clone(),
//         //     token_id: user.token_id.clone(),
//         //     name: user.name.clone(),
//         // });
//         Ok(ctx.clone())
//     }
// }

// #[derive(Debug)]
// pub enum AuthError {
//     WrongCredentials,
//     MissingCredentials,
//     TokenCreation,
//     InvalidToken,
//     CheckOutToken,
// }
// impl IntoResponse for AuthError {
//     fn into_response(self) -> Response {
//         let (status, error_message) = match self {
//             AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
//             AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
//             AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
//             AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
//             AuthError::CheckOutToken => (StatusCode::UNAUTHORIZED, "该账户已经退出"),
//         };
//         let body = Json(json!({
//             "error": error_message,
//         }));
//         (status, body).into_response()
//     }
// }

pub const HEADER_LANGUAGE: &str = "Accept-Language";

pub async fn ctx_fn_mid(req: Request<Body>, next: Next<Body>) -> impl IntoResponse {
    // req.extensions_mut().
    let locale = match req.headers().get(HEADER_LANGUAGE) {
        Some(r) => {
            let l = r
                .to_str()
                .unwrap_or("")
                .split("\r\n")
                .nth(0)
                .unwrap_or("")
                .to_owned();
            Locale::from(l.as_str())
        }
        None => Locale::En,
    };
    let db = GLOBAL_DB.get().unwrap();
    let ctx = AppContext::new(db.clone(), locale);

    let (parts, req_body) = req.into_parts();

    let (bytes, _) = match get_body_data(req_body).await {
        Err(e) => return Err(e),
        Ok((x, y)) => (x, y),
    };

    let mut req = Request::from_parts(parts, Body::from(bytes));

    req.extensions_mut().insert(ctx);
    let res = next.run(req).await;
    Ok(res)
}

async fn get_body_data<B>(body: B) -> Result<(Bytes, String), (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read body: {}", err),
            ));
        }
    };

    match std::str::from_utf8(&bytes) {
        Ok(x) => {
            let res_data = x.to_string();
            Ok((bytes, res_data))
        }
        Err(_) => Ok((bytes, "该数据无法转输出，可能为blob，binary".to_string())),
    }
}
