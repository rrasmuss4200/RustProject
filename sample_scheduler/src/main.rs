use std::{time::{SystemTime, Duration}, io::{self}};
use log::{error, debug};
use log4rs;
use std::time;
pub mod message;
use crate::message::*;
pub mod scheduler;
use crate::scheduler::*;

fn main() {
    debug!("Initializing Logger.");
    if let Err(e) = log4rs::init_file("log4rs.toml", Default::default()) {
        println!("Error initializing log4rs: {}", e);
    } else {
        println!("Logger initialized successfully.");
    }

    let stdin = io::stdin();
    let mut command_arg = String::new();
    stdin.read_line(&mut command_arg).expect("Failed to read command");

    // Read the second line from stdin
    let mut human_date = String::new();
    stdin.read_line(&mut human_date).expect("Failed to read date");

    // Convert input human-readable time to epoch time to compare times when program is run
    let command_time: Result<u64, String> = timestamp_to_epoch(human_date.trim().to_string());

    // while true to keep current time updating?

    let curr_time: Result<Duration, time::SystemTimeError> = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    let curr_time_millis: u64 = match curr_time {
        Ok(duration) => duration.as_millis() as u64,
        Err(e) => {eprint!("Error {:?}", e);
    return;}
    };

    let now: Result<u64, String> = command_time.clone();

    let input_tuple: (Result<u64, String>, String) = (now.clone(),command_arg.clone());

    if let Err(err) = write_input_tuple_to_rolling_file(&input_tuple) {
        eprintln!("Failed to write input tuple to file: {}", err);
    } else {
        println!("Input tuple written to file successfully.");
    }

    println!("Command Time: {:?} ms, Command: {}\nUnix Epoch is {:?} ms", input_tuple.0.unwrap(), input_tuple.1, curr_time_millis);

    let mut msg = Message {
        time: command_time.clone(),
        state: MessageState::New,
        id: 0,
        command: command_arg,
    };

    if msg.time >= Ok(curr_time_millis) {
        // Send to CmdDispathcer
        msg.state = MessageState::Running;
        handle_state(msg);
        println!("Sent to CmdDispatcher");
    } else {
        error!("Command is of type 'now'. Should have been dispatched.");
        eprint!("Error: Command was before Unix Epoch.\n");
    }
}

    // TODO: create queue for incoming tasks. Assign priority values and sort based on time to be execcuted
    // can create function for comparing UTC epoch values that returns true if one occurs before the other
    // and an if statement uses this to determine whether the values will be swapped.

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn conversion_test_valid_timestamp() {
            let timestamp: String = "2024-06-23 4:22:22".to_string();
            let expected_epoch: u64 = 1719116542000;

            match timestamp_to_epoch(timestamp) {
                Ok(epoch) => assert_eq!(epoch, expected_epoch),
                Err(e) => panic!("Expected Ok({}) but got ERR({})", expected_epoch, e),
            }
        }

        #[test]
        fn conversion_test_no_space() {
            let timestamp: String = "2024-11-2103:33:32".to_string();

            match timestamp_to_epoch(timestamp) {
                Ok(epoch) => panic!("Expected Err, but got Ok({})", epoch),
                Err(e) => assert_eq!(e, "Invalid timestamp format".to_string()),
            }
        }

        #[test]
        fn conversion_test_empty_timestamp() {
            let timestamp = "".to_string();

            match timestamp_to_epoch(timestamp) {
                Ok(epoch) => panic!("Expected Err, but got Ok({})", epoch),
                Err(e) => assert_eq!(e, "Invalid timestamp format".to_string()),
            }
        }

        #[test]
        fn conversion_test_invalid_format_wrong_date() {
            let timestamp = "2023/05/30 12:34:56".to_string();

            match timestamp_to_epoch(timestamp) {
                Ok(epoch) => panic!("Expected Err, but got Ok({})", epoch),
                Err(e) => assert!(e.contains("Failed to parse timestamp")),
            }
        }

    }