use chrono::Local;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::env;

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!(concat!(file!(), "-", module_path!(), "::", line!(), " {}"), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!(concat!(file!(), "-", module_path!(), "::", line!(), " {}"), format_args!($($arg)*));
    };
}

pub static LOGGER: SimpleLogger = SimpleLogger;

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[{}] {} [{}-{}::{}] {}",
                Local::now().format("%d/%b/%Y %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("<unknown>"),
                record.module_path().unwrap_or("<unknown>"),
                record.line().unwrap_or(0),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn init_logger() -> Result<(), SetLoggerError> {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());
    let level_filter = match log_level.to_lowercase().as_str() {
        "info" => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };

    log::set_logger(&LOGGER).map(|()| log::set_max_level(level_filter))
}
