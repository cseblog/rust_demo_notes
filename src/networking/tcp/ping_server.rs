use chrono::{DateTime, Utc};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::{net, thread};

/**
Building into tcp server process
Listen on port: xxx
Response to client "Hello <client name>"
 */
fn main() {
    let listener = TcpListener::bind("0.0.0.0:3334").unwrap();
    println!("TCP server on port 3333");

    for stream in listener.incoming() {
        //create a new thread
        let thread_handler = thread::spawn(move || {
            let mut counter = 0;
            let reply = "Hi from Server";

            match stream {
                Ok(mut stream) => {
                    loop {
                        let now: DateTime<Utc> = Utc::now();
                        let mut first_8bytes = [0u8;2];
                        stream.read_exact(&mut first_8bytes).unwrap();
                        
                        let len_str = from_utf8(&first_8bytes).unwrap().to_string();
                        let len = len_str.parse::<u32>().unwrap();
    
                        println!("Recieved msg with len: {}", len);
    
    
                        let mut data = vec![0u8; len as usize];
                        // Read the rest of the response
                        stream.read_exact(&mut data).unwrap();
                        let txt = from_utf8(&data).unwrap();
                        println!("[{}], msg: {}",now , txt);
                    }





                    // while match stream.read_exact(&mut data) {
                    //     Ok(_) => {
                    //         let txt = from_utf8(&data).unwrap();
                    //         let now: DateTime<Utc> = Utc::now();
                    //         println!("[{}] msg: {}",now, txt);

                    //         // let now: DateTime<Utc> = Utc::now();
                    //         // let new_msg = format!("[{}] {}[{}]", now, reply, counter);

                    //         // stream.write(new_msg.as_bytes()).unwrap();
                    //         // counter += 1;
                    //         true
                    //     }
                    //     Err(e) => {
                    //         println!("{}", e);
                    //         false
                    //     }
                    // } {}


                }
                Err(e) => {
                    println!("Error: {}", e);
                    panic!("Exit!");
                }
            }
        });
    }
}
