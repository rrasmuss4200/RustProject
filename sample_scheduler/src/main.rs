use std::{env, time::{SystemTime, Duration}};
use log::{error, info, debug};
use log4rs;
use std::time;
use chrono::{NaiveDateTime, TimeZone, Utc};

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
    // RETURNS VALUE IN MILLISECONDS
    // Parse the input timestamp using NaiveDateTime from the chrono crate
    match NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S") {
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

fn main() {
    debug!("Initializing Logger.");
    if let Err(e) = log4rs::init_file("log4rs.toml", Default::default()) {
        println!("Error initializing log4rs: {}", e);
    } else {
        println!("Logger initialized successfully.");
    }

    let args: Vec<String> = env::args().collect();
    let command_arg: String = args[1].parse::<String>().unwrap();
    // YYYY-MM-DD HH:MM:SS
    let human_date: String = args[2].parse::<String>().unwrap();

    // Convert input human-readalbe time to epoch time to compare times when program is run
    let command_time: Result<u64, String> = timestamp_to_epoch(human_date);

    // while true to keep current time updating

    let curr_time: Result<Duration, time::SystemTimeError> = time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    let curr_time_millis: u64 = match curr_time {
        Ok(duration) => duration.as_millis() as u64,
        Err(e) => {eprint!("Error {:?}", e);
    return;}
    };

    let now: Result<u64, String> = command_time.clone();

    let input_tuple: (Result<u64, String>, String) = (now.clone(),command_arg.clone());

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
        error!("Command is of type 'now'. Should have been dispatched.")
    }

    // TODO: create queue of incoming tasks. Assign priority values and sort based on time to be execcuted
    // can create function for comparing UTC epoch values that returns true if ones before he other
    // and an if statement uses this to determine whether the values will be swapped

}
