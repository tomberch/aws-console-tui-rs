use super::app_config::LoggingConfig;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{filter::LevelFilter, prelude::*, Layer};

pub fn init_tracing(logging_config: &LoggingConfig) -> Option<WorkerGuard> {
    if !logging_config.log_file_path.is_empty() {
        let guard = register_file_logging(&logging_config.level, &logging_config.log_file_path);
        return Some(guard);
    }

    Option::None
}

fn register_file_logging(log_level: &str, log_file_path: &str) -> WorkerGuard {
    let (file_writer, guard) = create_writer(log_file_path);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_ansi(false)
        .with_line_number(true)
        .with_target(true)
        .with_writer(file_writer)
        .with_filter(LevelFilter::from(level_from_string(log_level)));

    tracing_subscriber::registry().with(file_layer).init();

    guard
}

fn create_writer(
    log_file_path: &str,
) -> (tracing_appender::non_blocking::NonBlocking, WorkerGuard) {
    let file_appender = tracing_appender::rolling::daily(log_file_path, "log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    (file_writer, guard)
}

fn level_from_string(log_level: &str) -> Level {
    return match log_level.to_uppercase().as_str() {
        "TRACE" => Level::TRACE,
        "DEBUG" => Level::DEBUG,
        "INFO" => Level::INFO,
        "WARN" => Level::WARN,
        _ => Level::ERROR,
    };
}

#[cfg(test)]
mod tests_logging {

    use std::fs;

    use tracing::event;

    use super::*;

    #[test]
    fn test_log_file_with_warn_message() {
        let temp_dir = assert_fs::TempDir::new().unwrap();

        let config = LoggingConfig {
            level: "WARN".to_string(),
            log_file_path: temp_dir.as_os_str().to_os_string().into_string().unwrap(),
        };

        let _guard = init_tracing(&config);
        let warn_message = "My nice logging message";
        event!(Level::WARN, "{}", warn_message);
        let info_message = "Something different";
        event!(Level::INFO, "{}", info_message);

        let mut entries = fs::read_dir(temp_dir.path()).unwrap();
        let path = entries.next().unwrap().unwrap().path();
        let contents = fs::read_to_string(path).unwrap();
        assert!(contents.contains(warn_message));
        assert!(!contents.contains(info_message));

        temp_dir.close().unwrap();
    }
}
