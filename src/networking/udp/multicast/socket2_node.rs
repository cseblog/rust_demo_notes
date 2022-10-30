use std::{io, thread};
use std::time::Duration;

use socket2::{Domain, Protocol, SockAddr, Socket, Type};

fn main() {
    let addrIPv4 = Ipv4Addr::new(224, 0, 0, 123);
    let addr = SocketAddr::new(addrIPv4.into(), 9876);
    let socket = Socket::new(Domain::ipv4(), Type::dgram(), Some(Protocol::udp()));

    // we're going to use read timeouts so that we don't hang waiting for packets
    socket.set_read_timeout(Some(Duration::from_millis(100)))?;
    socket.join_multicast_v4(&addrIPv4, &Ipv4Addr::new(0, 0, 0, 0))?;
    let thread_1 = thread::spawn(move || {
        let mut buf = [0u8; 64]; // receive buffer
        // we're assuming failures were timeouts, the client_done loop will stop us
        match socket.recv_from(&mut buf) {
            Ok((len, remote_addr)) => {
                let data = &buf[..len];

                println!(
                    "server: got data: {} from {}",
                    String::from_utf8_lossy(data),
                    remote_addr
                );

                // create a socket to send the response
                // let responder = new_socket(&remote_addr)
                //     .expect("failed to create responder")
                //     .into_udp_socket();
                //
                // // we send the response that was set at the method beginning
                // responder
                //     .send_to(response.as_bytes(), &remote_addr)
                //     .expect("failed to respond");
                //
                // println!("{}:server: sent response to: {}", response, remote_addr);
            }
            Err(err) => {
                println!("server: got an error: {}", err);
            }
        }
    });

}