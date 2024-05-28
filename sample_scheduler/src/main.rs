use std::env;
use log::{error, info, debug};
use log4rs;

// enum of different states of a process
pub enum MessageState {
    New,
    Ready,
    Suspended,
    Waiting,
    Running,
    Done,
}


pub struct Message {
    state: MessageState,
    id: u32,
    len: u8,
    dest: u8,
    opcode: u8,
    data: u32,
}

fn handle_state(msg: Message) {
    match msg.state {
        MessageState::New => {
            info!("New task received. #{}", msg.id);
        }
        MessageState::Ready => {
            info!("Task #{} ready to execute.", msg.id);
        }
        MessageState::Running => {
            info!("Task #{} is running.", msg.id);
        }
        MessageState::Done => {
            info!("Task #{} is done running.", msg.id)
        }
        MessageState::Waiting => {
            info!("Task #{} is waiting to run.", msg.id);
        }
        MessageState::Suspended => {
            info!("Task #{} is suspended.", msg.id);
        }
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

    // Template for time:
    // YYYY-MM-DDTHH:MM:SS
    // the T is part of the message. You put it in
    // let time_arg: String = args[1].parse::<String>().unwrap();

    // let command_arg: String = args[2].parse::<String>().unwrap();

    // let input_tuple: (String, String) = (time_arg,command_arg);

    // let (time,command) = input_tuple;
    // println!("Time: {}, Command: {}", time, command);

    let msg = Message {
        state: MessageState::New,
        id: 0,
        len: 35,
        dest: 1,
        opcode: 4,
        data: 32,
    };

    handle_state(msg);


    // create queue of incoming tasks. Assign priority values

}
