use log::{error, info, debug};
use log4rs;

fn main() {
    debug!("Starting the logger initialization.");
    if let Err(e) = log4rs::init_file("log4rs.toml", Default::default()) {
        println!("Error initializing log4rs: {}", e);
    } else {
        println!("Logger initialized successfully.");
    }

    // Log some messages
    info!("This is an info message.");
    error!("This is an error message.");
    debug!("Finished logging messages.");
}
