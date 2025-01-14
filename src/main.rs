use std::io::{Read, Write};
use std::net;
use std::thread;

fn echo(mut stream: net::TcpStream) {
    let mut buffer: [u8; 512] = [0; 512];
    println!(
        "Incoming TCP connection from: {}",
        stream.peer_addr().unwrap()
    );
    // todo: add error handling
    while stream.read(&mut buffer).expect("read failed") > 0 {
        stream.write(&buffer).expect("write failed");
    }

    println!("Connection closed!");
}

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8080").expect("Could not bind on port 8080");
    println!("Listening on: {}", listener.local_addr().unwrap());
    let mut handlers: Vec<thread::JoinHandle<()>> = Vec::new();
    for stream in listener.incoming() {
        //  todo: add error handling
        handlers.push(std::thread::spawn(move || {
            echo(stream.unwrap());
        }));
    }
}
