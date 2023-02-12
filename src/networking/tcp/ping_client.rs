use chrono::{DateTime, Utc};
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
    fn connect(&self) {
        let result = TcpStream::connect(format!("{}:{}", self.host, self.port));
        match result {
            Ok(mut stream) => {
                let msg = "Hello my friend";
                let mut counter = 0;

                loop {
                    let now: DateTime<Utc> = Utc::now();
                    let new_msg = format!("[{}] {}[{}]", now, msg, counter);
                    let msg_wrappter = format!("{}{}", new_msg.len().to_string(), new_msg);
                    println!("Sending..{}, size: {}, old size: {}", msg_wrappter, msg_wrappter.len(), new_msg.len());  
                    stream.write(msg_wrappter.as_bytes()).unwrap();
                    counter += 1;

                    // let mut data = [0 as u8; 50];
                    // match stream.read(&mut data) {
                    //     Ok(_) => {
                    //         let txt = from_utf8(&data);
                    //         println!("{}", txt.unwrap());
                    //         counter += 1;
                    //     }
                    //     Err(e) => {
                    //         println!("{}", e)
                    //     }
                    // }
                    sleep(Duration::from_millis(10));
                }
            }
            Err(e) => {
                println!("{}", e);
                panic!("Failed to connect!");
            }
        }
    }
}

fn main() {
    println!("{}", "Client is trying to connect to server!");
    let tcp_client = TcpClient {
        host: "localhost".to_string(),
        port: 3333,
    };
    tcp_client.connect();
}
