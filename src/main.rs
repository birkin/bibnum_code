use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use chrono::Local;

macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!(concat!(file!(), "-", module_path!(), "::", line!(), " {}"), format_args!($($arg)*));
    };
}

macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!(concat!(file!(), "-", module_path!(), "::", line!(), " {}"), format_args!($($arg)*));
    };
}

// Additional macros for other levels (e.g., log_warn!, log_error!) can be defined similarly

static LOGGER: SimpleLogger = SimpleLogger;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug // Adjust to the highest level you want to support
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

fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug)) // Adjust as needed
}

fn main() {
    init_logger().unwrap();

    log_info!("log-entry from the main function.");

    first_function();
    second_function();
}

fn first_function() {
    log_debug!("log-entry from the first function.");
}

fn second_function() {
    log_info!("log-entry from the second function.");
}
