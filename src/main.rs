use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use chrono::Local;

macro_rules! custom_log {
    ($($arg:tt)*) => ({
        log::info!(concat!(file!(), "-", module_path!(), "::", line!(), " {}"), format_args!($($arg)*));
    });
}

static LOGGER: SimpleLogger = SimpleLogger;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
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
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

fn main() {
    init_logger().unwrap();

    custom_log!("log-entry from the main function.");

    first_function();
    second_function();
}

fn first_function() {
    custom_log!("log-entry from the first function.");
}

fn second_function() {
    custom_log!("log-entry from the second function.");
}
