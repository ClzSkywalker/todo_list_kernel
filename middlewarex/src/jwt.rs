use axum::{
    extract::{Extension, TypedHeader},
    headers::authorization::{Authorization, Bearer},
    http::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
    http,
};
use common::{
    contextx::AppContext,
    jwt::{self},
};

pub async fn auth<B>(
    // TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Extension(mut ctx): Extension<AppContext>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = get_auth_header(&req);

    let auth_header = match auth_header {
        Some(r) => r,
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // let req1 = &mut req;
    // let ctx = req1.extensions_mut().get::<AppContext>().as_mut().unwrap();

    let claim = match jwt::decode_token(ctx.locale, auth_header.as_str()) {
        Ok(r) => r,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    ctx.uid = claim.uid;
    ctx.tid = claim.tid;

    Ok(next.run(req).await)
    // Err(StatusCode::UNAUTHORIZED)

    // if let Some(current_user) = authorize_current_user(auth_header).await {
    //     // insert the current user into a request extension so the handler can
    //     // extract it
    //     req.extensions_mut().insert(current_user);
    //     Ok(next.run(req).await)
    // } else {
    //     Err(StatusCode::UNAUTHORIZED)
    // }
}

fn get_auth_header<B>(req: &Request<B>) -> Option<String> {
    // let req1 = &req;
    req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| match header.to_str() {
            Ok(r) => Some(r.to_string()),
            Err(e) => None,
        })
}
