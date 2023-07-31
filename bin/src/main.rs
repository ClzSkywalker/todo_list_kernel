use clap::Parser;
use cmd::{api::server_api, CliCommond};
use common::config::AppConfig;

pub mod cmd;

#[tokio::main]
async fn main() {
    let cli = CliCommond::parse();

    match cli.command {
        cmd::Commands::Api(args) => {
            let mut config = AppConfig::default();
            config.mode = args.mode;
            config.port = args.port;
            config.log_path = args.log_path;
            config.db_path = args.db_path;
            server_api(config).await
        }
    }
}
