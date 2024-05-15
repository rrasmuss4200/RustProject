use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}}
    ;
// For simplicity, I will be stopping the code instead of explicitly handling errors with requests
fn main() {
    //creates a socket that listens for connections at the specified address
    //.unwrap() stops the program if bind returns Err
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // processes each connection and produces streams to handle
    for stream in listener.incoming() {
        let stream: std::net::TcpStream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // manages calls to std::io::Read
    let buf_reader = BufReader::new(&mut stream);

    // collects the lines that the request from the browser generates and puts them into a vector
    let http_request: Vec<_> = buf_reader
        // returns an iterator that splits each time it reads a new line character
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // an HTTP successful request response. Communicates HTTP version 1.1,
    // a statues code of 200 (success), an OK reason phrase, no headers and no body
    let response = "HTTP/1.1 200 OK\rn\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}