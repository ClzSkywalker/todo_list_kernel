use axum::{
    extract::{rejection::FormRejection, FromRequest},
    response::{IntoResponse, Response},
    Form,
};
use hyper::{Request, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait::async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = match Form::<T>::from_request(req, state).await {
            Ok(r) => r,
            Err(e) => return Err(ServerError::AxumFormRejection(e)),
        };
        match value.validate() {
            Ok(_) => {}
            Err(e) => return Err(ServerError::ValidationError(e)),
        };
        Ok(ValidatedForm(value))
    }
}

pub enum ServerError {
    ValidationError(validator::ValidationErrors),

    AxumFormRejection(FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(item) => {
                let m = r#"{"code":"400","msg":"msg_data"}"#;
                let m = m.replace("msg_data", item.to_string().as_str());
                let message: Value = serde_json::from_str(m.as_str()).unwrap();
                (StatusCode::BAD_REQUEST, message.to_string())
            }
            ServerError::AxumFormRejection(item) => {
                let m = r#"{"code":"400","msg":"msg_data"}"#;
                let m = m.replace("msg_data", item.to_string().as_str());
                let message: Value = serde_json::from_str(m.as_str()).unwrap();
                (StatusCode::BAD_REQUEST, message.to_string())
            }
        }
        .into_response()
    }
}
