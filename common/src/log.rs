use std::io;

use chrono::Local;
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::fmt::{self, time::FormatTime};
use tracing_subscriber::layer::SubscriberExt;

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
    let err_file = rolling::daily(path, "error").with_max_level(Level::WARN);

    let info_file = rolling::daily(path, "info")
        .with_max_level(Level::DEBUG)
        .with_min_level(Level::INFO);

    let subscriber = tracing_subscriber::registry()
        // .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(
            fmt::Layer::new()
                .with_timer(MyTimeFormat)
                .with_line_number(true)
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
