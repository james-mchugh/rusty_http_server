use std::io::{Read, Write};
use std::net;

fn echo(mut stream: net::TcpStream) {
    let mut buffer: [u8; 512] = [0; 512];

    match stream.peer_addr() {
        Ok(addr) => println!("Incoming TCP connection from: {}", addr),
        Err(_) => println!("Cannot get peer address"),
    }

    // todo: add error handling
    stream.read(&mut buffer).expect("read failed");

    stream
       
        .write("HTTP/1.1 200 OK\r\n\r\n".as_ref())
        .expect("Failed to write to socket");
    // stream.write(&buffer).expect("Failed to write to socket");

    println!("Connection closed!");
}

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8080").expect("Could not bind on port 8080");
    println!("Listening on: {}", listener.local_addr().unwrap());
    let mut handlers = Vec::new();
    for stream in listener.incoming() {
        //  todo: add error handling
        handlers.push(std::thread::spawn(move || {
            echo(stream.unwrap());
        }));
    }
}
