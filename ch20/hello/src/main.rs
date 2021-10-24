use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7070").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connect(stream);
    }
}

fn handle_connect(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"Get / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let responce = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        println!("responce\n {}", responce);
        stream.write(responce.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        
    }
}
