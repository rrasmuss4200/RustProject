use log::{error, info};
use log4rs;
use std::fs;
use std::io::{self};


fn init_logger() {
    log4rs::init_file("log4rs.toml", Default::default()).unwrap();
}

fn generate_logs() {
    for _ in 0..2000 {
        println!("test");
        info!("This is a test log entry to fill up the log file and trigger the rolling mechanism.");
    }
    error!("This is an error log entry to ensure it gets logged correctly.");
}

fn file_size(path: &str) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self};
    use std::path::Path;

    #[test]
    fn test_log_file_rolling() {
        // Setup: Ensure the logs directory is clean
        let log_dir = Path::new("logs");
        if log_dir.exists() {
            fs::remove_dir_all(log_dir).unwrap();
        }
        fs::create_dir(log_dir).unwrap();

        init_logger();

        generate_logs();

        // Ensure the primary log file is at most 2 KB
        let output_log_path = "logs/output.log";
        assert!(file_size(output_log_path).unwrap() <= 2048);

        // Check for existence of rolled files. They go from 1-5 because
        //  that's what I chose for the limit in log4rs.toml
        for i in 1..=5 {
            let rolled_file_path = format!("logs/output.{}.log", i);
            if Path::new(&rolled_file_path).exists() {
                // Ensure the rolled files are not empty
                assert!(file_size(&rolled_file_path).unwrap() > 0);
            } else {
                break;
            }
        }

        // Cleanup: Remove the logs directory
        fs::remove_dir_all(log_dir).unwrap();
    }
}
