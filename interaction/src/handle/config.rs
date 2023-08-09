use axum::Extension;
use common::{contextx::AppContext, res::Responsex};

pub async fn get_config(Extension(c): Extension<AppContext>) -> Responsex<String> {
    Responsex::ok_with_data(c.locale.to_string())
}
