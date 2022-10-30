use std::io::{Read, Write};
use std::{net, thread};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;

/**
Building into tcp server process
Listen on port: xxx
Response to client "Hello <client name>"
 */
fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("TCP server on port 3333");

    for stream in listener.incoming() {
        //create a new thread
        let thread_handler = thread::spawn(move || {
            let mut data = [0 as u8; 17];
            match stream {
                Ok(mut stream) => {
                    while match stream.read_exact(&mut data) {
                        Ok(size) => {
                            let txt = from_utf8(&data).unwrap();
                            println!("Received: {}, size:{}", txt, txt.len());
                            let mut reply = "Hi from Server";
                            stream.write(reply.as_bytes()).unwrap();
                            true
                        }
                        Err(e) => {
                            println!("{}", e);
                            false
                        }
                    } {}
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        });
    }
}