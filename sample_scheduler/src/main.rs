use std::{time::{SystemTime, Duration}, io::{self}};
use std::time;
use std::path::Path;
use std::fs;
use std::io::BufRead;
pub mod message;
use crate::message::*;
pub mod scheduler;
use crate::scheduler::*;
pub mod log;
use crate::log::*;


fn main() {
    init_logger();

    let stdin = io::stdin();
    let mut command_arg = String::new();
    stdin.read_line(&mut command_arg).expect("Failed to read command");

    // Read the second line from stdin
    let mut human_date = String::new();
    stdin.read_line(&mut human_date).expect("Failed to read date");

    // Convert input human-readable time to epoch time to compare times when program is run
    let command_time: Result<u64, String> = timestamp_to_epoch(human_date.trim().to_string());

    // while true to keep current time updating?
    loop {
    let curr_time: Result<Duration, time::SystemTimeError> = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    let curr_time_millis: u64 = match curr_time {
        Ok(duration) => duration.as_millis() as u64,
        Err(e) => {eprint!("Error {:?}", e);
    return;}
    };

    let now: Result<u64, String> = command_time.clone();

    let input_tuple: (Result<u64, String>, String) = (now.clone(),command_arg.clone());


    println!("Command Time: {:?} ms, Command: {}\nCurrent time is {:?} ms", input_tuple.0.as_ref().unwrap(), input_tuple.1, curr_time_millis);

    // dummy message
    let mut msg = Message {
        time: command_time.clone(),
        state: MessageState::New,
        id: 0,
        command: command_arg.clone(),
    };

    if msg.time <= Ok(curr_time_millis) {
        // Send to CmdDispathcer
        msg.state = MessageState::Running;
        handle_state(&msg);
        println!("Sent to CmdDispatcher");
        log_info("Sent to CmdDispatcher".to_string(), msg.id);
    } else {
        // save to non-volatile memory
        if let Err(err) = write_input_tuple_to_rolling_file(&input_tuple) {
            eprintln!("Failed to write input tuple to file: {}", err);
        } else {
            println!("Input tuple written to file successfully.");
            log_info("Command stored to file.".to_string(), msg.id)
        }
        log_info("Command stored and scheduled for later".to_string(), msg.id);

    }

    let saved_commands_dir = Path::new("saved_commands");
    if saved_commands_dir.exists() && saved_commands_dir.is_dir() {
        match fs::read_dir(saved_commands_dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            if let Some(file_name) = entry.file_name().to_str() {
                                    if let Ok(file_time) = file_name.trim_end_matches(".txt").parse::<u64>() {
                                        if file_time <= curr_time_millis {
                                            // Read the file contents
                                            let file_path = entry.path();
                                            match fs::File::open(&file_path) {
                                                Ok(file) => {
                                                    let lines: Vec<String> = io::BufReader::new(file)
                                                        .lines()
                                                        .filter_map(Result::ok)
                                                        .collect();
                                                    if lines.len() > 1 {
                                                        println!("Second line of {}: {}", file_name, lines[1]);
                                                    } else {
                                                        println!("File {} does not have a second line.", file_name);
                                                    }
                                                }
                                                Err(e) => eprintln!("Failed to open file {}: {:?}", file_name, e),
                                            }
                                        }
                                    }

                            }
                        }
                        Err(e) => eprintln!("Error reading directory entry: {:?}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory: {:?}", e),
        }
    } else {
        eprintln!("Directory saved_commands does not exist or is not a directory.");
    }

}}

    // TODO: create queue for incoming tasks. Assign priority values and sort based on time to be execcuted
    // can create function for comparing UTC epoch values that returns true if one occurs before the other
    // and an if statement uses this to determine whether the values will be swapped.

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self};

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
    fn conversion_test_invalid_format() {
        let timestamp = "2023/05/30 12:34:56".to_string();

        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => panic!("Expected Err, but got Ok({})", epoch),
            Err(e) => assert!(e.contains("Failed to parse timestamp")),
        }
    }

    #[test]
    fn conversion_test_invalid_date() {
        let timestamp: String = "2024-55-41 12:33:33".to_string();

        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => panic!("Invalid timestamp format, got {}", epoch),
            Err(e) => assert!(e.contains("Failed to parse timestamp")),
        }
    }

    #[test]
    fn conversion_test_invalid_time() {
        let timestamp: String = "2024-10-30 24:41:77".to_string();

        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => panic!("Invalid timestamp format, got {}", epoch),
            Err(e) => assert!(e.contains("Failed to parse timestamp")),
        }
    }

    #[test]
    fn conversion_test_valid_timestamp_2() {
        let timestamp: String = "2027-02-05 02:04:38".to_string();
        match timestamp_to_epoch(timestamp) {
            Ok(epoch) => {
                // Verify the epoch value is correct
                let expected_epoch: u64 = 1801793078000;
                assert_eq!(epoch, expected_epoch);
            },
            Err(e) => panic!("Expected valid date, but got error: {}", e),
        }
    }

    #[test]
    fn test_write_input_tuple_creates_file() {
        // works when saved_commands directory not created yet. Just tests that 1 file is created
        let test_dir = "saved_commands".to_string();
        let input_tuple = (Ok(1717110630000), "Test command".to_string());

        let result = write_input_tuple_to_rolling_file(&input_tuple);
        assert!(result.is_ok());

        let files: Vec<_> = fs::read_dir(&test_dir).unwrap().collect();
        assert!(files.len() != 0);

    }

    #[test]
    fn test_oldest_file_deletion() {
        // works when saved_commands directory not created yet.
        let test_dir = "saved_commands";
        fs::create_dir_all(test_dir).unwrap();

        let input_tuple = (1717428208, String::from("Test Command"));

        // Create files to exceed the max size
        for i in 0..2000 {
            let new_timestamp: u64 = input_tuple.0.clone() + i;
            write_input_tuple_to_rolling_file(&(Ok(new_timestamp), input_tuple.1.clone())).unwrap();
        }

        // Check initial number of files
        let initial_files: Vec<_> = fs::read_dir(test_dir).unwrap().collect();

        // Write an input tuple to trigger the removal of the oldest file
        let input_tuple = (Ok(2717428208), String::from("Test Command"));
        write_input_tuple_to_rolling_file(&(input_tuple.0, input_tuple.1)).unwrap();

        // Check final number of files
        let final_files: Vec<_> = fs::read_dir(test_dir).unwrap().collect();
        assert_eq!(final_files.len(), initial_files.len());

        // Ensure the oldest file was removed
        let files: Vec<_> = fs::read_dir(test_dir)
            .unwrap()
            .map(|res| res.unwrap().file_name().into_string().unwrap())
            .collect();
        assert!(!files.contains(&String::from("1717428208.txt")));

        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }

}