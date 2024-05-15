use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}, fs}
    ;
// For simplicity, I will be stopping the code instead of explicitly handling errors with requests
fn main() {
    //creates a socket that listens for connections at the specified address
    //.unwrap() stops the program if bind returns Err
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // processes each connection and produces streams to handle
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // manages calls to std::io::Read
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
        }