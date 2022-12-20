use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;

/**
Client send "hi <client name>"
 */

struct TcpClient {
    host: String,
    port: u32,
}

impl TcpClient {
    fn subscribe(&self) {}

    fn connect(&self) {
        let result = TcpStream::connect("0.0.0.0:3333");
        match result {
            Ok(mut stream) => {
                let msg = "Hello my friend 2";
                // create 1 thread sending data
                // 1 Thread handle input data
                
                loop {
                    stream.write(msg.as_bytes()).unwrap();
                    let mut data = [0 as u8; 14];
                    match stream.read_exact(&mut data) {
                        Ok(_) => {
                            let txt = from_utf8(&data);
                            println!("{}, size: {}", txt.unwrap(), data.len());
                        }
                        Err(e) => {
                            println!("{}", e)
                        }
                    }
                    sleep(Duration::from_secs(3));
                }
                
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn main() {
    println!("{}", "Client");
}
