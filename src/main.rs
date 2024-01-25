mod logger; // enables the log_debug!() and log_info!() macros

fn main() {
    logger::init_logger().unwrap();

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
