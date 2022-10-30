use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;

/**
Client send "hi <client name>"
 */
fn main() {
    println!("{}", "Client");
    match TcpStream::connect("0.0.0.0:3333") {
        Ok(mut stream) => {
            let msg = "Hello my friend 2";
            loop {
                stream.write(msg.as_bytes()).unwrap();
                let mut data = [0 as u8; 14];
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        let txt = from_utf8(&data);
                        println!("{}, size: {}", txt.unwrap(), data.len());
                    },
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