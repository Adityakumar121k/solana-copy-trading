use std::sync::LazyLock;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static CONFIG_SETUP: LazyLock<(NonBlocking, NonBlocking, WorkerGuard, WorkerGuard)> =
    LazyLock::new(|| {
        let path = "logs";
        let _ = std::fs::create_dir_all(path);
        let file_appender = tracing_appender::rolling::daily(path, "log.log");
        let (file_writer, file_guard) = tracing_appender::non_blocking(file_appender);
        let (stdout_writer, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());
        (file_writer, stdout_writer, file_guard, stdout_guard)
    });

pub struct Tracing;

impl Tracing {
    pub fn init() {
        let (file_writer, stdout_writer, _, _) = LazyLock::force(&CONFIG_SETUP);

        let filter = EnvFilter::from_default_env();

        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(file_writer.clone())
            .with_ansi(false)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .json();

        let stdout_layer = tracing_subscriber::fmt::layer()
            .with_writer(stdout_writer.clone())
            .with_target(false);

        tracing_subscriber::registry()
            .with(filter)
            .with(file_layer)
            .with(stdout_layer)
            .init();
    }
}
