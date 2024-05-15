use std::net::TcpListener;

fn main() {
    //creates a socket that listens for connections at the specified address
    //.unwrap() stops the program if bind returns Err
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // processes each connection and produces streams to handle
    for stream in listener.incoming() {
        let stream: std::net::TcpStream = stream.unwrap();
        println!("Connection Established!");
    }
}