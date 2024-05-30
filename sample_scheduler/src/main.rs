use std::{time::{SystemTime, Duration}, io::{self, Write}};
use std::fs;
use std::fs::File;
use std::path::Path;
use log::{error, info, debug};
use log4rs;
use std::time;
use chrono::{NaiveDateTime, TimeZone, Utc};


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
    loop {
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
        command: command_arg.clone(),
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


    // TODO: create queue of incoming tasks. Assign priority values and sort based on time to be execcuted
    // can create function for comparing UTC epoch values that returns true if one occurs before the other
    // and an if statement uses this to determine whether the values will be swapped

}
// enum of different states of a process
pub enum MessageState {
    New,
    Suspended,
    Waiting,
    Running,
    Done,
}

pub struct Message {
    time: Result<u64, String>,
    state: MessageState,
    id: u32,
    command: String,
}

fn handle_state(msg: Message) {
    match msg.state {
        MessageState::New => {
            info!("New task received at {:?}. #{}",msg.time, msg.id);
        }
        MessageState::Running => {
            info!("Task #{} is running.", msg.id);
            // Send to CmdDispatcher
        }
        MessageState::Done => {
            info!("Task #{} is done running.", msg.id)
            // Acknowledgement from CmdDispatcher
        }
        MessageState::Waiting => {
            info!("Task #{} is waiting to run.", msg.id);
            // Put into CmdDispatcher buffer
        }
        MessageState::Suspended => {
            info!("Task #{} is suspended.", msg.id);
            // Put into non-volatile memory
        }
    }
}

fn timestamp_to_epoch(timestamp: String) -> Result<u64, String> {
    // Split the input timestamp into date and time parts
    let parts: Vec<&str> = timestamp.split(' ').collect();
    if parts.len() != 2 {
        return Err("Invalid timestamp format".to_string());
    }

    let date_str = parts[0];
    let time_str = parts[1];

    // Parse the input date and time separately using NaiveDateTime from the chrono crate
    match NaiveDateTime::parse_from_str(&format!("{} {}", date_str, time_str), "%Y-%m-%d %H:%M:%S") {
        Ok(naive_date_time) => {
            // Convert the NaiveDateTime to a DateTime<Utc> object
            let date_time_utc = Utc.from_utc_datetime(&naive_date_time);
            // Get the epoch timestamp in milliseconds
            let epoch_millis = date_time_utc.timestamp_millis();
            // Convert the epoch timestamp to u64
            Ok(epoch_millis as u64)
        }
        Err(e) => Err(format!("Failed to parse timestamp: {}", e)),
    }
}

fn write_input_tuple_to_rolling_file(input_tuple: &(Result<u64, String>, String)) -> Result<(), io::Error> {
    // Create the directory if it doesn't exist
    let dir_path = "saved_commands";
    fs::create_dir_all(dir_path)?;

    // Get the total size of files in the directory
    let total_size: u64 = fs::read_dir(dir_path)?
        .filter_map(|res| res.ok())
        .map(|entry| entry.metadata().ok().map(|m| m.len()).unwrap_or(0))
        .sum();

    // Specify the maximum size in bytes
    let max_size_bytes: u64 = 2048; // 2 KB

    // If the total size exceeds the maximum size, remove the oldest file
    if total_size >= max_size_bytes {
        remove_oldest_file(&dir_path)?;
    }

    // Create a new file
    let file_name = format!("{}.txt", Utc::now().timestamp_millis());
    let file_path = Path::new(dir_path).join(&file_name);
    let mut file = File::create(&file_path)?;

    // Write input_tuple to the file
    writeln!(file, "Command Time: {:?} ms, Command: {}", input_tuple.0.as_ref().unwrap(), input_tuple.1)?;

    Ok(())
}

fn remove_oldest_file(dir_path: &str) -> Result<(), io::Error> {
    let oldest_file = fs::read_dir(dir_path)?
        .filter_map(|res| res.ok())
        .min_by_key(|entry| entry.metadata().unwrap().modified().unwrap());

    if let Some(oldest_file) = oldest_file {
        fs::remove_file(oldest_file.path())?;
    }

    Ok(())
}