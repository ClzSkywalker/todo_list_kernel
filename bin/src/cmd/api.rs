use std::{net::SocketAddr, str::FromStr};

use axum::Server;
use common::config::{AppConfig, APP_CONFIG};
use infrastructure::db::model::common::{init_db, GLOBAL_DB};
use interaction::router;

const LOCAL_HOSE: &str = "0.0.0.0";
pub async fn server_api(config: AppConfig) {
    APP_CONFIG.get_or_init(|| config.clone());

    common::log::init_log(&config.log_path);
    tracing::info!("api server srart:{}:{}", LOCAL_HOSE, config.port.clone());
    // tracing::info!(
    //     "workspace:{} \ndb:{} \nlog:{}",
    //     config.workspace,
    //     config.database_path,
    //     config.log_path,
    // );

    let db = match init_db(common::i18n::Locale::En, &"./todo_list.db".to_string()).await {
        Ok(r) => r,
        Err(e) => {
            panic!("{}", e)
        }
    };
    GLOBAL_DB.get_or_init(|| db);

    let router = router::router::create_router();
    let addr = SocketAddr::from_str(&format!("{}:{}", LOCAL_HOSE, config.port)).unwrap();
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
