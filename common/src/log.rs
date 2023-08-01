use std::io;

use chrono::Local;

use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::filter::{FilterFn, FilterId, self};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::fmt::{self, time::FormatTime};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Layer};

#[derive(Debug)]
struct MyTimeFormat;

impl FormatTime for MyTimeFormat {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        write!(w, "{}", now.to_string())
    }
}

/**
 * @Author         : ClzSkywalker
 * @Date           : 2023-04-28
 * @Description    : 初始化 Log
 * @param           {*} path
 * @return          {*}
 */
pub fn init_log(path: &str) {
    let err_file = rolling::daily(path, "error").with_max_level(Level::ERROR);

    let info_file = rolling::daily(path, "info")
        .with_max_level(Level::INFO)
        .with_min_level(Level::INFO);
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());

    // let console_filter =
    //     EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = tracing_subscriber::registry()
        // .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        // todo 控制台日志等级过滤
        .with(
            fmt::Layer::new()
                .with_timer(MyTimeFormat)
                .with_line_number(true)
                // .with_filter(filter)
                // .with_filter(filter::LevelFilter::DEBUG)
                .with_writer(io::stdout),
        )
        .with(
            fmt::Layer::new()
                .json()
                .with_timer(MyTimeFormat)
                .with_line_number(true)
                .with_span_list(true)
                .with_writer(info_file),
        )
        .with(
            fmt::Layer::new()
                .json()
                .with_timer(MyTimeFormat)
                .with_line_number(true)
                .with_span_list(true)
                .with_writer(err_file),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global collector");
    ()
}
