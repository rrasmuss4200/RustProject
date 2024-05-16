use std::{
    thread,
    time::Duration,
    net::{TcpListener, TcpStream},
    io::prelude::*,
    fs
};
use hello::ThreadPool;
// For simplicity, I will be stopping the code instead of explicitly handling errors with requests

fn main() {
    //creates a socket that listens for connections at the specified address
    //.unwrap() stops the program if bind returns Err
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // processes each connection and produces streams to handle
    let pool = ThreadPool::new(4);

    // .take(n) allows us to specify the number of connections we want to have present
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // manages calls to std::io::Read
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // need to match with a slice from request_line to match string literals
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
        }