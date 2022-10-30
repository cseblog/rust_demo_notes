mod socket2_node;

use std::net::{Ipv4Addr, UdpSocket};
use std::str::from_utf8;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello");

    let mcast_group: Ipv4Addr = "239.255.255.1".parse().unwrap();
    let port: u16 = 4321;
    let mut buffer = [0u8; 12];
    let any = "0.0.0.0".parse().unwrap();

    let socket = UdpSocket::bind((any, port)).expect("failed to connect");
    socket.join_multicast_v4(&mcast_group, &any)
        .expect("Could not join multicast group");
    let socket2 = socket.try_clone().unwrap();


    let thread0 = thread::spawn(move || {
        loop {
            socket.recv_from(&mut buffer);
            println!("{}", from_utf8(&buffer).unwrap())
        }
    });

    let thread1 = thread::spawn(move || {
        loop {
            socket2.send_to("Hello world!".as_bytes(), &(mcast_group, port))
                .expect("Failed to write data");
            sleep(Duration::from_secs(1));
        }
    });

    thread1.join();
    thread0.join();
}