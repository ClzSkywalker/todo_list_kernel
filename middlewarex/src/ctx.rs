use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use bytes::Bytes;
use common::{contextx::AppContext, i18n::Locale};
use infrastructure::db::model::common::GLOBAL_DB;

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
